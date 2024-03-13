use axum::{response::IntoResponse, routing::get, Router};
use axum_test::TestServer;

#[tokio::main]
async fn main() {
    let app = create_app();

    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            // Serve the app on the provided listener
            if let Err(err) = axum::serve(listener, app).await {
                eprintln!("Server error: {}", err)
            }
        }
        Err(err) => eprintln!("Failed to bind to address: {}", err),
    }
}

async fn hello() -> impl IntoResponse {
    "Hello, world"
}

async fn foo() -> impl IntoResponse {
    "bar"
}

#[tokio::test]
async fn test_foo_endpoint() {
    if let Ok(app) = create_app().try_into() {
        let client = TestServer::new::<Router>(app).unwrap();

        let response = client.get("/foo").await;
        assert_eq!(response.status_code(), 200);
        assert_eq!(response.text(), "bar");
    } else {
        panic!("Failed to create Axum application for testing")
    }
}

fn create_app() -> Router {
    Router::new().route("/", get(hello)).route("/foo", get(foo))
}
