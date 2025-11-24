use async_trait::async_trait;
use bigdecimal::ToPrimitive;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::entities::booking::{
    Booking, BookingQuote, BookingQuoteRequest, CreateBookingRequest,
};
use crate::domain::repositories::booking_repository::BookingRepository;
use crate::infrastructure::database::{schema, DbPool};
use crate::shared::error_types::{
    ApiError, ERROR_DATABASE_ERROR, ERROR_INTERNAL_SERVER_ERROR,
};

#[derive(Clone)]
pub struct PostgresBookingRepository {
    pool: DbPool,
}

impl PostgresBookingRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn map_db_error(context: &str, e: diesel::result::Error) -> ApiError {
    ApiError::with_details(
        ERROR_DATABASE_ERROR,
        context,
        &e.to_string(),
    )
}

#[async_trait]
impl BookingRepository for PostgresBookingRepository {
    async fn calculate_quote(
        &self,
        request: &BookingQuoteRequest,
    ) -> Result<BookingQuote, ApiError> {
        use schema::experience_schedules::dsl as schedules_dsl;
        use schema::experiences::dsl as experiences_dsl;
        use schema::promotions::dsl as promotions_dsl;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        // Validar que el schedule pertenezca a la experiencia y esté activo
        let schedule = schedules_dsl::experience_schedules
            .filter(schedules_dsl::id.eq(request.schedule_id))
            .filter(schedules_dsl::experience_id.eq(request.experience_id))
            .filter(schedules_dsl::is_active.eq(true))
            .first::<(Uuid, Uuid, DateTime<Utc>, DateTime<Utc>, i32, Option<i32>, Option<bool>, Option<DateTime<Utc>>, Option<DateTime<Utc>>)>(&mut conn)
            .optional()
            .map_err(|e| map_db_error("Failed to validate schedule", e))?;

        if schedule.is_none() {
            return Err(ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Invalid schedule",
                "Schedule does not exist or does not belong to the experience",
            ));
        }

        // Obtener precio base de la experiencia
        let experience = experiences_dsl::experiences
            .filter(experiences_dsl::id.eq(request.experience_id))
            .select((
                experiences_dsl::id,
                experiences_dsl::price_per_person,
                experiences_dsl::currency,
            ))
            .first::<(Uuid, bigdecimal::BigDecimal, Option<String>)>(&mut conn)
            .map_err(|e| map_db_error("Failed to load experience price", e))?;

        let price_per_person = experience
            .1
            .to_f64()
            .ok_or_else(|| ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Invalid price format",
                "Could not convert price_per_person to f64",
            ))?;

        let subtotal = price_per_person * request.number_of_participants as f64;

        // Aplicar promoción si viene en la solicitud
        let mut discount = 0.0_f64;
        if let Some(promotion_id) = request.promotion_id {
            if let Some(promo) = promotions_dsl::promotions
                .filter(promotions_dsl::id.eq(promotion_id))
                .filter(promotions_dsl::is_active.eq(true))
                .first::<(
                    Uuid,
                    String,
                    Option<String>,
                    Option<String>,
                    String,
                    bigdecimal::BigDecimal,
                    Option<DateTime<Utc>>,
                    Option<DateTime<Utc>>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<bool>,
                    Option<bool>,
                    Option<DateTime<Utc>>,
                    Option<DateTime<Utc>>,
                )>(&mut conn)
                .optional()
                .map_err(|e| map_db_error("Failed to load promotion", e))?
            {
                let discount_type = promo.4;
                let discount_value = promo
                    .5
                    .to_f64()
                    .ok_or_else(|| ApiError::with_details(
                        ERROR_INTERNAL_SERVER_ERROR,
                        "Invalid discount format",
                        "Could not convert discount_value to f64",
                    ))?;

                discount = match discount_type.as_str() {
                    "percentage" => subtotal * (discount_value / 100.0),
                    "amount" => discount_value.min(subtotal),
                    _ => 0.0,
                };
            }
        }

        let total = (subtotal - discount).max(0.0);

        Ok(BookingQuote {
            experience_id: request.experience_id,
            schedule_id: request.schedule_id,
            number_of_participants: request.number_of_participants,
            base_price_per_person: price_per_person,
            subtotal,
            discount,
            total,
            promotion_id: request.promotion_id,
        })
    }

    async fn create_booking(
        &self,
        request: &CreateBookingRequest,
        total: f64,
    ) -> Result<Booking, ApiError> {
        use schema::bookings::dsl as bookings_dsl;
        use schema::experience_schedules::dsl as schedules_dsl;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        // Validar disponibilidad antes de crear la reserva
        let schedule = schedules_dsl::experience_schedules
            .filter(schedules_dsl::id.eq(request.schedule_id))
            .filter(schedules_dsl::experience_id.eq(request.experience_id))
            .filter(schedules_dsl::is_active.eq(true))
            .first::<(Uuid, Uuid, DateTime<Utc>, DateTime<Utc>, i32, Option<i32>, Option<bool>, Option<DateTime<Utc>>, Option<DateTime<Utc>>)>(&mut conn)
            .optional()
            .map_err(|e| map_db_error("Failed to validate schedule", e))?;

        if schedule.is_none() {
            return Err(ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Invalid schedule",
                "Schedule does not exist or does not belong to the experience",
            ));
        }

        let (_, _, _, _, max_spots, current_bookings, _, _, _) = schedule.unwrap();
        let available_spots = max_spots - current_bookings.unwrap_or(0);
        if request.number_of_participants > available_spots {
            return Err(ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Insufficient availability",
                &format!("Only {} spots available, requested {}", available_spots, request.number_of_participants),
            ));
        }

        #[derive(Insertable)]
        #[diesel(table_name = schema::bookings)]
        struct NewBooking {
            id: Uuid,
            booking_reference: String,
            experience_id: Uuid,
            schedule_id: Uuid,
            user_id: Uuid,
            number_of_participants: i32,
            total_price: bigdecimal::BigDecimal,
            status: String,
            special_requests: Option<String>,
            booking_date: Option<DateTime<Utc>>,
        }

        let id = Uuid::new_v4();
        let booking_reference = format!("BK-{}", id.simple());

        let total_bigdecimal =
            bigdecimal::BigDecimal::from_f64(total).ok_or_else(|| {
                ApiError::with_details(
                    ERROR_INTERNAL_SERVER_ERROR,
                    "Invalid total format",
                    "Could not convert total to BigDecimal",
                )
            })?;

        let new_booking = NewBooking {
            id,
            booking_reference: booking_reference.clone(),
            experience_id: request.experience_id,
            schedule_id: request.schedule_id,
            user_id: request.user_id,
            number_of_participants: request.number_of_participants,
            total_price: total_bigdecimal,
            status: "pending".to_string(),
            special_requests: request.special_requests.clone(),
            booking_date: Some(Utc::now()),
        };

        diesel::insert_into(bookings_dsl::bookings)
            .values(&new_booking)
            .execute(&mut conn)
            .map_err(|e| map_db_error("Failed to create booking", e))?;

        Ok(Booking {
            id,
            booking_reference,
            experience_id: request.experience_id,
            schedule_id: request.schedule_id,
            user_id: request.user_id,
            number_of_participants: request.number_of_participants,
            total_price: total,
            status: "pending".to_string(),
            special_requests: request.special_requests.clone(),
            booking_date: Some(Utc::now()),
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Booking>, ApiError> {
        use schema::bookings::dsl as bookings_dsl;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let result = bookings_dsl::bookings
            .filter(bookings_dsl::id.eq(id))
            .select((
                bookings_dsl::id,
                bookings_dsl::booking_reference,
                bookings_dsl::experience_id,
                bookings_dsl::schedule_id,
                bookings_dsl::user_id,
                bookings_dsl::number_of_participants,
                bookings_dsl::total_price,
                bookings_dsl::status,
                bookings_dsl::special_requests,
                bookings_dsl::booking_date,
            ))
            .first::<(
                Uuid,
                String,
                Uuid,
                Uuid,
                Uuid,
                i32,
                bigdecimal::BigDecimal,
                String,
                Option<String>,
                Option<DateTime<Utc>>,
            )>(&mut conn)
            .optional()
            .map_err(|e| map_db_error("Failed to load booking by id", e))?;

        Ok(result.map(
            |(
                id,
                booking_reference,
                experience_id,
                schedule_id,
                user_id,
                number_of_participants,
                total_price,
                status,
                special_requests,
                booking_date,
            )| Booking {
                id,
                booking_reference,
                experience_id,
                schedule_id,
                user_id,
                number_of_participants,
                total_price: total_price.to_f64().unwrap_or(0.0),
                status,
                special_requests,
                booking_date,
            },
        ))
    }

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<Booking>, ApiError> {
        use schema::bookings::dsl as bookings_dsl;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let mut query = bookings_dsl::bookings
            .filter(bookings_dsl::user_id.eq(user_id))
            .into_boxed();

        if let Some(status_filter) = status {
            query = query.filter(bookings_dsl::status.eq(status_filter));
        }

        let rows = query
            .order(bookings_dsl::booking_date.desc())
            .select((
                bookings_dsl::id,
                bookings_dsl::booking_reference,
                bookings_dsl::experience_id,
                bookings_dsl::schedule_id,
                bookings_dsl::user_id,
                bookings_dsl::number_of_participants,
                bookings_dsl::total_price,
                bookings_dsl::status,
                bookings_dsl::special_requests,
                bookings_dsl::booking_date,
            ))
            .load::<(
                Uuid,
                String,
                Uuid,
                Uuid,
                Uuid,
                i32,
                bigdecimal::BigDecimal,
                String,
                Option<String>,
                Option<DateTime<Utc>>,
            )>(&mut conn)
            .map_err(|e| map_db_error("Failed to list bookings by user", e))?;

        Ok(rows
            .into_iter()
            .map(
                |(
                    id,
                    booking_reference,
                    experience_id,
                    schedule_id,
                    user_id,
                    number_of_participants,
                    total_price,
                    status,
                    special_requests,
                    booking_date,
                )| Booking {
                    id,
                    booking_reference,
                    experience_id,
                    schedule_id,
                    user_id,
                    number_of_participants,
                    total_price: total_price.to_f64().unwrap_or(0.0),
                    status,
                    special_requests,
                    booking_date,
                },
            )
            .collect())
    }
}
