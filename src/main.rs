pub mod content_directory;
pub mod rbx_helper;
pub mod content_server;

use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc, widget::{Label, TextBox}};
use crate::content_server::ContentServer;
use log::{info, warn, error, debug};
use env_logger::{self, Env};
const WINDOW_TITLE: LocalizedString<()> = LocalizedString::new("Guava - Roblox Content Manager");

fn test() -> impl Widget<()> {
    Label::new("Hello Coda & Kisty!")
}

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    // init env_logger
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Starting Guava Content Manager");

    let content_server = ContentServer::new("http://localhost:8080".to_string());

    debug!("RBX VERSION: {:?}", rbx_helper::get_roblox_client_version().await.ok());
    debug!("RBX VERSION: {:?}", rbx_helper::get_roblox_studio_version().await.ok());

    let primary_window = WindowDesc::new(test())
        .title(WINDOW_TITLE)
        .window_size((450.0, 450.0));

    AppLauncher::with_window(primary_window).launch(())?;
    Ok(())
}