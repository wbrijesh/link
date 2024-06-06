use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod internal;
mod schema;
mod utils;

use internal::db::initialize_db_pool;
use internal::types::DbPool;
use utils::env_var::load_env;
use utils::health_check::health_check_handler;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_port = load_env("APP_PORT").unwrap().parse::<u16>().unwrap();

    let pool = initialize_db_pool();

    println!("Starting server at http://127.0.0.1:{}", app_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check_handler))
            .route("/hello", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", app_port))?
    .run()
    .await
}
