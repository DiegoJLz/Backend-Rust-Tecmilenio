use async_trait::async_trait;

use crate::domain::entities::landing::{
    LandingCategory, LandingCollection, LandingExperienceSummary, LandingHighlight,
    LandingPageData, LandingPromotion, LandingTestimonial,
};
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait LandingRepository: Send + Sync {
    async fn fetch_highlights(&self, limit: i64) -> Result<Vec<LandingHighlight>, ApiError>;
    async fn fetch_featured_experiences(
        &self,
        limit: i64,
    ) -> Result<Vec<LandingExperienceSummary>, ApiError>;
    async fn fetch_hero_categories(&self, limit: i64) -> Result<Vec<LandingCategory>, ApiError>;
    async fn fetch_promotions(&self, limit: i64) -> Result<Vec<LandingPromotion>, ApiError>;
    async fn fetch_curated_collections(
        &self,
        limit: i64,
    ) -> Result<Vec<LandingCollection>, ApiError>;
    async fn fetch_testimonials(&self, limit: i64) -> Result<Vec<LandingTestimonial>, ApiError>;

    async fn fetch_landing_page_data(
        &self,
        limits: LandingPageLimits,
    ) -> Result<LandingPageData, ApiError> {
        let highlights = self.fetch_highlights(limits.highlights_limit).await?;
        let featured_experiences = self
            .fetch_featured_experiences(limits.featured_experiences_limit)
            .await?;
        let hero_categories = self
            .fetch_hero_categories(limits.hero_categories_limit)
            .await?;
        let promotions = self.fetch_promotions(limits.promotions_limit).await?;
        let curated_collections = self
            .fetch_curated_collections(limits.curated_collections_limit)
            .await?;
        let testimonials = self.fetch_testimonials(limits.testimonials_limit).await?;

        Ok(LandingPageData {
            highlights,
            featured_experiences,
            hero_categories,
            curated_collections,
            promotions,
            testimonials,
        })
    }
}

#[derive(Copy, Clone)]
pub struct LandingPageLimits {
    pub highlights_limit: i64,
    pub featured_experiences_limit: i64,
    pub hero_categories_limit: i64,
    pub promotions_limit: i64,
    pub curated_collections_limit: i64,
    pub testimonials_limit: i64,
}

impl Default for LandingPageLimits {
    fn default() -> Self {
        Self {
            highlights_limit: 5,
            featured_experiences_limit: 8,
            hero_categories_limit: 6,
            promotions_limit: 4,
            curated_collections_limit: 5,
            testimonials_limit: 6,
        }
    }
}
