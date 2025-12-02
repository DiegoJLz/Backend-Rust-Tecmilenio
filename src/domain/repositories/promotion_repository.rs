use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::landing::LandingPromotion;
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait PromotionRepository: Send + Sync {
    async fn list_active_promotions(&self) -> Result<Vec<LandingPromotion>, ApiError>;

    async fn get_promotion_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LandingPromotion>, ApiError>;
}

