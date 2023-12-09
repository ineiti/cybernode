use actix_web::{get, App, HttpResponse, HttpServer};
use backend::api::stats::StatsReply;

#[get("/v1/stats")]
async fn greet() -> HttpResponse {
    HttpResponse::Ok().json(StatsReply { ids: vec![] })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
