use actix_web::{web, HttpResponse, Result};

use crate::application::dto::promotion_dto::PromotionQueryParams;
use crate::application::handlers::promotion_handler::PromotionHandler;
use crate::shared::response_types::ApiResponse;

#[derive(Clone)]
pub struct PromotionController {
    handler: PromotionHandler,
}

impl PromotionController {
    pub fn new(handler: PromotionHandler) -> Self {
        Self { handler }
    }

    pub async fn list_promotions(
        &self,
        query: web::Query<PromotionQueryParams>,
    ) -> Result<HttpResponse> {
        match self.handler.list_promotions(query.into_inner()).await {
            Ok(promotions) => Ok(HttpResponse::Ok().json(promotions)),
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn get_promotion(&self, promotion_id: web::Path<String>) -> Result<HttpResponse> {
        match self.handler.get_promotion(promotion_id.into_inner()).await {
            Ok(promotion) => Ok(HttpResponse::Ok().json(promotion)),
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::NotFound().json(api_response))
            }
        }
    }
}
