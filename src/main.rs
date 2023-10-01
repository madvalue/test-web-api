use actix_web::{web, App, HttpServer};

mod database;
mod dto;
mod enums;
mod routes;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1/blog")
                    .configure(routes::posts::route_config)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}