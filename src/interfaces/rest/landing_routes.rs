use actix_web::{web, HttpRequest, HttpResponse, Scope};

use crate::interfaces::controllers::landing_controller::LandingController;

pub fn landing_routes() -> Scope {
    web::scope("/landing")
        .route("", web::get().to(get_landing_handler))
        .route("/", web::get().to(get_landing_handler))
}

async fn get_landing_handler(req: HttpRequest) -> HttpResponse {
    let landing_controller = req
        .app_data::<web::Data<LandingController>>()
        .expect("LandingController not found in app data");

    landing_controller
        .get_landing_page()
        .await
        .unwrap_or_else(|e| {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string()
            }))
        })
}
