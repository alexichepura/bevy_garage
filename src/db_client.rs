use crate::db::PrismaClient;
use prisma_client_rust::NewClientError;

pub struct DbClientResource {
    pub client: PrismaClient,
}
impl Default for DbClientResource {
    #[tokio::main]
    async fn default() -> Self {
        let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
        let client = client.unwrap();
        return DbClientResource { client };
    }
}
