use tokio::fs;

use tracing::info;

mod build;
mod model;
mod route;
mod server;
mod service;
mod r#static;
mod template;

use crate::server::Server;

// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
