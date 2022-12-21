use crate::db::*;
pub mod db;

// #[tokio::main]
// pub async fn get_db_client() -> PrismaClient {
//     let client = PrismaClient::_builder().build().await.unwrap();
//     #[cfg(debug)]
//     client._db_push(false).await.unwrap();
//     client
// }
