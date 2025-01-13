mod todo;
use actix_web::{main, middleware::Logger, App, HttpServer};
use std::{env, error::Error, net::Ipv4Addr};
use todo::{addTodo, getTodo};
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt};
use utoipa_scalar::{Scalar, Servable};

#[main]
async fn main() -> Result<(), impl Error> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(info(title = "SimpleTodo API", description = "Document of SimpleTodo API"))]
    struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .service(scope("/todo").service(getTodo).service(addTodo))
            .openapi_service(|api| Scalar::with_url("/docs", api))
            .into_app()
    })
    .bind((Ipv4Addr::UNSPECIFIED, 3000))?
    .run()
    .await
}
