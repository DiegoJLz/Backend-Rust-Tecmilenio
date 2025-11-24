use crate::domain::entities::booking::{Booking, CreateBookingRequest};
use crate::domain::repositories::booking_repository::BookingRepository;
use crate::shared::error_types::ApiError;

pub struct CreateBookingUseCase<R: BookingRepository> {
    booking_repository: R,
}

impl<R: BookingRepository + Clone> Clone for CreateBookingUseCase<R> {
    fn clone(&self) -> Self {
        Self {
            booking_repository: self.booking_repository.clone(),
        }
    }
}

impl<R: BookingRepository> CreateBookingUseCase<R> {
    pub fn new(booking_repository: R) -> Self {
        Self { booking_repository }
    }

    pub async fn execute(&self, request: CreateBookingRequest) -> Result<Booking, ApiError> {
        // Primero validar disponibilidad y calcular precio
        let quote_request = crate::domain::entities::booking::BookingQuoteRequest {
            experience_id: request.experience_id,
            schedule_id: request.schedule_id,
            number_of_participants: request.number_of_participants,
            promotion_id: request.promotion_id,
        };

        let quote = self.booking_repository.calculate_quote(&quote_request).await?;

        // Crear la reserva con el precio calculado
        self.booking_repository
            .create_booking(&request, quote.total)
            .await
    }
}
