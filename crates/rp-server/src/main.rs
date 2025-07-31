use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use rp_server::{
    config::Config,
    handlers::{
        create_entity, delete_entity, get_entity, health_handler, list_entities, update_entity,
        websocket::websocket_handler,
        branch_theory, get_theory_evidence, get_theory_compliance_status,
        get_person_timeline, get_person_relationships, merge_persons,
        list_workspace_members, invite_workspace_member, remove_workspace_member,
    },
    middleware::{auth_middleware, request_id_middleware},
    notify::start_notification_listener,
    state::AppState,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Load environment variables
    dotenv::dotenv().ok();

    // Load configuration
    let config = Config::from_env();
    info!("Starting ResearchProcess-GPS API server");

    // Initialize app state
    let state = Arc::new(AppState::new(&config.database.url).await?);
    
    // Start PostgreSQL notification listener
    start_notification_listener(state.pg_pool.clone(), state.event_tx.clone()).await?;

    // Build our application with routes
    let app = create_router(state);

    // Set up the server address
    let addr = config.server_addr();
    info!("Server listening on {}", addr);

    // Create the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    // Create the API v1 routes
    let api_v1 = Router::new()
        // Generic entity endpoints
        .route("/entities", post(create_entity).get(list_entities))
        .route(
            "/entities/:id",
            get(get_entity).put(update_entity).delete(delete_entity),
        )
        // Theory-specific endpoints
        .route("/theories/:id/branch", post(branch_theory))
        .route("/theories/:id/evidence", get(get_theory_evidence))
        .route("/theories/:id/compliance-status", get(get_theory_compliance_status))
        // Person-specific endpoints
        .route("/persons/:id/timeline", get(get_person_timeline))
        .route("/persons/:id/relationships", get(get_person_relationships))
        .route("/persons/:id/merge", post(merge_persons))
        // Workspace-specific endpoints
        .route("/workspaces/:id/members", get(list_workspace_members))
        .route("/workspaces/:id/invite", post(invite_workspace_member))
        .route("/workspaces/:id/members/:member_id", axum::routing::delete(remove_workspace_member))
        // WebSocket endpoint
        .route("/ws", get(websocket_handler))
        // Add auth middleware to all API routes
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        // Health check endpoints (no auth required)
        .route("/health", get(health_handler))
        .route("/api/v1/health", get(health_handler))
        // Mount API v1 routes
        .nest("/api/v1", api_v1)
        // Global middleware
        .layer(middleware::from_fn(request_id_middleware))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}