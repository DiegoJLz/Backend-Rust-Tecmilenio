use crate::domain::entities::booking::{BookingQuote, BookingQuoteRequest};
use crate::domain::repositories::booking_repository::BookingRepository;
use crate::shared::error_types::ApiError;

pub struct QuoteBookingUseCase<R: BookingRepository> {
    booking_repository: R,
}

impl<R: BookingRepository + Clone> Clone for QuoteBookingUseCase<R> {
    fn clone(&self) -> Self {
        Self {
            booking_repository: self.booking_repository.clone(),
        }
    }
}

impl<R: BookingRepository> QuoteBookingUseCase<R> {
    pub fn new(booking_repository: R) -> Self {
        Self { booking_repository }
    }

    pub async fn execute(&self, request: BookingQuoteRequest) -> Result<BookingQuote, ApiError> {
        self.booking_repository.calculate_quote(&request).await
    }
}
