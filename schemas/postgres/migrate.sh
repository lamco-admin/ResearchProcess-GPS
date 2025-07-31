#!/bin/bash
# ResearchProcess-GPS PostgreSQL Migration Runner

# Configuration
DB_HOST="${DB_HOST:-192.168.10.90}"
DB_PORT="${DB_PORT:-5432}"
DB_NAME="${DB_NAME:-researchprocess_gps}"
DB_USER="${DB_USER:-researchprocess_gps}"
DB_PASSWORD="${DB_PASSWORD:-researchprocess_gps}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Migration directory
MIGRATION_DIR="$(dirname "$0")/migrations"

echo -e "${GREEN}ResearchProcess-GPS PostgreSQL Migration Runner${NC}"
echo "================================================"
echo "Host: $DB_HOST:$DB_PORT"
echo "Database: $DB_NAME"
echo "User: $DB_USER"
echo ""

# Test connection
echo -n "Testing database connection... "
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "SELECT 1;" > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    echo "Could not connect to database. Please check your configuration."
    exit 1
fi

# Create migrations tracking table if it doesn't exist
echo -n "Initializing migration tracking... "
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME <<EOF > /dev/null 2>&1
CREATE TABLE IF NOT EXISTS schema_migrations (
    version VARCHAR(50) PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    checksum VARCHAR(64),
    execution_time INTERVAL,
    applied_by VARCHAR(100) DEFAULT CURRENT_USER
);
EOF
echo -e "${GREEN}OK${NC}"

# Function to calculate file checksum
calculate_checksum() {
    sha256sum "$1" | cut -d' ' -f1
}

# Function to check if migration was applied
is_migration_applied() {
    local version=$1
    local result=$(PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -t -c "SELECT COUNT(*) FROM schema_migrations WHERE version = '$version';")
    [ $result -gt 0 ]
}

# Function to apply a migration
apply_migration() {
    local file=$1
    local version=$(basename "$file" .sql)
    local checksum=$(calculate_checksum "$file")
    
    echo -e "\n${YELLOW}Applying migration: $version${NC}"
    echo "File: $file"
    echo "Checksum: $checksum"
    
    # Start timing
    local start_time=$(date +%s.%N)
    
    # Apply the migration
    PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f "$file"
    local result=$?
    
    # End timing
    local end_time=$(date +%s.%N)
    local execution_time=$(echo "$end_time - $start_time" | bc)
    
    if [ $result -eq 0 ]; then
        # Record successful migration
        PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME <<EOF > /dev/null 2>&1
INSERT INTO schema_migrations (version, checksum, execution_time) 
VALUES ('$version', '$checksum', '$execution_time seconds'::interval);
EOF
        echo -e "${GREEN}✓ Migration applied successfully${NC} (${execution_time}s)"
        return 0
    else
        echo -e "${RED}✗ Migration failed!${NC}"
        return 1
    fi
}

# Main migration logic
echo -e "\n${YELLOW}Checking for pending migrations...${NC}"

# Get all migration files
migration_files=$(find "$MIGRATION_DIR" -name "*.sql" -type f | sort)
pending_count=0
applied_count=0

for file in $migration_files; do
    version=$(basename "$file" .sql)
    
    if is_migration_applied "$version"; then
        echo -e "  ${GREEN}✓${NC} $version (already applied)"
        ((applied_count++))
    else
        echo -e "  ${YELLOW}○${NC} $version (pending)"
        ((pending_count++))
    fi
done

echo -e "\nSummary: ${GREEN}$applied_count applied${NC}, ${YELLOW}$pending_count pending${NC}"

if [ $pending_count -eq 0 ]; then
    echo -e "\n${GREEN}Database is up to date!${NC}"
    exit 0
fi

# Ask for confirmation
echo -e "\n${YELLOW}Do you want to apply $pending_count pending migration(s)? (y/N)${NC}"
read -r response

if [[ ! "$response" =~ ^[Yy]$ ]]; then
    echo "Migration cancelled."
    exit 0
fi

# Apply pending migrations
failed=0
for file in $migration_files; do
    version=$(basename "$file" .sql)
    
    if ! is_migration_applied "$version"; then
        if ! apply_migration "$file"; then
            ((failed++))
            echo -e "${RED}Migration failed. Stopping here to prevent further issues.${NC}"
            break
        fi
    fi
done

# Final summary
echo -e "\n${GREEN}Migration Summary${NC}"
echo "=================="
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -t <<EOF
SELECT 
    version,
    to_char(applied_at, 'YYYY-MM-DD HH24:MI:SS') as applied_at,
    execution_time
FROM schema_migrations
ORDER BY applied_at DESC
LIMIT 10;
EOF

if [ $failed -gt 0 ]; then
    echo -e "\n${RED}Warning: $failed migration(s) failed!${NC}"
    exit 1
else
    echo -e "\n${GREEN}All migrations completed successfully!${NC}"
fi