use uuid::Uuid;

use crate::application::dto::booking_dto::{
    BookingListResponse, BookingQuoteResponse, BookingResponse, CreateBookingDto, QuoteBookingRequest,
};
use crate::application::use_cases::{
    create_booking_use_case::CreateBookingUseCase,
    get_booking_use_case::GetBookingUseCase,
    list_user_bookings_use_case::ListUserBookingsUseCase,
    quote_booking_use_case::QuoteBookingUseCase,
};
use crate::infrastructure::repositories::postgres_booking_repository::PostgresBookingRepository;
use crate::shared::error_types::ApiError;

#[derive(Clone)]
pub struct BookingHandler {
    quote_booking_use_case: QuoteBookingUseCase<PostgresBookingRepository>,
    create_booking_use_case: CreateBookingUseCase<PostgresBookingRepository>,
    get_booking_use_case: GetBookingUseCase<PostgresBookingRepository>,
    list_user_bookings_use_case: ListUserBookingsUseCase<PostgresBookingRepository>,
}

impl BookingHandler {
    pub fn new(booking_repository: PostgresBookingRepository) -> Self {
        let quote_booking_use_case = QuoteBookingUseCase::new(booking_repository.clone());
        let create_booking_use_case = CreateBookingUseCase::new(booking_repository.clone());
        let get_booking_use_case = GetBookingUseCase::new(booking_repository.clone());
        let list_user_bookings_use_case = ListUserBookingsUseCase::new(booking_repository);

        Self {
            quote_booking_use_case,
            create_booking_use_case,
            get_booking_use_case,
            list_user_bookings_use_case,
        }
    }

    pub async fn quote_booking(
        &self,
        request: QuoteBookingRequest,
    ) -> Result<BookingQuoteResponse, ApiError> {
        let domain_request = request.to_domain()?;
        let quote = self.quote_booking_use_case.execute(domain_request).await?;
        Ok(BookingQuoteResponse::from(quote))
    }

    pub async fn create_booking(
        &self,
        request: CreateBookingDto,
    ) -> Result<BookingResponse, ApiError> {
        let domain_request = request.to_domain()?;
        let booking = self.create_booking_use_case.execute(domain_request).await?;
        Ok(BookingResponse::from(booking))
    }

    pub async fn get_booking(&self, booking_id: String) -> Result<BookingResponse, ApiError> {
        let id = Uuid::parse_str(&booking_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid booking_id format"))?;
        let booking = self.get_booking_use_case.execute(id).await?;
        Ok(BookingResponse::from(booking))
    }

    pub async fn list_user_bookings(
        &self,
        user_id: String,
        status: Option<String>,
    ) -> Result<BookingListResponse, ApiError> {
        let id = Uuid::parse_str(&user_id)
            .map_err(|_| ApiError::new("INVALID_UUID", "Invalid user_id format"))?;
        let bookings = self.list_user_bookings_use_case.execute(id, status).await?;
        let booking_responses: Vec<BookingResponse> =
            bookings.into_iter().map(BookingResponse::from).collect();
        let total = booking_responses.len();
        Ok(BookingListResponse {
            bookings: booking_responses,
            total,
            page: 1,
            per_page: total,
        })
    }
}
