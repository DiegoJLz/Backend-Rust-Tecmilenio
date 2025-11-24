use actix_web::{web, HttpResponse, Scope};

use crate::application::dto::promotion_dto::PromotionQueryParams;
use crate::interfaces::controllers::promotion_controller::PromotionController;

pub fn promotion_routes() -> Scope {
    web::scope("/promotions")
        .route("", web::get().to(list_promotions_handler))
        .route("/{id}", web::get().to(get_promotion_handler))
}

async fn list_promotions_handler(
    req: actix_web::HttpRequest,
    query: web::Query<PromotionQueryParams>,
) -> HttpResponse {
    let controller = req
        .app_data::<web::Data<PromotionController>>()
        .expect("PromotionController not found in app_data");

    controller.list_promotions(query).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn get_promotion_handler(
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let controller = req
        .app_data::<web::Data<PromotionController>>()
        .expect("PromotionController not found in app_data");

    controller.get_promotion(path).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}
