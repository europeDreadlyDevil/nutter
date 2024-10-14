use nutt_web::{routes, NuttServer};
use crate::app::controller::Controller;
use crate::LOCAL_ADDR;

mod controller;
mod service;

pub struct App {
    server: NuttServer,
}

impl App {
    pub fn new() -> Self {
        Self {
            server: NuttServer::new()
                .bind_dev(LOCAL_ADDR)
                .routes(routes![
                    Controller::hello,
                ])
        }
    }

    pub async fn run(self) {
        self.server.run().await
    }
}