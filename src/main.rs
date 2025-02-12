use tracing::info;

mod route;
mod server;
mod template;

use crate::server::Server;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  tracing_subscriber::fmt()
    .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
    .with_target(false)
    .with_thread_ids(true)
    .with_ansi(false)
    .init();

  log_panics::init();

  info!("initializing golf_server...");

  info!(
    "visible files for binary: {:?}",
    std::fs::read_dir("./")
      .unwrap()
      .into_iter()
      .collect::<Vec<_>>()
  );

  let server = Server::new().await;
  let _ = server.run().await;

  info!("exiting golf_server...");
}
