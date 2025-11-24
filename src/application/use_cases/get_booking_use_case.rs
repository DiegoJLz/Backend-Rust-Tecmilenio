use uuid::Uuid;

use crate::domain::entities::booking::Booking;
use crate::domain::repositories::booking_repository::BookingRepository;
use crate::shared::error_types::ApiError;

pub struct GetBookingUseCase<R: BookingRepository> {
    booking_repository: R,
}

impl<R: BookingRepository + Clone> Clone for GetBookingUseCase<R> {
    fn clone(&self) -> Self {
        Self {
            booking_repository: self.booking_repository.clone(),
        }
    }
}

impl<R: BookingRepository> GetBookingUseCase<R> {
    pub fn new(booking_repository: R) -> Self {
        Self { booking_repository }
    }

    pub async fn execute(&self, booking_id: Uuid) -> Result<Booking, ApiError> {
        self.booking_repository
            .find_by_id(booking_id)
            .await?
            .ok_or_else(|| {
                ApiError::new("BOOKING_NOT_FOUND", "Booking not found")
            })
    }
}
