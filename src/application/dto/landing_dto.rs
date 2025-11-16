use serde::Serialize;

use crate::domain::entities::landing::{
    LandingCategory, LandingCollection, LandingExperienceSummary, LandingHighlight,
    LandingPageData, LandingPromotion, LandingTestimonial,
};

#[derive(Serialize)]
pub struct LandingHighlightDto {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub image_url: String,
    pub mobile_image_url: Option<String>,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub badge_label: Option<String>,
}

impl From<LandingHighlight> for LandingHighlightDto {
    fn from(value: LandingHighlight) -> Self {
        Self {
            id: value.id.to_string(),
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

#[derive(Serialize)]
pub struct LandingCategoryDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub background_image_url: Option<String>,
}

impl From<LandingCategory> for LandingCategoryDto {
    fn from(value: LandingCategory) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            description: value.description,
            icon_url: value.icon_url,
            background_image_url: value.background_image_url,
        }
    }
}

#[derive(Serialize)]
pub struct LandingPromotionDto {
    pub id: String,
    pub name: String,
    pub headline: Option<String>,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: f64,
    pub badge_label: Option<String>,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub image_url: Option<String>,
    pub terms: Option<String>,
}

impl From<LandingPromotion> for LandingPromotionDto {
    fn from(value: LandingPromotion) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            headline: value.headline,
            description: value.description,
            discount_type: value.discount_type,
            discount_value: value.discount_value,
            badge_label: value.badge_label,
            cta_label: value.cta_label,
            cta_url: value.cta_url,
            image_url: value.image_url,
            terms: value.terms,
        }
    }
}

#[derive(Serialize)]
pub struct LandingCollectionDto {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub cover_image_url: Option<String>,
}

impl From<LandingCollection> for LandingCollectionDto {
    fn from(value: LandingCollection) -> Self {
        Self {
            id: value.id.to_string(),
            slug: value.slug,
            title: value.title,
            subtitle: value.subtitle,
            description: value.description,
            cover_image_url: value.cover_image_url,
        }
    }
}

#[derive(Serialize)]
pub struct LandingExperienceSummaryDto {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub thumbnail_url: Option<String>,
    pub hero_image_url: Option<String>,
    pub price_per_person: f64,
    pub currency: Option<String>,
    pub average_rating: f64,
    pub review_count: i32,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub category_name: Option<String>,
}

impl From<LandingExperienceSummary> for LandingExperienceSummaryDto {
    fn from(value: LandingExperienceSummary) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            slug: value.slug,
            summary: value.summary,
            thumbnail_url: value.thumbnail_url,
            hero_image_url: value.hero_image_url,
            price_per_person: value.price_per_person,
            currency: value.currency,
            average_rating: value.average_rating,
            review_count: value.review_count,
            city: value.city,
            state: value.state,
            country: value.country,
            category_name: value.category_name,
        }
    }
}

#[derive(Serialize)]
pub struct LandingTestimonialDto {
    pub id: String,
    pub author_name: String,
    pub author_city: Option<String>,
    pub author_country: Option<String>,
    pub avatar_url: Option<String>,
    pub quote: String,
    pub rating: Option<i32>,
    pub experience_title: Option<String>,
    pub experience_slug: Option<String>,
}

impl From<LandingTestimonial> for LandingTestimonialDto {
    fn from(value: LandingTestimonial) -> Self {
        Self {
            id: value.id.to_string(),
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

#[derive(Serialize)]
pub struct LandingPageResponse {
    pub highlights: Vec<LandingHighlightDto>,
    pub featured_experiences: Vec<LandingExperienceSummaryDto>,
    pub hero_categories: Vec<LandingCategoryDto>,
    pub curated_collections: Vec<LandingCollectionDto>,
    pub promotions: Vec<LandingPromotionDto>,
    pub testimonials: Vec<LandingTestimonialDto>,
}

impl From<LandingPageData> for LandingPageResponse {
    fn from(value: LandingPageData) -> Self {
        Self {
            highlights: value.highlights.into_iter().map(Into::into).collect(),
            featured_experiences: value
                .featured_experiences
                .into_iter()
                .map(Into::into)
                .collect(),
            hero_categories: value.hero_categories.into_iter().map(Into::into).collect(),
            curated_collections: value
                .curated_collections
                .into_iter()
                .map(Into::into)
                .collect(),
            promotions: value.promotions.into_iter().map(Into::into).collect(),
            testimonials: value.testimonials.into_iter().map(Into::into).collect(),
        }
    }
}
