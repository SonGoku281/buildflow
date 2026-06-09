# BuildFlow

End-to-end construction operations platform for residential builds in India.

## Platform Overview

BuildFlow is a dual-arm platform:
1. **Digital Planning Hub (SaaS)** - 2D/3D layout planning, material selection, cost forecasting, expense tracking
2. **Execution Engine (Turnkey Services)** - Physical construction service with Six Sigma quality control

## Tech Stack

- **Frontend:** SvelteKit (PWA)
- **Backend:** Rust + Axum (async, type-safe)
- **Database:** PostgreSQL (via Supabase)
- **ORM:** SQLx (async, compile-time checked)
- **Auth:** Supabase Auth (Email + OTP)
- **Storage:** Supabase Storage
- **Payments:** Pluggable (Razorpay default)
- **Deployment:** Railway/Render

## Project Structure

```
buildflow/
├── apps/
│   ├── web/                 # Client-facing PWA
│   └── admin/               # Admin dashboard
├── packages/
│   ├── api/                 # Rust + Axum backend
│   ├── shared/              # Shared types/utils
│   └── payment/             # Pluggable payment adapters
├── data/
│   └── materials.json       # Material catalog seed data
├── infra/
│   └── supabase/            # DB migrations, seed scripts
├── scripts/                 # Dev utilities
└── docs/                    # Architecture docs
```

## Getting Started

### Prerequisites
- Rust 1.75+ (rustup)
- Node.js 18+
- Supabase account
- PostgreSQL (via Supabase)

### Backend Setup (Rust)
```bash
cd packages/api
cargo build
cargo run
```

### Frontend Setup
```bash
cd apps/web
npm install
npm run dev
```

## MVP Timeline (8 Weeks)

| Week | Focus | Deliverables |
|------|-------|-------------|
| 1 | Setup | GitHub, repo, scaffolding |
| 2 | Auth + Foundation | Supabase, DB schema, auth |
| 3 | Core APIs | Project CRUD, file upload, leads |
| 4 | Client Frontend | PWA, auth flows, dashboard |
| 5 | Admin Panel | Dashboard, estimate creator |
| 6 | Payment + PWA | Razorpay, OTP, install prompt |
| 7 | Analytics + Polish | Event tracking, performance |
| 8 | Beta Deploy | Production deploy, docs |

## API Documentation

OpenAPI docs: `/api/docs` (running locally)

## License

MIT
