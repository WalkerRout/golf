use axum::Router;

use tokio::net::TcpListener;

use tracing::info;

use crate::route;

pub struct Server {
  app: Router,
  listener: TcpListener,
}

impl Server {
  pub async fn new() -> Self {
    let address = "0.0.0.0";
    let port = 3000;

    let app = route::rest_router();
    let listener = TcpListener::bind((address, port)).await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());

    Self { app, listener }
  }

  pub async fn run(self) {
    info!("running server...");
    axum::serve(self.listener, self.app).await.unwrap();
  }
}
