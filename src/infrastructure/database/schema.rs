// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Uuid,
        booking_reference -> Varchar,
        experience_id -> Uuid,
        schedule_id -> Uuid,
        user_id -> Uuid,
        number_of_participants -> Int4,
        total_price -> Numeric,
        status -> Varchar,
        special_requests -> Nullable<Text>,
        booking_date -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        icon_url -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    experience_images (id) {
        id -> Uuid,
        experience_id -> Uuid,
        image_url -> Text,
        is_primary -> Bool,
        display_order -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    experience_schedules (id) {
        id -> Uuid,
        experience_id -> Uuid,
        start_datetime -> Timestamptz,
        end_datetime -> Timestamptz,
        max_available_spots -> Int4,
        current_bookings -> Int4,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    experience_tags (id) {
        id -> Uuid,
        experience_id -> Uuid,
        tag_name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    experiences (id) {
        id -> Uuid,
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
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    locations (id) {
        id -> Uuid,
        city -> Varchar,
        state -> Varchar,
        country -> Varchar,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    payments (id) {
        id -> Uuid,
        booking_id -> Uuid,
        payment_method -> Varchar,
        amount -> Numeric,
        currency -> Varchar,
        status -> Varchar,
        transaction_id -> Nullable<Varchar>,
        payment_gateway -> Nullable<Varchar>,
        processed_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    reviews (id) {
        id -> Uuid,
        experience_id -> Uuid,
        booking_id -> Uuid,
        user_id -> Uuid,
        rating -> Int4,
        title -> Nullable<Varchar>,
        comment -> Nullable<Text>,
        is_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_favorites (id) {
        id -> Uuid,
        user_id -> Uuid,
        experience_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        phone -> Nullable<Varchar>,
        avatar_url -> Nullable<Text>,
        is_host -> Bool,
        is_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(bookings -> experiences (experience_id));
diesel::joinable!(bookings -> experience_schedules (schedule_id));
diesel::joinable!(bookings -> users (user_id));
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
