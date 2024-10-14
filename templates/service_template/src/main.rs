use nutt_web::modules::include_addr;
use crate::app::App;

mod app;

include_addr!();

#[tokio::main]
async fn main() {
    let app = App::new();
    app.run().await;
}