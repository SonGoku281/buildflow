# BuildFlow Setup Guide

## Week 2: Supabase Setup + Auth Foundation

### 1. Create Supabase Project

1. Go to https://supabase.com/dashboard
2. Click "New Project"
3. Fill in:
   - **Name:** `buildflow`
   - **Database Password:** (save this securely!)
   - **Region:** Choose closest to India (Mumbai or Singapore)
   - **Environment:** Development

### 2. Get Your Credentials

After creation, go to **Settings → API** and copy:
- `Project URL` → `SUPABASE_URL`
- `anon/public` key → `SUPABASE_ANON_KEY`
- `service_role` key (for migrations only)

Go to **Settings → Database** and copy:
- Connection string (Pooler) → `DATABASE_URL`
- Or use direct: `postgresql://postgres.YOUR_PROJECT_REF:YOUR_PASSWORD@db.YOUR_PROJECT_REF.supabase.co:5432/postgres`

Go to **Settings → JWT** to get:
- `JWT Secret` → `SUPABASE_JWT_SECRET`

### 3. Run Migrations

```bash
# Copy .env.example to .env
cp .env.example .env

# Edit .env with your Supabase credentials
# DATABASE_URL, SUPABASE_URL, SUPABASE_ANON_KEY, SUPABASE_JWT_SECRET

# Run migration 001 (schema)
cd infra/supabase/migrations
psql "$DATABASE_URL" -f 001_initial_schema.sql

# Run migration 002 (seed data)
psql "$DATABASE_URL" -f 002_seed_materials.sql
```

Or using Supabase CLI:
```bash
# Install Supabase CLI
npm install -g supabase

# Link your project
supabase link --project-ref YOUR_PROJECT_REF

# Apply migrations
supabase db push
```

### 4. Verify Setup

```bash
# Check tables
psql "$DATABASE_URL" -c "\dt"

# Check profiles table
psql "$DATABASE_URL" -c "SELECT * FROM profiles LIMIT 5;"

# Check materials
psql "$DATABASE_URL" -c "SELECT category, COUNT(*) FROM materials GROUP BY category;"
```

### 5. Create Admin User

```sql
-- 1. Sign up a user via Supabase Auth (email/password)
-- 2. Then promote to admin:
UPDATE profiles SET role = 'admin' WHERE email = 'your-email@example.com';
```

### 6. Start Backend

```bash
cd packages/api
cargo run
```

### 7. Test Endpoints

```bash
# Health check
curl http://localhost:8000/api/health

# Materials
curl http://localhost:8000/api/materials
curl http://localhost:8000/api/materials/cement

# Try auth (will need Supabase integration)
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password"}'
```

## Troubleshooting

### "Connection refused"
- Check PostgreSQL is running: `pg_isready -h localhost`
- Verify DATABASE_URL in .env

### "relation does not exist"
- Run migrations: `psql "$DATABASE_URL" -f 001_initial_schema.sql`

### "permission denied"
- Check RLS policies are enabled
- Verify user has correct role in profiles table
