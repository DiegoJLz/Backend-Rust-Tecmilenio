use actix_web::{web, HttpResponse, Result};

use crate::application::dto::booking_dto::{CreateBookingDto, QuoteBookingRequest};
use crate::application::handlers::booking_handler::BookingHandler;
use crate::shared::response_types::ApiResponse;

#[derive(Clone)]
pub struct BookingController {
    handler: BookingHandler,
}

impl BookingController {
    pub fn new(handler: BookingHandler) -> Self {
        Self { handler }
    }

    pub async fn quote_booking(
        &self,
        request: web::Json<QuoteBookingRequest>,
    ) -> Result<HttpResponse> {
        match self.handler.quote_booking(request.into_inner()).await {
            Ok(quote) => Ok(HttpResponse::Ok().json(quote)),
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn create_booking(
        &self,
        request: web::Json<CreateBookingDto>,
    ) -> Result<HttpResponse> {
        match self.handler.create_booking(request.into_inner()).await {
            Ok(booking) => Ok(HttpResponse::Created().json(booking)),
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn get_booking(&self, booking_id: web::Path<String>) -> Result<HttpResponse> {
        match self.handler.get_booking(booking_id.into_inner()).await {
            Ok(booking) => Ok(HttpResponse::Ok().json(booking)),
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::NotFound().json(api_response))
            }
        }
    }

    pub async fn list_user_bookings(
        &self,
        path: web::Path<String>,
        query: web::Query<std::collections::HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        let user_id = path.into_inner();
        let status = query.get("status").cloned();
        match self.handler.list_user_bookings(user_id, status).await {
            Ok(bookings) => Ok(HttpResponse::Ok().json(bookings)),
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }
}
