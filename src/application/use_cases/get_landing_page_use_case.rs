use crate::domain::entities::landing::LandingPageData;
use crate::domain::repositories::landing_repository::{LandingPageLimits, LandingRepository};
use crate::shared::error_types::ApiError;

#[derive(Clone)]
pub struct GetLandingPageUseCase<R: LandingRepository> {
    landing_repository: R,
    limits: LandingPageLimits,
}

impl<R: LandingRepository> GetLandingPageUseCase<R> {
    pub fn new(landing_repository: R, limits: Option<LandingPageLimits>) -> Self {
        Self {
            landing_repository,
            limits: limits.unwrap_or_default(),
        }
    }

    pub async fn execute(&self) -> Result<LandingPageData, ApiError> {
        self.landing_repository
            .fetch_landing_page_data(self.limits)
            .await
    }
}
