use async_trait::async_trait;
use bigdecimal::ToPrimitive;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::entities::landing::LandingPromotion;
use crate::domain::repositories::promotion_repository::PromotionRepository;
use crate::infrastructure::database::{schema, DbPool};
use crate::shared::error_types::{ApiError, ERROR_DATABASE_ERROR};

#[derive(Clone)]
pub struct PostgresPromotionRepository {
    pool: DbPool,
}

impl PostgresPromotionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_db_error(context: &str, e: diesel::result::Error) -> ApiError {
    ApiError::with_details(
        ERROR_DATABASE_ERROR,
        context,
        &e.to_string(),
    )
}

#[async_trait]
impl PromotionRepository for PostgresPromotionRepository {
    async fn list_active_promotions(&self) -> Result<Vec<LandingPromotion>, ApiError> {
        use schema::promotions::dsl as promotions_dsl;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| map_db_error("Failed to get database connection", diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )))?;

        let rows = promotions_dsl::promotions
            .filter(promotions_dsl::is_active.eq(true))
            .order(promotions_dsl::created_at.desc().nulls_last())
            .select((
                promotions_dsl::id,
                promotions_dsl::name,
                promotions_dsl::headline,
                promotions_dsl::description,
                promotions_dsl::discount_type,
                promotions_dsl::discount_value,
                promotions_dsl::start_date,
                promotions_dsl::end_date,
                promotions_dsl::terms,
                promotions_dsl::image_url,
                promotions_dsl::badge_label,
                promotions_dsl::cta_label,
                promotions_dsl::cta_url,
                promotions_dsl::is_stackable,
                promotions_dsl::is_active,
                promotions_dsl::created_at,
                promotions_dsl::updated_at,
            ))
            .load::<(
                Uuid,
                String,
                Option<String>,
                Option<String>,
                String,
                bigdecimal::BigDecimal,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<bool>,
                Option<bool>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
            )>(&mut conn)
            .map_err(|e| map_db_error("Failed to load promotions", e))?;

        Ok(rows
            .into_iter()
            .map(
                |(
                    id,
                    name,
                    headline,
                    description,
                    discount_type,
                    discount_value,
                    _start_date,
                    _end_date,
                    terms,
                    image_url,
                    badge_label,
                    cta_label,
                    cta_url,
                    _is_stackable,
                    _is_active,
                    _created_at,
                    _updated_at,
                )| LandingPromotion {
                    id,
                    name,
                    headline,
                    description,
                    discount_type,
                    discount_value: discount_value.to_f64().unwrap_or(0.0),
                    badge_label,
                    cta_label,
                    cta_url,
                    image_url,
                    terms,
                },
            )
            .collect())
    }

    async fn get_promotion_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LandingPromotion>, ApiError> {
        use schema::promotions::dsl as promotions_dsl;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| map_db_error("Failed to get database connection", diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )))?;

        let row = promotions_dsl::promotions
            .filter(promotions_dsl::id.eq(id))
            .select((
                promotions_dsl::id,
                promotions_dsl::name,
                promotions_dsl::headline,
                promotions_dsl::description,
                promotions_dsl::discount_type,
                promotions_dsl::discount_value,
                promotions_dsl::start_date,
                promotions_dsl::end_date,
                promotions_dsl::terms,
                promotions_dsl::image_url,
                promotions_dsl::badge_label,
                promotions_dsl::cta_label,
                promotions_dsl::cta_url,
                promotions_dsl::is_stackable,
                promotions_dsl::is_active,
                promotions_dsl::created_at,
                promotions_dsl::updated_at,
            ))
            .first::<(
                Uuid,
                String,
                Option<String>,
                Option<String>,
                String,
                bigdecimal::BigDecimal,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<bool>,
                Option<bool>,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
            )>(&mut conn)
            .optional()
            .map_err(|e| map_db_error("Failed to load promotion by id", e))?;

        Ok(row.map(
            |(
                id,
                name,
                headline,
                description,
                discount_type,
                discount_value,
                _start_date,
                _end_date,
                terms,
                image_url,
                badge_label,
                cta_label,
                cta_url,
                _is_stackable,
                _is_active,
                _created_at,
                _updated_at,
            )| LandingPromotion {
                id,
                name,
                headline,
                description,
                discount_type,
                discount_value: discount_value.to_f64().unwrap_or(0.0),
                badge_label,
                cta_label,
                cta_url,
                image_url,
                terms,
            },
        ))
    }
}
