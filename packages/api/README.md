# BuildFlow API

High-performance Rust backend built with Axum.

## Tech Stack

- **Axum** - Async web framework (Tokio team)
- **SQLx** - Async SQL with compile-time query checking
- **Utoipa** - OpenAPI/Swagger documentation
- **Tracing** - Structured logging
- **Serde** - Serialization
- **Validator** - Request validation

## Structure

```
api/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Configuration (env vars)
│   ├── schema/              # Request/Response types
│   ├── routes/              # API route handlers
│   │   ├── auth.rs
│   │   ├── projects.rs
│   │   ├── plots.rs
│   │   ├── preferences.rs
│   │   ├── estimates.rs
│   │   ├── leads.rs
│   │   ├── uploads.rs
│   │   ├── admin.rs
│   │   └── materials.rs
│   ├── middleware/           # Auth, rate limiting
│   └── utils/                # Response helpers, error types
├── Cargo.toml
└── migrations/               # SQLx migrations
```

## Setup

```bash
# Install dependencies
cargo build

# Run with environment variables
export DATABASE_URL="postgresql://..."
export SUPABASE_URL="..."
export SUPABASE_ANON_KEY="..."
cargo run
```

## API Documentation

OpenAPI docs available at `/api/docs` when running locally.

## Endpoints

### Auth
- `POST /api/auth/login` - Login (via Supabase)
- `POST /api/auth/verify-otp` - Phone OTP verification
- `GET /api/auth/me` - Get current user profile

### Projects
- `GET /api/projects` - List user projects
- `POST /api/projects` - Create project
- `GET /api/projects/:id` - Get project details
- `PUT /api/projects/:id` - Update project

### Leads
- `POST /api/leads` - Create lead (with anti-abuse)
- `GET /api/leads` - List leads (admin)

### Materials
- `GET /api/materials` - Browse catalog
- `GET /api/materials/:category` - Get category

### Admin
- `GET /api/admin/projects` - List all projects
- `GET /api/admin/analytics` - Dashboard stats
