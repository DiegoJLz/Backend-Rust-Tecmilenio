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
        background_image_url -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        show_on_home -> Nullable<Bool>,
        home_order -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    email_verification_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        expires_at -> Timestamptz,
        is_used -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        session_token -> Text,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        expires_at -> Timestamptz,
        is_active -> Nullable<Bool>,
        ip_address -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    access_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        token_type -> Varchar,
        expires_at -> Timestamptz,
        is_used -> Nullable<Bool>,
        is_revoked -> Nullable<Bool>,
        metadata -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
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
        #[max_length = 200]
        slug -> Varchar,
        summary -> Nullable<Text>,
        description -> Text,
        hero_image_url -> Nullable<Text>,
        thumbnail_url -> Nullable<Text>,
        host_id -> Uuid,
        category_id -> Uuid,
        location_id -> Uuid,
        price_per_person -> Numeric,
        max_participants -> Int4,
        min_participants -> Int4,
        duration_hours -> Numeric,
        difficulty_level -> Nullable<Int4>,
        average_rating -> Nullable<Numeric>,
        review_count -> Nullable<Int4>,
        #[max_length = 50]
        language -> Nullable<Varchar>,
        #[max_length = 3]
        currency -> Nullable<Varchar>,
        featured_rank -> Nullable<Int4>,
        is_featured -> Nullable<Bool>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    landing_highlights (id) {
        id -> Uuid,
        #[max_length = 150]
        title -> Varchar,
        #[max_length = 255]
        subtitle -> Nullable<Varchar>,
        description -> Nullable<Text>,
        image_url -> Text,
        mobile_image_url -> Nullable<Text>,
        #[max_length = 100]
        cta_label -> Nullable<Varchar>,
        cta_url -> Nullable<Text>,
        #[max_length = 50]
        badge_label -> Nullable<Varchar>,
        display_order -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    promotions (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        #[max_length = 255]
        headline -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 20]
        discount_type -> Varchar,
        discount_value -> Numeric,
        start_date -> Nullable<Timestamptz>,
        end_date -> Nullable<Timestamptz>,
        terms -> Nullable<Text>,
        image_url -> Nullable<Text>,
        #[max_length = 100]
        badge_label -> Nullable<Varchar>,
        #[max_length = 100]
        cta_label -> Nullable<Varchar>,
        cta_url -> Nullable<Text>,
        is_stackable -> Nullable<Bool>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    promotion_experiences (id) {
        id -> Uuid,
        promotion_id -> Uuid,
        experience_id -> Uuid,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    experience_collections (id) {
        id -> Uuid,
        #[max_length = 100]
        slug -> Varchar,
        #[max_length = 150]
        title -> Varchar,
        #[max_length = 255]
        subtitle -> Nullable<Varchar>,
        description -> Nullable<Text>,
        cover_image_url -> Nullable<Text>,
        filter_criteria -> Nullable<Jsonb>,
        display_order -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    experience_collection_items (id) {
        id -> Uuid,
        collection_id -> Uuid,
        experience_id -> Uuid,
        display_order -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    landing_testimonials (id) {
        id -> Uuid,
        review_id -> Nullable<Uuid>,
        #[max_length = 150]
        author_name -> Varchar,
        #[max_length = 150]
        author_city -> Nullable<Varchar>,
        #[max_length = 150]
        author_country -> Nullable<Varchar>,
        avatar_url -> Nullable<Text>,
        quote -> Text,
        rating -> Nullable<Int4>,
        featured_experience_id -> Nullable<Uuid>,
        display_order -> Nullable<Int4>,
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
        #[max_length = 255]
        password_hash -> Varchar,
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
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(access_tokens -> users (user_id));
diesel::joinable!(experience_images -> experiences (experience_id));
diesel::joinable!(experience_schedules -> experiences (experience_id));
diesel::joinable!(experience_tags -> experiences (experience_id));
diesel::joinable!(experiences -> categories (category_id));
diesel::joinable!(experiences -> locations (location_id));
diesel::joinable!(experiences -> users (host_id));
diesel::joinable!(promotion_experiences -> promotions (promotion_id));
diesel::joinable!(promotion_experiences -> experiences (experience_id));
diesel::joinable!(experience_collection_items -> experience_collections (collection_id));
diesel::joinable!(experience_collection_items -> experiences (experience_id));
diesel::joinable!(payments -> bookings (booking_id));
diesel::joinable!(reviews -> bookings (booking_id));
diesel::joinable!(reviews -> experiences (experience_id));
diesel::joinable!(reviews -> users (user_id));
diesel::joinable!(landing_testimonials -> experiences (featured_experience_id));
diesel::joinable!(landing_testimonials -> reviews (review_id));
diesel::joinable!(user_favorites -> experiences (experience_id));
diesel::joinable!(user_favorites -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    categories,
    experience_collection_items,
    experience_collections,
    email_verification_tokens,
    landing_highlights,
    landing_testimonials,
    promotion_experiences,
    promotions,
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
