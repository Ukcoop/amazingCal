use actix_cors::Cors;
use actix_web::{
//    web::Data,
    {App, HttpServer},
};
//use serde::Serialize;

//mod core;
mod routes;
//mod services;

use routes::hello::hello;
//use services::database::{convert_sqlx_error, init_db, DbWrapper};
/*
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
*/
/*
pub struct AppState {
    pub db: DbWrapper,
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
/*
    let db = convert_sqlx_error(init_db(false).await)?;
    let shared_state = Data::new(AppState { db });
*/
    return HttpServer::new(move || {
        App::new()
            /*.app_data(shared_state.clone())*/
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(hello)
    })
    .bind(("0.0.0.0", 3080))?
    .run()
    .await;
}

