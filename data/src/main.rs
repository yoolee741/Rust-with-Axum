use data::run;
use dotenvy_macro::dotenv;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri);
}
