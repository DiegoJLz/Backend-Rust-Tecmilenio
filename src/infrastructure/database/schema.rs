// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Uuid,
        #[max_length = 20]
        booking_reference -> Varchar,
        experience_id -> Uuid,
        schedule_id -> Uuid,
        user_id -> Uuid,
        number_of_participants -> Int4,
        total_price -> Numeric,
        #[max_length = 20]
        status -> Varchar,
        special_requests -> Nullable<Text>,
        booking_date -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        icon_url -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    email_verification_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        token -> Varchar,
        expires_at -> Timestamptz,
        is_used -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    experience_images (id) {
        id -> Uuid,
        experience_id -> Uuid,
        image_url -> Text,
        is_primary -> Nullable<Bool>,
        display_order -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    experience_schedules (id) {
        id -> Uuid,
        experience_id -> Uuid,
        start_datetime -> Timestamptz,
        end_datetime -> Timestamptz,
        max_available_spots -> Int4,
        current_bookings -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    experience_tags (id) {
        id -> Uuid,
        experience_id -> Uuid,
        #[max_length = 50]
        tag_name -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    experiences (id) {
        id -> Uuid,
        #[max_length = 200]
        title -> Varchar,
        description -> Text,
        host_id -> Uuid,
        category_id -> Uuid,
        location_id -> Uuid,
        price_per_person -> Numeric,
        max_participants -> Int4,
        min_participants -> Int4,
        duration_hours -> Numeric,
        difficulty_level -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    locations (id) {
        id -> Uuid,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 100]
        state -> Varchar,
        #[max_length = 100]
        country -> Varchar,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    payments (id) {
        id -> Uuid,
        booking_id -> Uuid,
        #[max_length = 50]
        payment_method -> Varchar,
        amount -> Numeric,
        #[max_length = 3]
        currency -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Varchar,
        #[max_length = 255]
        transaction_id -> Nullable<Varchar>,
        #[max_length = 50]
        payment_gateway -> Nullable<Varchar>,
        processed_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Uuid,
        experience_id -> Uuid,
        booking_id -> Uuid,
        user_id -> Uuid,
        rating -> Int4,
        #[max_length = 200]
        title -> Nullable<Varchar>,
        comment -> Nullable<Text>,
        is_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_favorites (id) {
        id -> Uuid,
        user_id -> Uuid,
        experience_id -> Uuid,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        avatar_url -> Nullable<Text>,
        is_host -> Nullable<Bool>,
        is_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(bookings -> experience_schedules (schedule_id));
diesel::joinable!(bookings -> experiences (experience_id));
diesel::joinable!(bookings -> users (user_id));
diesel::joinable!(email_verification_tokens -> users (user_id));
diesel::joinable!(experience_images -> experiences (experience_id));
diesel::joinable!(experience_schedules -> experiences (experience_id));
diesel::joinable!(experience_tags -> experiences (experience_id));
diesel::joinable!(experiences -> categories (category_id));
diesel::joinable!(experiences -> locations (location_id));
diesel::joinable!(experiences -> users (host_id));
diesel::joinable!(payments -> bookings (booking_id));
diesel::joinable!(reviews -> bookings (booking_id));
diesel::joinable!(reviews -> experiences (experience_id));
diesel::joinable!(reviews -> users (user_id));
diesel::joinable!(user_favorites -> experiences (experience_id));
diesel::joinable!(user_favorites -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    categories,
    email_verification_tokens,
    experience_images,
    experience_schedules,
    experience_tags,
    experiences,
    locations,
    payments,
    reviews,
    user_favorites,
    users,
);
