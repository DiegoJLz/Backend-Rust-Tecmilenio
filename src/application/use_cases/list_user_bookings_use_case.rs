use uuid::Uuid;

use crate::domain::entities::booking::Booking;
use crate::domain::repositories::booking_repository::BookingRepository;
use crate::shared::error_types::ApiError;

pub struct ListUserBookingsUseCase<R: BookingRepository> {
    booking_repository: R,
}

impl<R: BookingRepository + Clone> Clone for ListUserBookingsUseCase<R> {
    fn clone(&self) -> Self {
        Self {
            booking_repository: self.booking_repository.clone(),
        }
    }
}

impl<R: BookingRepository> ListUserBookingsUseCase<R> {
    pub fn new(booking_repository: R) -> Self {
        Self { booking_repository }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<Booking>, ApiError> {
        self.booking_repository
            .find_by_user_id(user_id, status)
            .await
    }
}
