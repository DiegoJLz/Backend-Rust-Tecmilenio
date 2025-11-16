use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct LandingHighlight {
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

#[derive(Clone, Debug)]
pub struct LandingCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub background_image_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LandingPromotion {
    pub id: Uuid,
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

#[derive(Clone, Debug)]
pub struct LandingCollection {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub cover_image_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LandingExperienceSummary {
    pub id: Uuid,
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

#[derive(Clone, Debug)]
pub struct LandingTestimonial {
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

#[derive(Clone, Debug, Default)]
pub struct LandingPageData {
    pub highlights: Vec<LandingHighlight>,
    pub featured_experiences: Vec<LandingExperienceSummary>,
    pub hero_categories: Vec<LandingCategory>,
    pub curated_collections: Vec<LandingCollection>,
    pub promotions: Vec<LandingPromotion>,
    pub testimonials: Vec<LandingTestimonial>,
}
