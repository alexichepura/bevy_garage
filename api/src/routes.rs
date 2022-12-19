use crate::db::{self, rb};
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Router,
};
use itertools::Itertools;
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};
use serde::Deserialize;

type Database = Extension<std::sync::Arc<db::PrismaClient>>;
type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

#[derive(Deserialize)]
pub struct ReplayBufferRecord {
    state: Vec<String>,
    action: i32,
    reward: f64,
    next_state: Vec<String>,
    done: bool,
}

pub fn create_route() -> Router {
    Router::new().route("/replay", post(handle_replay_post))
}

async fn handle_replay_post(
    db: Database,
    Json(input): Json<Vec<ReplayBufferRecord>>,
) -> AppJsonResult<String> {
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
