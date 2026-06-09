# BuildFlow API

FastAPI backend for BuildFlow construction platform.

## Structure

```
api/
├── app/
│   ├── main.py          # FastAPI app entry
│   ├── config.py        # Configuration
│   ├── database.py      # DB connection
│   ├── routers/         # API route handlers
│   │   ├── auth.py
│   │   ├── projects.py
│   │   ├── plots.py
│   │   ├── estimates.py
│   │   ├── leads.py
│   │   ├── uploads.py
│   │   └── admin.py
│   ├── models/          # SQLAlchemy models
│   └── schemas/         # Pydantic schemas
└── requirements.txt
```

## Setup

```bash
pip install -r requirements.txt
uvicorn app.main:app --reload
```

## Endpoints

- `GET /api/health` - Health check
- `POST /api/auth/login` - User login
- `GET/POST/PUT /api/projects` - Project management
- `GET/POST /api/materials` - Material catalog
