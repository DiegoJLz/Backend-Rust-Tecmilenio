use async_trait::async_trait;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use diesel::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::entities::landing::{
    LandingCategory, LandingCollection, LandingExperienceSummary, LandingHighlight,
    LandingPromotion, LandingTestimonial,
};
use crate::domain::repositories::landing_repository::LandingRepository;
use crate::infrastructure::database::{
    schema,
    schema::{categories, experiences, locations},
    DbPool,
};
use crate::shared::error_types::{ApiError, ERROR_DATABASE_ERROR};

#[derive(Clone)]
pub struct PostgresLandingRepository {
    pool: DbPool,
}

impl PostgresLandingRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn map_db_error(e: diesel::result::Error, context: &str) -> ApiError {
        ApiError::with_details(ERROR_DATABASE_ERROR, context, &e.to_string())
    }

    fn decimal_to_f64(decimal: BigDecimal) -> f64 {
        decimal.to_f64().unwrap_or(0.0)
    }
}

#[async_trait]
impl LandingRepository for PostgresLandingRepository {
    async fn fetch_highlights(&self, limit: i64) -> Result<Vec<LandingHighlight>, ApiError> {
        use schema::landing_highlights;

        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get DB connection",
                &e.to_string(),
            )
        })?;

        let rows = landing_highlights::table
            .filter(landing_highlights::is_active.eq(true))
            .order((
                landing_highlights::display_order.asc(),
                landing_highlights::created_at.desc(),
            ))
            .limit(limit)
            .select((
                landing_highlights::id,
                landing_highlights::title,
                landing_highlights::subtitle,
                landing_highlights::description,
                landing_highlights::image_url,
                landing_highlights::mobile_image_url,
                landing_highlights::cta_label,
                landing_highlights::cta_url,
                landing_highlights::badge_label,
            ))
            .load::<LandingHighlightModel>(&mut conn)
            .map_err(|e| Self::map_db_error(e, "Failed to load landing highlights"))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn fetch_featured_experiences(
        &self,
        limit: i64,
    ) -> Result<Vec<LandingExperienceSummary>, ApiError> {
        use schema::{categories, experiences, locations};

        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get DB connection",
                &e.to_string(),
            )
        })?;

        let experience_rows = experiences::table
            .filter(experiences::is_active.eq(true))
            .order((
                experiences::is_featured.desc(),
                experiences::featured_rank.asc(),
                experiences::average_rating.desc(),
                experiences::created_at.desc(),
            ))
            .limit(limit)
            .select(ExperienceSummaryModel::as_select())
            .load::<ExperienceSummaryModel>(&mut conn)
            .map_err(|e| Self::map_db_error(e, "Failed to load featured experiences"))?;

        if experience_rows.is_empty() {
            return Ok(vec![]);
        }

        let location_ids: Vec<Uuid> = experience_rows.iter().map(|row| row.location_id).collect();
        let category_ids: Vec<Uuid> = experience_rows.iter().map(|row| row.category_id).collect();

        let location_rows = locations::table
            .filter(locations::id.eq_any(&location_ids))
            .select(LocationSummaryModel::as_select())
            .load::<LocationSummaryModel>(&mut conn)
            .map_err(|e| Self::map_db_error(e, "Failed to load locations for featured experiences"))?;

        let category_rows = categories::table
            .filter(categories::id.eq_any(&category_ids))
            .select(CategorySummaryModel::as_select())
            .load::<CategorySummaryModel>(&mut conn)
            .map_err(|e| Self::map_db_error(e, "Failed to load categories for featured experiences"))?;

        let location_map: HashMap<Uuid, LocationSummaryModel> =
            location_rows.into_iter().map(|loc| (loc.id, loc)).collect();
        let category_map: HashMap<Uuid, CategorySummaryModel> =
            category_rows.into_iter().map(|cat| (cat.id, cat)).collect();

        let featured = experience_rows
            .into_iter()
            .map(|row| {
                let location = location_map.get(&row.location_id);
                let city = location.map(|loc| loc.city.clone());
                let state = location.map(|loc| loc.state.clone());
                let country = location.map(|loc| loc.country.clone());
                let category_name = category_map
                    .get(&row.category_id)
                    .map(|cat| cat.name.clone());

                LandingExperienceSummary {
                    id: row.id,
                    title: row.title,
                    slug: row.slug,
                    summary: row.summary,
                    thumbnail_url: row.thumbnail_url,
                    hero_image_url: row.hero_image_url,
                    price_per_person: Self::decimal_to_f64(row.price_per_person),
                    currency: row.currency,
                    average_rating: row
                        .average_rating
                        .map(Self::decimal_to_f64)
                        .unwrap_or(0.0),
                    review_count: row.review_count.unwrap_or(0),
                    city,
                    state,
                    country,
                    category_name,
                }
            })
            .collect();

        Ok(featured)
    }

    async fn fetch_hero_categories(&self, limit: i64) -> Result<Vec<LandingCategory>, ApiError> {
        use schema::categories;

        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get DB connection",
                &e.to_string(),
            )
        })?;

        let rows = categories::table
            .filter(categories::is_active.eq(true))
            .filter(categories::show_on_home.eq(Some(true)))
            .order((categories::home_order.asc(), categories::updated_at.desc()))
            .limit(limit)
            .select((
                categories::id,
                categories::name,
                categories::description,
                categories::icon_url,
                categories::background_image_url,
            ))
            .load::<LandingCategoryModel>(&mut conn)
            .map_err(|e| Self::map_db_error(e, "Failed to load hero categories"))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn fetch_promotions(&self, limit: i64) -> Result<Vec<LandingPromotion>, ApiError> {
        use schema::promotions;

        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get DB connection",
                &e.to_string(),
            )
        })?;

        let now_ts = Utc::now();
        let now_ts_end = now_ts.clone();

        let rows = promotions::table
            .filter(promotions::is_active.eq(true))
            .filter(
                promotions::start_date
                    .is_null()
                    .or(promotions::start_date.le(now_ts)),
            )
            .filter(
                promotions::end_date
                    .is_null()
                    .or(promotions::end_date.ge(now_ts_end)),
            )
            .order((
                promotions::start_date.desc().nulls_last(),
                promotions::created_at.desc(),
            ))
            .limit(limit)
            .select((
                promotions::id,
                promotions::name,
                promotions::headline,
                promotions::description,
                promotions::discount_type,
                promotions::discount_value,
                promotions::badge_label,
                promotions::cta_label,
                promotions::cta_url,
                promotions::image_url,
                promotions::terms,
            ))
            .load::<LandingPromotionModel>(&mut conn)
            .map_err(|e| Self::map_db_error(e, "Failed to load promotions"))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn fetch_curated_collections(
        &self,
        limit: i64,
    ) -> Result<Vec<LandingCollection>, ApiError> {
        use schema::experience_collections;

        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get DB connection",
                &e.to_string(),
            )
        })?;

        let rows = experience_collections::table
            .filter(experience_collections::is_active.eq(true))
            .order((
                experience_collections::display_order.asc(),
                experience_collections::updated_at.desc(),
            ))
            .limit(limit)
            .select((
                experience_collections::id,
                experience_collections::slug,
                experience_collections::title,
                experience_collections::subtitle,
                experience_collections::description,
                experience_collections::cover_image_url,
            ))
            .load::<LandingCollectionModel>(&mut conn)
            .map_err(|e| Self::map_db_error(e, "Failed to load collections"))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn fetch_testimonials(&self, limit: i64) -> Result<Vec<LandingTestimonial>, ApiError> {
        use schema::{experiences, landing_testimonials};

        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get DB connection",
                &e.to_string(),
            )
        })?;

        let rows =
            landing_testimonials::table
                .left_join(experiences::table.on(
                    landing_testimonials::featured_experience_id.eq(experiences::id.nullable()),
                ))
                .filter(landing_testimonials::is_active.eq(true))
                .order((
                    landing_testimonials::display_order.asc(),
                    landing_testimonials::created_at.desc(),
                ))
                .limit(limit)
                .select((
                    landing_testimonials::id,
                    landing_testimonials::author_name,
                    landing_testimonials::author_city,
                    landing_testimonials::author_country,
                    landing_testimonials::avatar_url,
                    landing_testimonials::quote,
                    landing_testimonials::rating,
                    experiences::title.nullable(),
                    experiences::slug.nullable(),
                ))
                .load::<LandingTestimonialModel>(&mut conn)
                .map_err(|e| Self::map_db_error(e, "Failed to load testimonials"))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[derive(Queryable)]
struct LandingHighlightModel {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub image_url: String,
    pub mobile_image_url: Option<String>,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub badge_label: Option<String>,
}

impl From<LandingHighlightModel> for LandingHighlight {
    fn from(value: LandingHighlightModel) -> Self {
        Self {
            id: value.id,
            title: value.title,
            subtitle: value.subtitle,
            description: value.description,
            image_url: value.image_url,
            mobile_image_url: value.mobile_image_url,
            cta_label: value.cta_label,
            cta_url: value.cta_url,
            badge_label: value.badge_label,
        }
    }
}

#[derive(Queryable)]
struct LandingCategoryModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub background_image_url: Option<String>,
}

impl From<LandingCategoryModel> for LandingCategory {
    fn from(value: LandingCategoryModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            icon_url: value.icon_url,
            background_image_url: value.background_image_url,
        }
    }
}

#[derive(Queryable)]
struct LandingPromotionModel {
    pub id: Uuid,
    pub name: String,
    pub headline: Option<String>,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: BigDecimal,
    pub badge_label: Option<String>,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub image_url: Option<String>,
    pub terms: Option<String>,
}

impl From<LandingPromotionModel> for LandingPromotion {
    fn from(value: LandingPromotionModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            headline: value.headline,
            description: value.description,
            discount_type: value.discount_type,
            discount_value: PostgresLandingRepository::decimal_to_f64(value.discount_value),
            badge_label: value.badge_label,
            cta_label: value.cta_label,
            cta_url: value.cta_url,
            image_url: value.image_url,
            terms: value.terms,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = experiences)]
struct ExperienceSummaryModel {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub thumbnail_url: Option<String>,
    pub hero_image_url: Option<String>,
    pub price_per_person: BigDecimal,
    pub currency: Option<String>,
    pub average_rating: Option<BigDecimal>,
    pub review_count: Option<i32>,
    pub location_id: Uuid,
    pub category_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = locations)]
struct LocationSummaryModel {
    pub id: Uuid,
    pub city: String,
    pub state: String,
    pub country: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = categories)]
struct CategorySummaryModel {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable)]
struct LandingCollectionModel {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub cover_image_url: Option<String>,
}

impl From<LandingCollectionModel> for LandingCollection {
    fn from(value: LandingCollectionModel) -> Self {
        Self {
            id: value.id,
            slug: value.slug,
            title: value.title,
            subtitle: value.subtitle,
            description: value.description,
            cover_image_url: value.cover_image_url,
        }
    }
}

#[derive(Queryable)]
struct LandingTestimonialModel {
    pub id: Uuid,
    pub author_name: String,
    pub author_city: Option<String>,
    pub author_country: Option<String>,
    pub avatar_url: Option<String>,
    pub quote: String,
    pub rating: Option<i32>,
    pub experience_title: Option<String>,
    pub experience_slug: Option<String>,
}

impl From<LandingTestimonialModel> for LandingTestimonial {
    fn from(value: LandingTestimonialModel) -> Self {
        Self {
            id: value.id,
            author_name: value.author_name,
            author_city: value.author_city,
            author_country: value.author_country,
            avatar_url: value.avatar_url,
            quote: value.quote,
            rating: value.rating,
            experience_title: value.experience_title,
            experience_slug: value.experience_slug,
        }
    }
}
