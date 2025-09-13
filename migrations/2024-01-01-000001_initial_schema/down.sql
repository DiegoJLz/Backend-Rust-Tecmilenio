-- Migration: Rollback Initial Schema for Marketplace de Experiencias Locales
-- Created: 2024-01-01

-- Drop triggers first
DROP TRIGGER IF EXISTS booking_count_trigger ON bookings;
DROP TRIGGER IF EXISTS generate_booking_reference_trigger ON bookings;
DROP TRIGGER IF EXISTS update_reviews_updated_at ON reviews;
DROP TRIGGER IF EXISTS update_payments_updated_at ON payments;
DROP TRIGGER IF EXISTS update_bookings_updated_at ON bookings;
DROP TRIGGER IF EXISTS update_experience_schedules_updated_at ON experience_schedules;
DROP TRIGGER IF EXISTS update_experiences_updated_at ON experiences;
DROP TRIGGER IF EXISTS update_categories_updated_at ON categories;
DROP TRIGGER IF EXISTS update_locations_updated_at ON locations;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;

-- Drop functions
DROP FUNCTION IF EXISTS update_booking_count();
DROP FUNCTION IF EXISTS generate_booking_reference();
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes
DROP INDEX IF EXISTS idx_reviews_rating;
DROP INDEX IF EXISTS idx_reviews_user_id;
DROP INDEX IF EXISTS idx_reviews_experience_id;
DROP INDEX IF EXISTS idx_payments_status;
DROP INDEX IF EXISTS idx_payments_booking_id;
DROP INDEX IF EXISTS idx_bookings_booking_date;
DROP INDEX IF EXISTS idx_bookings_status;
DROP INDEX IF EXISTS idx_bookings_schedule_id;
DROP INDEX IF EXISTS idx_bookings_experience_id;
DROP INDEX IF EXISTS idx_bookings_user_id;
DROP INDEX IF EXISTS idx_experience_schedules_is_active;
DROP INDEX IF EXISTS idx_experience_schedules_start_datetime;
DROP INDEX IF EXISTS idx_experience_schedules_experience_id;
DROP INDEX IF EXISTS idx_experiences_is_active;
DROP INDEX IF EXISTS idx_experiences_price;
DROP INDEX IF EXISTS idx_experiences_location_id;
DROP INDEX IF EXISTS idx_experiences_category_id;
DROP INDEX IF EXISTS idx_experiences_host_id;
DROP INDEX IF EXISTS idx_users_is_host;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;

-- Drop tables in reverse order (respecting foreign key constraints)
DROP TABLE IF EXISTS experience_tags;
DROP TABLE IF EXISTS user_favorites;
DROP TABLE IF EXISTS reviews;
DROP TABLE IF EXISTS payments;
DROP TABLE IF EXISTS bookings;
DROP TABLE IF EXISTS experience_schedules;
DROP TABLE IF EXISTS experience_images;
DROP TABLE IF EXISTS experiences;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS locations;
DROP TABLE IF EXISTS users;