use axum::{
    Router,
    routing::{get, post},
};
// Import the vehicle module
mod vehicle;
use vehicle::{vehicle_get, vehicle_post};

#[tokio::main]
async fn main() {
    // 1. Create axum Router
    let vehicle_router = Router::new()
        .route("/vehicle", get(vehicle_get))
        .route("/vehicle", post(vehicle_post));

    // 2. Define the IP and Port listener (TCP)
    let tcp_address = "0.0.0.0:4734";
    let listener = tokio::net::TcpListener::bind(tcp_address).await.unwrap();

    // 3. Serve the axum Router on the listener 'axum serve'
    axum::serve(listener, vehicle_router).await.unwrap();
}
