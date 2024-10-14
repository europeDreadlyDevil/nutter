use nutt_web::{routes, NuttServer};
use crate::app::controller::Controller;
use crate::{LOCAL_ADDR, DOCKER_ADDR};

mod controller;
mod service;

pub struct MainApp {
    server: NuttServer,
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            server: NuttServer::new()
                .bind_dev(LOCAL_ADDR)
                .bind_release(DOCKER_ADDR)
                .routes(routes![
                    Controller::hello,
                ])
        }
    }

    pub async fn run(self) {
        self.server.run().await
    }
}