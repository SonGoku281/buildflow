# BuildFlow

End-to-end construction operations platform for residential builds in India.

## Platform Overview

BuildFlow is a dual-arm platform:
1. **Digital Planning Hub (SaaS)** - 2D/3D layout planning, material selection, cost forecasting, expense tracking
2. **Execution Engine (Turnkey Services)** - Physical construction service with Six Sigma quality control

## Tech Stack

- **Frontend:** SvelteKit (PWA)
- **Backend:** Python FastAPI
- **Database:** PostgreSQL (via Supabase)
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
│   ├── api/                 # FastAPI backend
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
- Node.js 18+
- Python 3.11+
- Supabase account

### Setup
1. Clone the repo
2. Set up Supabase project
3. Install dependencies: `npm install` (or per-app)
4. Run migrations: `supabase db reset`
5. Start dev servers

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

## License

MIT
