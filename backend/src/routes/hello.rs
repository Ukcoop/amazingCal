use actix_web::get;
use actix_web::{HttpResponse, Responder};

#[get("/api/")]
pub async fn hello() -> impl Responder {
    return HttpResponse::Ok()
        .body("This is the backend.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;

        let request = test::TestRequest::get().uri("/api/").to_request();
        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());
    }
}
