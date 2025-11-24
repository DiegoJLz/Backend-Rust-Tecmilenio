use serde::{Deserialize, Serialize};

use crate::domain::entities::promotion::Promotion;
use crate::domain::entities::landing::LandingPromotion;

#[derive(Debug, Serialize)]
pub struct PromotionResponse {
    pub id: String,
    pub name: String,
    pub headline: Option<String>,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: f64,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub terms: Option<String>,
    pub image_url: Option<String>,
    pub badge_label: Option<String>,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub is_stackable: bool,
    pub is_active: bool,
}

impl From<Promotion> for PromotionResponse {
    fn from(promo: Promotion) -> Self {
        Self {
            id: promo.id.to_string(),
            name: promo.name,
            headline: promo.headline,
            description: promo.description,
            discount_type: promo.discount_type,
            discount_value: promo.discount_value,
            start_date: promo.start_date,
            end_date: promo.end_date,
            terms: promo.terms,
            image_url: promo.image_url,
            badge_label: promo.badge_label,
            cta_label: promo.cta_label,
            cta_url: promo.cta_url,
            is_stackable: promo.is_stackable,
            is_active: promo.is_active,
        }
    }
}

impl From<LandingPromotion> for PromotionResponse {
    fn from(promo: LandingPromotion) -> Self {
        Self {
            id: promo.id.to_string(),
            name: promo.name,
            headline: promo.headline,
            description: promo.description,
            discount_type: promo.discount_type,
            discount_value: promo.discount_value,
            start_date: None, // LandingPromotion no tiene estas fechas
            end_date: None,
            terms: promo.terms,
            image_url: promo.image_url,
            badge_label: promo.badge_label,
            cta_label: promo.cta_label,
            cta_url: promo.cta_url,
            is_stackable: false, // Default
            is_active: true,    // Si está en landing, está activa
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PromotionListResponse {
    pub promotions: Vec<PromotionResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct PromotionQueryParams {
    #[serde(default = "default_only_active")]
    pub only_active: bool,
    pub date: Option<String>, // ISO 8601 date string
}

fn default_only_active() -> bool {
    true
}
