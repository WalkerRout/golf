use std::io;

use axum::Router;

use tokio::net::TcpListener;

use tracing::info;

use crate::route;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("failed to bind to address - {0}")]
  BindError(io::Error),

  #[error("server error - {0}")]
  ServeError(io::Error),
}

pub struct Server {
  app: Router,
  listener: TcpListener,
}

impl Server {
  pub async fn new() -> Result<Self, Error> {
    let address = "0.0.0.0";
    let port = 3000;

    let app = route::router();
    let listener = TcpListener::bind((address, port))
      .await
      .map_err(Error::BindError)?;

    info!(
      "listening on {}",
      listener.local_addr().map_err(Error::BindError)?
    );

    Ok(Self { app, listener })
  }

  pub async fn run(self) -> Result<(), Error> {
    info!("running server...");
    axum::serve(self.listener, self.app)
      .await
      .map_err(Error::ServeError)
  }
}
