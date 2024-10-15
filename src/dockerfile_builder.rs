use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;

pub struct DockerfileBuilder {
    service_name: String,
}

impl DockerfileBuilder {
    pub fn new(
        service_name: String,
    ) -> Self {
        Self {
            service_name,
        }
    }

    pub fn build(mut self) -> String {
        let mut content = String::new();
        content.push_str("FROM rust:latest AS builder\n");
        content.push_str("WORKDIR /usr/src/service\n");
        content.push_str(&format!("COPY ./services/{}/Cargo.toml ./services/{}/Cargo.lock ./\n", &self.service_name, &self.service_name));
        content.push_str("RUN cargo build --release || true\n");
        content.push_str("COPY ./nutt.conf.toml ./\n");
        content.push_str(&format!("COPY ./services/{} .\n", &self.service_name));
        content.push_str("RUN cargo build --release\n");
        content.push_str("FROM debian:bookworm\n");
        content.push_str("COPY --from=builder /usr/src/service/target/release/service /usr/local/bin/service\n");
        content.push_str("CMD [\"service\"]");
        content
    }
}