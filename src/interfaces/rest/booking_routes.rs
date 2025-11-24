use actix_web::{web, HttpResponse, Scope};

use crate::application::dto::booking_dto::{CreateBookingDto, QuoteBookingRequest};
use crate::interfaces::controllers::booking_controller::BookingController;

pub fn booking_routes() -> Scope {
    web::scope("/bookings")
        .route("/quote", web::post().to(quote_booking_handler))
        .route("", web::post().to(create_booking_handler))
        .route("/{id}", web::get().to(get_booking_handler))
}

pub fn user_bookings_routes() -> Scope {
    web::scope("/users/{user_id}/bookings")
        .route("", web::get().to(list_user_bookings_handler))
}

async fn quote_booking_handler(
    req: actix_web::HttpRequest,
    body: web::Json<QuoteBookingRequest>,
) -> HttpResponse {
    let controller = req
        .app_data::<web::Data<BookingController>>()
        .expect("BookingController not found in app_data");

    controller.quote_booking(body).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn create_booking_handler(
    req: actix_web::HttpRequest,
    body: web::Json<CreateBookingDto>,
) -> HttpResponse {
    let controller = req
        .app_data::<web::Data<BookingController>>()
        .expect("BookingController not found in app_data");

    controller.create_booking(body).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn get_booking_handler(
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let controller = req
        .app_data::<web::Data<BookingController>>()
        .expect("BookingController not found in app_data");

    controller.get_booking(path).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn list_user_bookings_handler(
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let controller = req
        .app_data::<web::Data<BookingController>>()
        .expect("BookingController not found in app_data");

    controller.list_user_bookings(path, query).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}
