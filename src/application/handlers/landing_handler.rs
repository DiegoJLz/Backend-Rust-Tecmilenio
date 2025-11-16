use crate::application::dto::landing_dto::LandingPageResponse;
use crate::application::use_cases::get_landing_page_use_case::GetLandingPageUseCase;
use crate::domain::repositories::landing_repository::LandingPageLimits;
use crate::infrastructure::repositories::postgres_landing_repository::PostgresLandingRepository;
use crate::shared::error_types::ApiError;

#[derive(Clone)]
pub struct LandingHandler {
    get_landing_page_use_case: GetLandingPageUseCase<PostgresLandingRepository>,
}

impl LandingHandler {
    pub fn new(
        landing_repository: PostgresLandingRepository,
        limits: Option<LandingPageLimits>,
    ) -> Self {
        let get_landing_page_use_case = GetLandingPageUseCase::new(landing_repository, limits);

        Self {
            get_landing_page_use_case,
        }
    }

    pub async fn get_landing_data(&self) -> Result<LandingPageResponse, ApiError> {
        let data = self.get_landing_page_use_case.execute().await?;
        Ok(LandingPageResponse::from(data))
    }
}
