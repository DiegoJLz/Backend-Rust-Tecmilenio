use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::booking::{Booking, BookingQuote, CreateBookingRequest};

// ========== REQUEST DTOs ==========

#[derive(Debug, Deserialize)]
pub struct QuoteBookingRequest {
    pub experience_id: String,
    pub schedule_id: String,
    pub number_of_participants: i32,
    pub promotion_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingDto {
    pub experience_id: String,
    pub schedule_id: String,
    pub user_id: String,
    pub number_of_participants: i32,
    pub special_requests: Option<String>,
    pub promotion_id: Option<String>,
}

// ========== RESPONSE DTOs ==========

#[derive(Debug, Serialize)]
pub struct BookingQuoteResponse {
    pub experience_id: String,
    pub schedule_id: String,
    pub number_of_participants: i32,
    pub base_price_per_person: f64,
    pub subtotal: f64,
    pub discount: f64,
    pub total: f64,
    pub promotion_id: Option<String>,
}

impl From<BookingQuote> for BookingQuoteResponse {
    fn from(quote: BookingQuote) -> Self {
        Self {
            experience_id: quote.experience_id.to_string(),
            schedule_id: quote.schedule_id.to_string(),
            number_of_participants: quote.number_of_participants,
            base_price_per_person: quote.base_price_per_person,
            subtotal: quote.subtotal,
            discount: quote.discount,
            total: quote.total,
            promotion_id: quote.promotion_id.map(|id| id.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: String,
    pub booking_reference: String,
    pub experience_id: String,
    pub schedule_id: String,
    pub user_id: String,
    pub number_of_participants: i32,
    pub total_price: f64,
    pub status: String,
    pub special_requests: Option<String>,
    pub booking_date: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Booking> for BookingResponse {
    fn from(booking: Booking) -> Self {
        Self {
            id: booking.id.to_string(),
            booking_reference: booking.booking_reference,
            experience_id: booking.experience_id.to_string(),
            schedule_id: booking.schedule_id.to_string(),
            user_id: booking.user_id.to_string(),
            number_of_participants: booking.number_of_participants,
            total_price: booking.total_price,
            status: booking.status,
            special_requests: booking.special_requests,
            booking_date: booking.booking_date,
            created_at: None, // Se puede agregar si se necesita
            updated_at: None,  // Se puede agregar si se necesita
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BookingListResponse {
    pub bookings: Vec<BookingResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

// ========== HELPERS ==========

impl QuoteBookingRequest {
    pub fn to_domain(&self) -> Result<crate::domain::entities::booking::BookingQuoteRequest, ApiError> {
        let experience_id = Uuid::parse_str(&self.experience_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid experience_id format"))?;
        let schedule_id = Uuid::parse_str(&self.schedule_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid schedule_id format"))?;
        let promotion_id = self.promotion_id.as_ref()
            .map(|id| Uuid::parse_str(id))
            .transpose()
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid promotion_id format"))?;

        Ok(crate::domain::entities::booking::BookingQuoteRequest {
            experience_id,
            schedule_id,
            number_of_participants: self.number_of_participants,
            promotion_id,
        })
    }
}

impl CreateBookingDto {
    pub fn to_domain(&self) -> Result<CreateBookingRequest, ApiError> {
        let experience_id = Uuid::parse_str(&self.experience_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid experience_id format"))?;
        let schedule_id = Uuid::parse_str(&self.schedule_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid schedule_id format"))?;
        let user_id = Uuid::parse_str(&self.user_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid user_id format"))?;
        let promotion_id = self.promotion_id.as_ref()
            .map(|id| Uuid::parse_str(id))
            .transpose()
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid promotion_id format"))?;

        Ok(CreateBookingRequest {
            experience_id,
            schedule_id,
            user_id,
            number_of_participants: self.number_of_participants,
            special_requests: self.special_requests.clone(),
            promotion_id,
        })
    }
}

use crate::shared::error_types::ApiError;

