use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Dominio de una reserva de experiencia.
#[derive(Clone, Debug)]
pub struct Booking {
    pub id: Uuid,
    pub booking_reference: String,
    pub experience_id: Uuid,
    pub schedule_id: Uuid,
    pub user_id: Uuid,
    pub number_of_participants: i32,
    pub total_price: f64,
    pub status: String,
    pub special_requests: Option<String>,
    pub booking_date: Option<DateTime<Utc>>,
}

/// Datos necesarios para cotizar una reserva.
#[derive(Clone, Debug)]
pub struct BookingQuoteRequest {
    pub experience_id: Uuid,
    pub schedule_id: Uuid,
    pub number_of_participants: i32,
    pub promotion_id: Option<Uuid>,
}

/// Resultado de la cotización.
#[derive(Clone, Debug)]
pub struct BookingQuote {
    pub experience_id: Uuid,
    pub schedule_id: Uuid,
    pub number_of_participants: i32,
    pub base_price_per_person: f64,
    pub subtotal: f64,
    pub discount: f64,
    pub total: f64,
    pub promotion_id: Option<Uuid>,
}

/// Datos para crear la reserva en base a una cotización válida.
#[derive(Clone, Debug)]
pub struct CreateBookingRequest {
    pub experience_id: Uuid,
    pub schedule_id: Uuid,
    pub user_id: Uuid,
    pub number_of_participants: i32,
    pub special_requests: Option<String>,
    pub promotion_id: Option<Uuid>,
}
