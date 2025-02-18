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

use routes::hello::hello;

use routes::create::{ calendar::api_create_calendar, event::api_create_event };
use routes::get::user_data::api_get_user_data;
use routes::update::{ calendar::api_update_calendar, event::api_update_event };
use routes::delete::{ calendar::api_delete_calendar, event::api_delete_event };

use core::init_db::init_db;
use services::database::{convert_sqlx_error, Database};

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
        Ok(_) => {}
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
            .service(api_create_calendar)
            .service(api_create_event)
            .service(api_get_user_data)
            .service(api_update_calendar)
            .service(api_update_event)
            .service(api_delete_calendar)
            .service(api_delete_event)

    })
    .bind(("0.0.0.0", 3080))?
    .run()
    .await;
}
