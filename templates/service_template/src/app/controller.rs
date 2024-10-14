use nutt_web::modules::router::route::Route;
use nutt_web::http::response::responder::Responder;
use nutt_web::http::response::Response;
use nutt_web::modules::get;
use crate::app::service::Service;

pub(crate) struct Controller;

impl Controller {
    #[get("/")]
    pub async fn hello() -> Response {
        Service::get_hello().await.into_response()
    }
}