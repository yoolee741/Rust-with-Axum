use sea_orm::Database;

#[tokio::main] // for async
pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await;
    
}