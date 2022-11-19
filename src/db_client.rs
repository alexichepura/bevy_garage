use crate::db::PrismaClient;
use bevy::prelude::Resource;
use prisma_client_rust::NewClientError;

#[derive(Resource)]
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
