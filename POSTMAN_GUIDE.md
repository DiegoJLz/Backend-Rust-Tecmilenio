# 📮 Guía de Endpoints - Postman

**Base URL:** `http://localhost:8080/api/v1`

---

## 🔐 Autenticación (`/auth`)

### 1. Registrar Usuario
**POST** `/auth/register`

**Body (JSON):**
```json
{
  "email": "usuario@ejemplo.com",
  "username": "usuario123",
  "first_name": "Juan",
  "last_name": "Pérez",
  "password": "Password123!",
  "phone": "+521234567890"
}
```

**Respuesta exitosa (201):**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "email": "usuario@ejemplo.com",
      "username": "usuario123",
      ...
    },
    "token": "jwt_token_here"
  },
  "message": "User registered successfully"
}
```

---

### 2. Iniciar Sesión
**POST** `/auth/login`

**Body (JSON):**
```json
{
  "email": "usuario@ejemplo.com",
  "password": "Password123!"
}
```

**Respuesta exitosa (200):**
```json
{
  "success": true,
  "data": {
    "user": {...},
    "access_token": "jwt_token",
    "refresh_token": "refresh_token",
    "session_token": "session_token"
  }
}
```

---

### 3. Verificar Email
**GET** `/auth/verify-email?token=JWT_TOKEN_AQUI`

**Query Params:**
- `token` (string, requerido): Token JWT recibido por email

**Respuesta exitosa (200):**
```json
{
  "success": true,
  "message": "Email verified successfully"
}
```

---

### 4. Olvidé mi Contraseña
**POST** `/auth/forgot-password`

**Body (JSON):**
```json
{
  "email": "usuario@ejemplo.com"
}
```

**Respuesta exitosa (200):**
```json
{
  "message": "Password reset email sent"
}
```

---

### 5. Resetear Contraseña
**POST** `/auth/reset-password?token=JWT_TOKEN_AQUI`

**Query Params:**
- `token` (string, requerido): Token JWT del email de reset

**Body (JSON):**
```json
{
  "new_password": "NuevaPassword123!",
  "confirm_password": "NuevaPassword123!"
}
```

**Respuesta exitosa (200):**
```json
{
  "message": "Password reset successfully",
  "user": {...}
}
```

---

### 6. Cerrar Sesión
**POST** `/auth/logout`

**Body (JSON):**
```json
{
  "session_token": "session_token_aqui"
}
```

**Respuesta exitosa (200):**
```json
{
  "message": "Logged out successfully"
}
```

---

### 7. Reenviar Verificación de Email
**POST** `/auth/resend-email-verification`

**Body (JSON):**
```json
{
  "email": "usuario@ejemplo.com"
}
```

**Respuesta exitosa (200):**
```json
{
  "message": "Verification email sent"
}
```

---

## 🏠 Landing Page (`/landing`)

### 8. Obtener Datos de Landing
**GET** `/landing`

**Respuesta exitosa (200):**
```json
{
  "success": true,
  "data": {
    "highlights": [...],
    "featured_experiences": [...],
    "hero_categories": [...],
    "curated_collections": [...],
    "promotions": [...],
    "testimonials": [...]
  },
  "message": "Landing data retrieved"
}
```

---

## 📅 Bookings (`/bookings`)

### 9. Cotizar Reserva
**POST** `/bookings/quote`

**Body (JSON):**
```json
{
  "experience_id": "uuid-de-experiencia",
  "schedule_id": "uuid-de-horario",
  "number_of_participants": 2,
  "promotion_id": "uuid-de-promocion-opcional"
}
```

**Respuesta exitosa (200):**
```json
{
  "experience_id": "uuid",
  "schedule_id": "uuid",
  "number_of_participants": 2,
  "base_price_per_person": 500.00,
  "subtotal": 1000.00,
  "discount": 200.00,
  "total": 800.00,
  "promotion_id": "uuid-opcional"
}
```

---

### 10. Crear Reserva
**POST** `/bookings`

**Body (JSON):**
```json
{
  "experience_id": "uuid-de-experiencia",
  "schedule_id": "uuid-de-horario",
  "user_id": "uuid-de-usuario",
  "number_of_participants": 2,
  "special_requests": "Necesito acceso para silla de ruedas",
  "promotion_id": "uuid-de-promocion-opcional"
}
```

**Respuesta exitosa (201):**
```json
{
  "id": "uuid",
  "booking_reference": "BK-20241201-abc123",
  "experience_id": "uuid",
  "schedule_id": "uuid",
  "user_id": "uuid",
  "number_of_participants": 2,
  "total_price": 800.00,
  "status": "pending",
  "special_requests": "Necesito acceso para silla de ruedas",
  "booking_date": "2024-12-01T10:00:00Z"
}
```

---

### 11. Obtener Detalle de Reserva
**GET** `/bookings/{id}`

**Path Params:**
- `id` (string, requerido): UUID de la reserva

**Respuesta exitosa (200):**
```json
{
  "id": "uuid",
  "booking_reference": "BK-20241201-abc123",
  "experience_id": "uuid",
  "schedule_id": "uuid",
  "user_id": "uuid",
  "number_of_participants": 2,
  "total_price": 800.00,
  "status": "pending",
  "special_requests": "...",
  "booking_date": "2024-12-01T10:00:00Z"
}
```

---

### 12. Listar Reservas de Usuario
**GET** `/users/{user_id}/bookings?status=pending`

**Path Params:**
- `user_id` (string, requerido): UUID del usuario

**Query Params (opcionales):**
- `status` (string): Filtrar por status (`pending`, `confirmed`, `cancelled`, `completed`, `refunded`)

**Ejemplo sin filtro:**
```
GET /api/v1/users/uuid-del-usuario/bookings
```

**Ejemplo con filtro:**
```
GET /api/v1/users/uuid-del-usuario/bookings?status=confirmed
```

**Respuesta exitosa (200):**
```json
{
  "bookings": [
    {
      "id": "uuid",
      "booking_reference": "BK-20241201-abc123",
      ...
    }
  ],
  "total": 5,
  "page": 1,
  "per_page": 5
}
```

---

## 🎁 Promociones (`/promotions`)

### 13. Listar Promociones
**GET** `/promotions?only_active=true`

**Query Params (opcionales):**
- `only_active` (boolean, default: `true`): Solo promociones activas
- `date` (string, opcional): Fecha ISO 8601 para validar vigencia

**Ejemplo:**
```
GET /api/v1/promotions?only_active=true
```

**Respuesta exitosa (200):**
```json
{
  "promotions": [
    {
      "id": "uuid",
      "name": "Descuento 30%",
      "headline": "Hasta 30% de descuento",
      "description": "...",
      "discount_type": "percentage",
      "discount_value": 30.0,
      "badge_label": "¡Nuevo!",
      "cta_label": "Ver más",
      "cta_url": "/promociones/30-off",
      "image_url": "https://...",
      "terms": "Válido hasta...",
      "is_stackable": false,
      "is_active": true
    }
  ],
  "total": 3
}
```

---

### 14. Obtener Detalle de Promoción
**GET** `/promotions/{id}`

**Path Params:**
- `id` (string, requerido): UUID de la promoción

**Respuesta exitosa (200):**
```json
{
  "id": "uuid",
  "name": "Descuento 30%",
  "headline": "Hasta 30% de descuento",
  "description": "...",
  "discount_type": "percentage",
  "discount_value": 30.0,
  "start_date": "2024-12-01T00:00:00Z",
  "end_date": "2024-12-31T23:59:59Z",
  "terms": "...",
  "image_url": "https://...",
  "badge_label": "¡Nuevo!",
  "cta_label": "Ver más",
  "cta_url": "/promociones/30-off",
  "is_stackable": false,
  "is_active": true
}
```

---

## 📝 Notas Importantes

### Headers Recomendados
Para todas las peticiones, asegúrate de incluir:
```
Content-Type: application/json
```

### Códigos de Estado HTTP
- `200 OK`: Operación exitosa
- `201 Created`: Recurso creado exitosamente
- `400 Bad Request`: Error en la solicitud (datos inválidos)
- `404 Not Found`: Recurso no encontrado
- `500 Internal Server Error`: Error del servidor

### Formato de UUIDs
Todos los IDs deben ser UUIDs válidos en formato estándar:
```
550e8400-e29b-41d4-a716-446655440000
```

### Formato de Fechas
Las fechas se manejan en formato ISO 8601:
```
2024-12-01T10:00:00Z
```

---

## 🚧 Endpoints Pendientes de Implementar

Según el diseño original, aún faltan estos endpoints:

1. **Experiencias:**
   - `GET /experiences` - Listar experiencias con filtros
   - `GET /experiences/{slug}` - Detalle de experiencia
   - `GET /experiences/{id}/schedules` - Horarios disponibles

2. **Categorías:**
   - `GET /categories` - Listar categorías

3. **Colecciones:**
   - `GET /collections` - Listar colecciones
   - `GET /collections/{slug}` - Detalle de colección

4. **Pagos:**
   - `POST /bookings/{id}/payments` - Crear pago
   - `GET /bookings/{id}/payments` - Historial de pagos

5. **Reviews:**
   - `POST /experiences/{id}/reviews` - Crear review
   - `GET /experiences/{id}/reviews` - Listar reviews

6. **Favoritos:**
   - `POST /experiences/{id}/favorite` - Agregar favorito
   - `DELETE /experiences/{id}/favorite` - Quitar favorito
   - `GET /users/{user_id}/favorites` - Listar favoritos

---

## 🧪 Ejemplo de Flujo Completo en Postman

### 1. Registrar Usuario
```
POST http://localhost:8080/api/v1/auth/register
Body: { "email": "test@test.com", "username": "testuser", ... }
```

### 2. Verificar Email (usar token del email recibido)
```
GET http://localhost:8080/api/v1/auth/verify-email?token=JWT_TOKEN
```

### 3. Iniciar Sesión
```
POST http://localhost:8080/api/v1/auth/login
Body: { "email": "test@test.com", "password": "..." }
```

### 4. Ver Landing Page
```
GET http://localhost:8080/api/v1/landing
```

### 5. Listar Promociones
```
GET http://localhost:8080/api/v1/promotions?only_active=true
```

### 6. Cotizar Reserva
```
POST http://localhost:8080/api/v1/bookings/quote
Body: { "experience_id": "...", "schedule_id": "...", "number_of_participants": 2 }
```

### 7. Crear Reserva
```
POST http://localhost:8080/api/v1/bookings
Body: { "experience_id": "...", "schedule_id": "...", "user_id": "...", ... }
```

### 8. Ver Mis Reservas
```
GET http://localhost:8080/api/v1/users/{user_id}/bookings?status=pending
```

---

¡Listo! Con esta guía puedes probar todos los endpoints implementados en Postman. 🚀
