# CRM Backend API

REST API server for CRM synchronization.

## Development

1. Copy `.env.example` to `.env` and configure database URL
2. Run with cargo:
```bash
cargo run
```

## Docker Deployment

From the project root:

```bash
docker-compose up -d
```

This will start:
- PostgreSQL database on port 5432
- Backend API on port 8080

## API Endpoints

- `GET /api/health` - Health check
- `POST /api/sync/customer` - Sync customer data
- `POST /api/sync/document` - Sync document data
- `GET /api/customers` - Get all customers
- `GET /api/documents` - Get all documents

