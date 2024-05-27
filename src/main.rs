mod routes;

use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use Axum::run;

#[tokio::main]

 async fn main() {
    dotenv().ok();
    let database = dotenv!("DATABASE_URL");
    run(database).await
}

