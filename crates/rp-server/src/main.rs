use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use rp_server::{
    config::Config,
    handlers::{
        entities::{create_entity, delete_entity, get_entity, list_entities, update_entity},
        health::health_handler,
        websocket::websocket_handler,
        theories::{branch_theory, get_theory_evidence, get_theory_compliance_status},
        persons::{get_person_timeline, get_person_relationships, merge_persons},
        workspaces::{list_workspace_members, invite_workspace_member, remove_workspace_member},
        search::search_entities,
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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        rp_server::handlers::health::health_handler,
        rp_server::handlers::entities::list_entities,
        rp_server::handlers::entities::create_entity,
        rp_server::handlers::entities::get_entity,
        rp_server::handlers::entities::update_entity,
        rp_server::handlers::entities::delete_entity,
        rp_server::handlers::search::search_entities,
        rp_server::handlers::theories::branch_theory,
        rp_server::handlers::theories::get_theory_evidence,
        rp_server::handlers::theories::get_theory_compliance_status,
        rp_server::handlers::persons::get_person_timeline,
        rp_server::handlers::persons::get_person_relationships,
        rp_server::handlers::persons::merge_persons,
        rp_server::handlers::workspaces::list_workspace_members,
        rp_server::handlers::workspaces::invite_workspace_member,
        rp_server::handlers::workspaces::remove_workspace_member,
    ),
    components(
        schemas(
            // Protocol types
            rp_protocol::CreateEntityRequest,
            rp_protocol::UpdateEntityRequest,
            rp_protocol::EntityResponse,
            rp_protocol::EntityMetadata,
            rp_protocol::ListResponse<rp_protocol::EntityResponse>,
            rp_protocol::PaginationParams,
            rp_protocol::HealthResponse,
            rp_protocol::HealthStatus,
            // Handler-specific types from theories.rs
            rp_server::handlers::BranchTheoryRequest,
            rp_server::handlers::BranchTheoryResponse,
            rp_server::handlers::ComplianceStatus,
            rp_server::handlers::ComplianceIssue,
            // Handler-specific types from persons.rs
            rp_server::handlers::TimelineEvent,
            rp_server::handlers::PersonTimeline,
            rp_server::handlers::PersonRelationship,
            rp_server::handlers::PersonRelationships,
            rp_server::handlers::MergePersonsRequest,
            rp_server::handlers::MergePersonsResponse,
            // Handler-specific types from workspaces.rs
            rp_server::handlers::WorkspaceMember,
            rp_server::handlers::WorkspaceMembers,
            rp_server::handlers::InviteMemberRequest,
            rp_server::handlers::InviteMemberResponse,
            // Handler-specific types from search.rs
            rp_server::handlers::SearchQuery,
            rp_server::handlers::SearchResponse,
            rp_server::handlers::SearchResult,
            rp_server::handlers::SearchFacets,
            // Core types
            rp_core::layer3::EntityType,
        )
    ),
    tags(
        (name = "entities", description = "Entity management operations"),
        (name = "theories", description = "Theory-specific operations"),
        (name = "persons", description = "Person-specific operations"),
        (name = "workspaces", description = "Workspace management operations"),
        (name = "health", description = "Health check endpoints"),
    ),
    info(
        title = "ResearchProcess-GPS API",
        version = "1.0.0",
        description = "Research Process Management System with GPS Integration",
        contact(
            name = "ResearchProcess-GPS Team",
            email = "support@researchprocess-gps.com"
        ),
        license(
            name = "MIT",
        ),
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
        (url = "https://api.researchprocess-gps.com", description = "Production server"),
    ),
)]
struct ApiDoc;

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
        // Search endpoint
        .route("/search", get(search_entities))
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
        // Mount Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Global middleware
        .layer(middleware::from_fn(request_id_middleware))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}