use uuid::Uuid;

use crate::domain::entities::landing::LandingPromotion;
use crate::domain::repositories::promotion_repository::PromotionRepository;
use crate::shared::error_types::ApiError;

pub struct GetPromotionUseCase<R: PromotionRepository> {
    promotion_repository: R,
}

impl<R: PromotionRepository + Clone> Clone for GetPromotionUseCase<R> {
    fn clone(&self) -> Self {
        Self {
            promotion_repository: self.promotion_repository.clone(),
        }
    }
}

impl<R: PromotionRepository> GetPromotionUseCase<R> {
    pub fn new(promotion_repository: R) -> Self {
        Self {
            promotion_repository,
        }
    }

    pub async fn execute(&self, promotion_id: Uuid) -> Result<LandingPromotion, ApiError> {
        self.promotion_repository
            .get_promotion_by_id(promotion_id)
            .await?
            .ok_or_else(|| ApiError::new("PROMOTION_NOT_FOUND", "Promotion not found"))
    }
}
