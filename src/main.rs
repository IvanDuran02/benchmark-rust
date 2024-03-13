use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world" }));

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
