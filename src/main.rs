mod prisma;
mod routes;
mod error;

use axum::{Router, Server};
use prisma::PrismaClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct  AppState {
    client: Arc<PrismaClient>
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", routes::create_route().await);
        
    println!("Server listening on localhost:5000");
    Server::bind(&"0.0.0.0:5000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    
}