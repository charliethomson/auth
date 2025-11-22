# Auth Service

Authentication and authorization service with JWT-based auth and fine-grained permission management.

## Features

- JWT authentication with HMAC-SHA256 signing
- Grant-based permission system (e.g., `dev.thmsn.auth.user.create`)
- User and application management
- OpenAPI documentation with Scalar UI
- Argon2 password hashing

## Architecture

Rust workspace with:
- **controller/rest** - REST API server (Poem + OpenAPI)
- **data** - Database layer (SeaORM)
- **migration** - Database migrations
- **seed** - Database seeding

## Quick Start

```bash
# stops and starts db, builds entities based on the migrations, runs migrations, seeds db
./scripts/fresh_db_and_seed.sh

# My start command:
RUST_LOG=debug cargo r --release --bin rest -- --port 8080 --address 0.0.0.0 --hostname localhost --environment development --database-url mysql://root:thedebugpassword@maria.auth.orb.local:3306/auth
```

[orbstack btw](https://orbstack.dev/)

## Configuration

Required environment variables:

```bash
DATABASE_URL=postgresql://user:password@localhost/auth
SIGNING_KEY=your-secure-key
HOSTNAME=localhost
PORT=8080
ENVIRONMENT=development
```

## API Documentation

Three APIs with interactive Scalar UI docs:

- **Auth API** - `/docs/` - Public authentication endpoints
- **Management API** - `/docs/manage` - Admin endpoints for users, applications, and grants
- **Debug API** - `/docs/debug` - Development utilities

## How Grants Work

Grants are permission identifiers using reverse-domain naming:

```
dev.thmsn.auth.user.create
dev.thmsn.auth.application.get
dev.thmsn.auth.grant.create
```

Grants are assigned to users and embedded in JWTs. Endpoints check for required grants before allowing access.

## Security

- Passwords hashed with Argon2
- JWTs signed with HMAC-SHA256
- Change `SIGNING_KEY` in production
- Set `ENVIRONMENT=production` to disable debug endpoints (TODO:)
