//! Database migrations for meta-model

use sqlx::PgPool;
use crate::PostgresResult;

/// Run all migrations
pub async fn run_migrations(pool: &PgPool) -> PostgresResult<()> {
    // Create tables for Layer 1
    create_entities_table(pool).await?;
    create_relationships_table(pool).await?;

    // Create tables for Layer 2
    create_processes_table(pool).await?;
    create_products_table(pool).await?;

    // Create tables for Layer 3
    create_workspaces_table(pool).await?;

    // Create indexes
    create_indexes(pool).await?;

    Ok(())
}

async fn create_entities_table(pool: &PgPool) -> PostgresResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS entities (
            id UUID PRIMARY KEY,
            entity_type TEXT NOT NULL,
            state TEXT NOT NULL,
            data JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_relationships_table(pool: &PgPool) -> PostgresResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS relationships (
            id UUID PRIMARY KEY,
            relationship_type TEXT NOT NULL,
            participant_ids UUID[] NOT NULL,
            data JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_processes_table(pool: &PgPool) -> PostgresResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS processes (
            id UUID PRIMARY KEY,
            process_type TEXT NOT NULL,
            state TEXT NOT NULL,
            data JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_products_table(pool: &PgPool) -> PostgresResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY,
            product_type TEXT NOT NULL,
            state TEXT NOT NULL,
            data JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_workspaces_table(pool: &PgPool) -> PostgresResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workspaces (
            id UUID PRIMARY KEY,
            workspace_type TEXT NOT NULL,
            state TEXT NOT NULL,
            data JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_indexes(pool: &PgPool) -> PostgresResult<()> {
    // Entity indexes
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_entities_type ON entities(entity_type)"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_entities_state ON entities(state)"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_entities_created ON entities(created_at)"
    ).execute(pool).await?;

    // JSONB indexes for common queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_entities_data ON entities USING GIN (data)"
    ).execute(pool).await?;

    // Relationship indexes
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_relationships_type ON relationships(relationship_type)"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_relationships_participants ON relationships USING GIN (participant_ids)"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_relationships_data ON relationships USING GIN (data)"
    ).execute(pool).await?;

    // Process indexes
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_processes_type ON processes(process_type)"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_processes_state ON processes(state)"
    ).execute(pool).await?;

    // Product indexes
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_products_type ON products(product_type)"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_products_state ON products(state)"
    ).execute(pool).await?;

    // Workspace indexes
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_workspaces_type ON workspaces(workspace_type)"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_workspaces_state ON workspaces(state)"
    ).execute(pool).await?;

    Ok(())
}