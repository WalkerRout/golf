use tokio::fs;

use tracing::info;

mod route;
mod server;
mod template;

use crate::server::Server;

async fn show_visible_files() {
  if let Ok(mut files) = fs::read_dir("./").await {
    info!("visible files:");
    while let Ok(Some(file)) = files.next_entry().await {
      info!("- {}", file.path().display());
    }
  }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  tracing_subscriber::fmt()
    .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
    .with_target(false)
    .with_thread_ids(true)
    .with_ansi(false)
    .init();

  log_panics::init();

  show_visible_files().await;

  info!("initializing golf_server...");

  let server = Server::new().await;
  let _ = server.run().await;

  info!("exiting golf_server...");
}
