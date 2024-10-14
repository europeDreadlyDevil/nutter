use nutt_web::modules::include_addr;
use crate::app::MainApp;

mod app;

include_addr!();

#[tokio::main]
async fn main() {
    let app = MainApp::new();
    app.run().await;
}