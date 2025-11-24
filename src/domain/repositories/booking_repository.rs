use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::booking::{
    Booking, BookingQuote, BookingQuoteRequest, CreateBookingRequest,
};
use crate::shared::error_types::ApiError;

/// Repositorio de alto nivel para gestionar reservas.
#[async_trait]
pub trait BookingRepository: Send + Sync {
    /// Calcula una cotización para una posible reserva.
    async fn calculate_quote(
        &self,
        request: &BookingQuoteRequest,
    ) -> Result<BookingQuote, ApiError>;

    /// Crea una reserva a partir de los datos proporcionados y el total calculado.
    async fn create_booking(
        &self,
        request: &CreateBookingRequest,
        total: f64,
    ) -> Result<Booking, ApiError>;

    /// Obtiene una reserva por ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Booking>, ApiError>;

    /// Lista todas las reservas de un usuario, opcionalmente filtradas por status.
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<Booking>, ApiError>;
}
