# PostgreSQL Integration Test Setup

## Overview

The integration tests for the Person repository require a PostgreSQL database with the schema migrations applied.

## Database Requirements

- PostgreSQL 14 or higher
- Required extensions (automatically created by migrations):
  - `uuid-ossp` - UUID generation
  - `btree_gin` - GIN indexing for full-text search

## Test Configuration

Tests connect to the database using environment variables or defaults:

```bash
# Environment variables (optional)
export TEST_DB_HOST=192.168.10.90  # Default: 192.168.10.90
export TEST_DB_PORT=5432           # Default: 5432
export TEST_DB_NAME=researchprocess_gps  # Default: researchprocess_gps
export TEST_DB_USER=researchprocess_gps  # Default: researchprocess_gps
export TEST_DB_PASSWORD=researchprocess_gps  # Default: researchprocess_gps
```

## Setting Up Test Database

### Option 1: Use Existing Database

If you have a PostgreSQL instance at 192.168.10.90, ensure migrations are applied:

```bash
# Apply all migrations
psql postgresql://researchprocess_gps:researchprocess_gps@192.168.10.90:5432/researchprocess_gps \
  -f schemas/postgres/migrations/001_initial_schema.sql
psql postgresql://researchprocess_gps:researchprocess_gps@192.168.10.90:5432/researchprocess_gps \
  -f schemas/postgres/migrations/002_extensions.sql
# ... apply all migrations in order ...
psql postgresql://researchprocess_gps:researchprocess_gps@192.168.10.90:5432/researchprocess_gps \
  -f schemas/postgres/migrations/010_person_authority_control.sql
```

### Option 2: Local PostgreSQL with Docker

```bash
# Start PostgreSQL in Docker
docker run --name rpgps-test-db \
  -e POSTGRES_DB=researchprocess_gps \
  -e POSTGRES_USER=researchprocess_gps \
  -e POSTGRES_PASSWORD=researchprocess_gps \
  -p 5432:5432 \
  -d postgres:16

# Wait for database to be ready
sleep 5

# Apply migrations
for migration in schemas/postgres/migrations/*.sql; do
  psql postgresql://researchprocess_gps:researchprocess_gps@localhost:5432/researchprocess_gps \
    -f "$migration"
done

# Update test configuration
export TEST_DB_HOST=localhost
```

### Option 3: Docker Compose (Recommended for CI/CD)

Create `docker-compose.test.yml`:

```yaml
version: '3.8'
services:
  test-db:
    image: postgres:16
    environment:
      POSTGRES_DB: researchprocess_gps
      POSTGRES_USER: researchprocess_gps
      POSTGRES_PASSWORD: researchprocess_gps
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U researchprocess_gps"]
      interval: 5s
      timeout: 5s
      retries: 5
```

```bash
# Start test database
docker-compose -f docker-compose.test.yml up -d

# Wait for health check
docker-compose -f docker-compose.test.yml ps

# Apply migrations
for migration in schemas/postgres/migrations/*.sql; do
  psql postgresql://researchprocess_gps:researchprocess_gps@localhost:5432/researchprocess_gps \
    -f "$migration"
done

# Update test configuration
export TEST_DB_HOST=localhost
```

## Running Tests

Once the database is set up and migrations are applied:

```bash
# Run all Person repository tests
cargo test -p rp-storage-postgres --test person_repository_test -- --nocapture

# Run specific test
cargo test -p rp-storage-postgres person_repository_test::test_person_create_and_get -- --nocapture

# Run all integration tests
cargo test -p rp-storage-postgres --tests
```

## Test Coverage

The Person repository integration tests cover:

1. **Person CRUD Operations**
   - `test_person_create_and_get` - Create and retrieve persons
   - `test_person_update` - Update person fields
   - `test_person_archive_and_unarchive` - Soft delete pattern
   - `test_person_list` - Pagination and filtering

2. **VariantName Operations**
   - `test_variant_name_operations` - Complete CRUD for name variants

3. **PersonRelationship Operations**
   - `test_relationship_operations` - Relationships with reciprocals

4. **SourcePerson Operations**
   - `test_source_person_operations` - Source attribution

5. **PersonMerge Operations**
   - `test_merge_operations` - Merge and reversal workflow

6. **Search Operations**
   - `test_search_operations` - Full-text search including variants

7. **Aggregate Operations**
   - `test_aggregate_operations` - Get person with all related data

8. **Complete Workflow**
   - `test_complete_person_workflow` - End-to-end scenario

## Troubleshooting

### Connection Timeout

If tests fail with `PoolTimedOut`:
- Verify database is running and accessible
- Check firewall rules
- Verify credentials
- Ensure migrations are applied

### Schema Errors

If tests fail with schema-related errors:
- Verify all migrations have been applied in order
- Check that migration 010 (person_authority_control) has been applied
- Verify custom types and enums exist

### Permission Errors

If tests fail with permission errors:
- Grant necessary permissions:
  ```sql
  GRANT ALL ON DATABASE researchprocess_gps TO researchprocess_gps;
  GRANT ALL ON ALL TABLES IN SCHEMA public TO researchprocess_gps;
  GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO researchprocess_gps;
  ```

## CI/CD Integration

For CI/CD pipelines, use Docker Compose approach:

```yaml
# .github/workflows/test.yml example
services:
  postgres:
    image: postgres:16
    env:
      POSTGRES_DB: researchprocess_gps
      POSTGRES_USER: researchprocess_gps
      POSTGRES_PASSWORD: researchprocess_gps
    options: >-
      --health-cmd pg_isready
      --health-interval 10s
      --health-timeout 5s
      --health-retries 5

steps:
  - name: Apply migrations
    run: |
      for migration in schemas/postgres/migrations/*.sql; do
        psql postgresql://researchprocess_gps:researchprocess_gps@postgres:5432/researchprocess_gps \
          -f "$migration"
      done

  - name: Run tests
    env:
      TEST_DB_HOST: postgres
    run: cargo test -p rp-storage-postgres --tests
```

## Required Migrations

The Person repository tests require these schema migrations to be applied:

1. `001_initial_schema.sql` - Base tables and types
2. `002_extensions.sql` - PostgreSQL extensions
3. `010_person_authority_control.sql` - Person entity schema with:
   - `persons` table
   - `person_variant_names` table
   - `person_relationships` table
   - `person_source_links` table
   - `person_merges` table
   - Custom enum types (sex, variant_name_type, relationship_type, etc.)
   - Indexes and triggers
   - Helper functions

## Notes

- Tests use `Uuid::now_v7()` for time-based UUIDs
- All tests clean up after themselves using archive/delete operations
- Tests run in parallel by default - use `-- --test-threads=1` for sequential execution
- Full-text search tests require GIN indexes to be created by migrations
