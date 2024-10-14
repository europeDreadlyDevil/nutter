pub struct Service;

impl Service {
    pub async fn get_hello() -> String {
        "Hello from Service".into()
    }
}