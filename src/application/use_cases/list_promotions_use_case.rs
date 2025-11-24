use crate::domain::entities::landing::LandingPromotion;
use crate::domain::repositories::promotion_repository::PromotionRepository;
use crate::shared::error_types::ApiError;

pub struct ListPromotionsUseCase<R: PromotionRepository> {
    promotion_repository: R,
}

impl<R: PromotionRepository + Clone> Clone for ListPromotionsUseCase<R> {
    fn clone(&self) -> Self {
        Self {
            promotion_repository: self.promotion_repository.clone(),
        }
    }
}

impl<R: PromotionRepository> ListPromotionsUseCase<R> {
    pub fn new(promotion_repository: R) -> Self {
        Self {
            promotion_repository,
        }
    }

    pub async fn execute(&self, only_active: bool) -> Result<Vec<LandingPromotion>, ApiError> {
        let promotions = if only_active {
            self.promotion_repository.list_active_promotions().await?
        } else {
            // Por ahora solo tenemos list_active_promotions, pero podemos extenderlo
            self.promotion_repository.list_active_promotions().await?
        };

        // LandingPromotion no tiene fechas, así que solo retornamos las promociones activas
        // En el futuro podemos extender esto para validar fechas
        Ok(promotions)
    }
}
