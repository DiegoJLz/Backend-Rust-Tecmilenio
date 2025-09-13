-- Migration: Initial Schema for Marketplace de Experiencias Locales
-- Created: 2024-01-01

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    avatar_url TEXT,
    is_host BOOLEAN DEFAULT FALSE,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Locations table
CREATE TABLE IF NOT EXISTS locations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    country VARCHAR(100) NOT NULL,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    icon_url TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Experiences table
CREATE TABLE IF NOT EXISTS experiences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    host_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES categories(id) ON DELETE RESTRICT,
    location_id UUID NOT NULL REFERENCES locations(id) ON DELETE RESTRICT,
    price_per_person DECIMAL(10, 2) NOT NULL CHECK (price_per_person >= 0),
    max_participants INTEGER NOT NULL CHECK (max_participants > 0),
    min_participants INTEGER NOT NULL DEFAULT 1 CHECK (min_participants > 0),
    duration_hours DECIMAL(4, 2) NOT NULL CHECK (duration_hours > 0),
    difficulty_level INTEGER CHECK (difficulty_level BETWEEN 1 AND 5),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Experience images table
CREATE TABLE IF NOT EXISTS experience_images (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL REFERENCES experiences(id) ON DELETE CASCADE,
    image_url TEXT NOT NULL,
    is_primary BOOLEAN DEFAULT FALSE,
    display_order INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Experience schedules table
CREATE TABLE IF NOT EXISTS experience_schedules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL REFERENCES experiences(id) ON DELETE CASCADE,
    start_datetime TIMESTAMP WITH TIME ZONE NOT NULL,
    end_datetime TIMESTAMP WITH TIME ZONE NOT NULL,
    max_available_spots INTEGER NOT NULL CHECK (max_available_spots > 0),
    current_bookings INTEGER DEFAULT 0 CHECK (current_bookings >= 0),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT valid_datetime_range CHECK (end_datetime > start_datetime),
    CONSTRAINT available_spots CHECK (current_bookings <= max_available_spots)
);

-- Bookings table
CREATE TABLE IF NOT EXISTS bookings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    booking_reference VARCHAR(20) UNIQUE NOT NULL,
    experience_id UUID NOT NULL REFERENCES experiences(id) ON DELETE RESTRICT,
    schedule_id UUID NOT NULL REFERENCES experience_schedules(id) ON DELETE RESTRICT,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    number_of_participants INTEGER NOT NULL CHECK (number_of_participants > 0),
    total_price DECIMAL(10, 2) NOT NULL CHECK (total_price >= 0),
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'cancelled', 'completed', 'refunded')),
    special_requests TEXT,
    booking_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Payments table
CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    booking_id UUID NOT NULL REFERENCES bookings(id) ON DELETE CASCADE,
    payment_method VARCHAR(50) NOT NULL,
    amount DECIMAL(10, 2) NOT NULL CHECK (amount >= 0),
    currency VARCHAR(3) DEFAULT 'USD',
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'refunded')),
    transaction_id VARCHAR(255),
    payment_gateway VARCHAR(50),
    processed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Reviews table
CREATE TABLE IF NOT EXISTS reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL REFERENCES experiences(id) ON DELETE CASCADE,
    booking_id UUID NOT NULL REFERENCES bookings(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating INTEGER NOT NULL CHECK (rating BETWEEN 1 AND 5),
    title VARCHAR(200),
    comment TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_user_experience_review UNIQUE (user_id, experience_id)
);

-- User favorites table
CREATE TABLE IF NOT EXISTS user_favorites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    experience_id UUID NOT NULL REFERENCES experiences(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_user_experience_favorite UNIQUE (user_id, experience_id)
);

-- Experience tags table
CREATE TABLE IF NOT EXISTS experience_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL REFERENCES experiences(id) ON DELETE CASCADE,
    tag_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_experience_tag UNIQUE (experience_id, tag_name)
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_is_host ON users(is_host);

CREATE INDEX IF NOT EXISTS idx_experiences_host_id ON experiences(host_id);
CREATE INDEX IF NOT EXISTS idx_experiences_category_id ON experiences(category_id);
CREATE INDEX IF NOT EXISTS idx_experiences_location_id ON experiences(location_id);
CREATE INDEX IF NOT EXISTS idx_experiences_price ON experiences(price_per_person);
CREATE INDEX IF NOT EXISTS idx_experiences_is_active ON experiences(is_active);

CREATE INDEX IF NOT EXISTS idx_experience_schedules_experience_id ON experience_schedules(experience_id);
CREATE INDEX IF NOT EXISTS idx_experience_schedules_start_datetime ON experience_schedules(start_datetime);
CREATE INDEX IF NOT EXISTS idx_experience_schedules_is_active ON experience_schedules(is_active);

CREATE INDEX IF NOT EXISTS idx_bookings_user_id ON bookings(user_id);
CREATE INDEX IF NOT EXISTS idx_bookings_experience_id ON bookings(experience_id);
CREATE INDEX IF NOT EXISTS idx_bookings_schedule_id ON bookings(schedule_id);
CREATE INDEX IF NOT EXISTS idx_bookings_status ON bookings(status);
CREATE INDEX IF NOT EXISTS idx_bookings_booking_date ON bookings(booking_date);

CREATE INDEX IF NOT EXISTS idx_payments_booking_id ON payments(booking_id);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);

CREATE INDEX IF NOT EXISTS idx_reviews_experience_id ON reviews(experience_id);
CREATE INDEX IF NOT EXISTS idx_reviews_user_id ON reviews(user_id);
CREATE INDEX IF NOT EXISTS idx_reviews_rating ON reviews(rating);

-- Triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_locations_updated_at BEFORE UPDATE ON locations FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_experiences_updated_at BEFORE UPDATE ON experiences FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_experience_schedules_updated_at BEFORE UPDATE ON experience_schedules FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_bookings_updated_at BEFORE UPDATE ON bookings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_payments_updated_at BEFORE UPDATE ON payments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_reviews_updated_at BEFORE UPDATE ON reviews FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trigger to update booking count in experience_schedules
CREATE OR REPLACE FUNCTION update_booking_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE experience_schedules
        SET current_bookings = current_bookings + NEW.number_of_participants
        WHERE id = NEW.schedule_id;
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        -- Handle booking updates (e.g., status changes)
        IF OLD.status != NEW.status AND NEW.status = 'cancelled' THEN
            UPDATE experience_schedules
            SET current_bookings = current_bookings - OLD.number_of_participants
            WHERE id = OLD.schedule_id;
        ELSIF OLD.status = 'cancelled' AND NEW.status != 'cancelled' THEN
            UPDATE experience_schedules
            SET current_bookings = current_bookings + NEW.number_of_participants
            WHERE id = NEW.schedule_id;
        END IF;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE experience_schedules
        SET current_bookings = current_bookings - OLD.number_of_participants
        WHERE id = OLD.schedule_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER booking_count_trigger
    AFTER INSERT OR UPDATE OR DELETE ON bookings
    FOR EACH ROW EXECUTE FUNCTION update_booking_count();

-- Function to generate booking reference
CREATE OR REPLACE FUNCTION generate_booking_reference()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.booking_reference IS NULL OR NEW.booking_reference = '' THEN
        NEW.booking_reference = 'BK-' || TO_CHAR(NOW(), 'YYYYMMDD') || '-' || SUBSTRING(NEW.id::text, 1, 8);
    END IF;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER generate_booking_reference_trigger
    BEFORE INSERT ON bookings
    FOR EACH ROW EXECUTE FUNCTION generate_booking_reference();

-- Insert default categories
INSERT INTO categories (id, name, description, icon_url) VALUES
    (uuid_generate_v4(), 'Cultura y Arte', 'Experiencias relacionadas con arte, música, teatro y cultura local', '/icons/culture.png'),
    (uuid_generate_v4(), 'Gastronomía', 'Tours culinarios, clases de cocina y experiencias gastronómicas', '/icons/food.png'),
    (uuid_generate_v4(), 'Aventura', 'Actividades al aire libre, deportes extremos y aventuras', '/icons/adventure.png'),
    (uuid_generate_v4(), 'Bienestar', 'Yoga, spa, meditación y experiencias de relajación', '/icons/wellness.png'),
    (uuid_generate_v4(), 'Educación', 'Talleres, clases y experiencias de aprendizaje', '/icons/education.png'),
    (uuid_generate_v4(), 'Entretenimiento', 'Música en vivo, shows, fiestas y eventos', '/icons/entertainment.png'),
    (uuid_generate_v4(), 'Tecnología', 'Workshops tecnológicos, gaming y experiencias digitales', '/icons/technology.png'),
    (uuid_generate_v4(), 'Naturaleza', 'Ecoturismo, senderismo y experiencias en la naturaleza', '/icons/nature.png')
ON CONFLICT (name) DO NOTHING;