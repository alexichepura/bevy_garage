use crate::db::{self, PrismaClient};
use prisma_client_rust::NewClientError;

pub struct DbClientResource {
    pub client: PrismaClient,
}
impl Default for DbClientResource {
    #[tokio::main]
    async fn default() -> Self {
        let client: Result<PrismaClient, NewClientError> = db::new_client().await;
        let client = client.unwrap();
        return DbClientResource { client };
    }
}
