use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Promotion {
    pub id: Uuid,
    pub name: String,
    pub headline: Option<String>,
    pub description: Option<String>,
    pub discount_type: String, // 'percentage', 'amount', 'bundle'
    pub discount_value: f64,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub terms: Option<String>,
    pub image_url: Option<String>,
    pub badge_label: Option<String>,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub is_stackable: bool,
    pub is_active: bool,
}

impl Promotion {
    pub fn is_valid_for_date(&self, date: Option<DateTime<Utc>>) -> bool {
        if !self.is_active {
            return false;
        }

        let check_date = date.unwrap_or_else(Utc::now);

        if let Some(start) = self.start_date {
            if check_date < start {
                return false;
            }
        }

        if let Some(end) = self.end_date {
            if check_date > end {
                return false;
            }
        }

        true
    }

    pub fn calculate_discount(&self, base_amount: f64) -> f64 {
        match self.discount_type.as_str() {
            "percentage" => base_amount * (self.discount_value / 100.0),
            "amount" => self.discount_value.min(base_amount),
            "bundle" => self.discount_value, // Para bundles, el valor es fijo
            _ => 0.0,
        }
    }
}
