use actix_cors::Cors;
use actix_web::{
    web::Data,
    {App, HttpServer},
};
use dotenv::dotenv;
use serde::Serialize;

mod core;
mod routes;
mod services;

use routes::get_user_data::api_get_user_data;
use routes::hello::hello;

use services::database::{convert_sqlx_error, Database};
use core::init_db::init_db;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub struct AppState {
    pub database: Database,
    pub jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(result) => result,
        Err(_) => "".to_string(),
    };

    let database_url = match std::env::var("DATABASE_URL") {
        Ok(result) => result,
        Err(_) => "".to_string(),
    };

    let use_memory_db = database_url == *"";

    let database = convert_sqlx_error(Database::new_db(use_memory_db, database_url).await)?;
    
    match init_db(&database).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let shared_state = Data::new(AppState {
        jwt_secret,
        database,
    });

    return HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(hello)
            .service(api_get_user_data)
    })
    .bind(("0.0.0.0", 3080))?
    .run()
    .await;
}
