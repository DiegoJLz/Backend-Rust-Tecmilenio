use actix_web::{HttpResponse, Result};

use crate::application::handlers::landing_handler::LandingHandler;
use crate::shared::response_types::ApiResponse;

#[derive(Clone)]
pub struct LandingController {
    landing_handler: LandingHandler,
}

impl LandingController {
    pub fn new(landing_handler: LandingHandler) -> Self {
        Self { landing_handler }
    }

    pub async fn get_landing_page(&self) -> Result<HttpResponse> {
        match self.landing_handler.get_landing_data().await {
            Ok(response) => {
                let api_response =
                    ApiResponse::success_with_message(response, "Landing data retrieved");
                Ok(HttpResponse::Ok().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::InternalServerError().json(api_response))
            }
        }
    }
}
