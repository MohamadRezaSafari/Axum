mod routes;

use routes::create_routes;


pub async fn run() {
    let app = create_routes();

    // let app: Router = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// async fn hello_world() -> String {
//     "Hello Mohamad Reza".to_owned()
// }
