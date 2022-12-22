use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::*,
    Extension, Router,
};
use db_client::db::{self, rb};
use itertools::Itertools;
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};
use serde::Deserialize;

pub type DbExt = Extension<std::sync::Arc<db::PrismaClient>>;
type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

#[derive(Deserialize)]
pub struct ReplayBufferRecord {
    state: Vec<f32>,
    action: i32,
    reward: f64,
    next_state: Vec<f32>,
    done: bool,
}

pub fn create_route() -> Router {
    Router::new().route("/replay", post(add_replay_buffer))
}

async fn add_replay_buffer(
    db: DbExt,
    Json(input): Json<Vec<ReplayBufferRecord>>,
) -> AppJsonResult<String> {
    println!("rb batch received {:?}", input.len());
    db.rb()
        .create_many(
            input
                .iter()
                .map(|t| {
                    return rb::create(
                        t.state.iter().map(|x| x.to_string()).join(","),
                        t.action,
                        t.reward,
                        t.next_state.iter().map(|x| x.to_string()).join(","),
                        t.done,
                        vec![],
                    );
                })
                .collect(),
        )
        .exec()
        .await?;

    Ok(Json::from("OK".to_string()))
}

enum AppError {
    PrismaError(QueryError),
    NotFound,
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// This centralizes all differents errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}
