use crate::application::dto::promotion_dto::{
    PromotionListResponse, PromotionResponse, PromotionQueryParams,
};
use crate::application::use_cases::{
    get_promotion_use_case::GetPromotionUseCase,
    list_promotions_use_case::ListPromotionsUseCase,
};
use crate::infrastructure::repositories::postgres_promotion_repository::PostgresPromotionRepository;
use crate::shared::error_types::ApiError;
use uuid::Uuid;

#[derive(Clone)]
pub struct PromotionHandler {
    list_promotions_use_case: ListPromotionsUseCase<PostgresPromotionRepository>,
    get_promotion_use_case: GetPromotionUseCase<PostgresPromotionRepository>,
}

impl PromotionHandler {
    pub fn new(promotion_repository: PostgresPromotionRepository) -> Self {
        let list_promotions_use_case = ListPromotionsUseCase::new(promotion_repository.clone());
        let get_promotion_use_case = GetPromotionUseCase::new(promotion_repository);

        Self {
            list_promotions_use_case,
            get_promotion_use_case,
        }
    }

    pub async fn list_promotions(
        &self,
        params: PromotionQueryParams,
    ) -> Result<PromotionListResponse, ApiError> {
        let promotions = self
            .list_promotions_use_case
            .execute(params.only_active)
            .await?;
        let promotion_responses: Vec<PromotionResponse> =
            promotions.into_iter().map(PromotionResponse::from).collect();
        let total = promotion_responses.len();
        Ok(PromotionListResponse {
            promotions: promotion_responses,
            total,
        })
    }

    pub async fn get_promotion(&self, promotion_id: String) -> Result<PromotionResponse, ApiError> {
        let id = Uuid::parse_str(&promotion_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid promotion_id format"))?;
        let promotion = self.get_promotion_use_case.execute(id).await?;
        Ok(PromotionResponse::from(promotion))
    }
}
