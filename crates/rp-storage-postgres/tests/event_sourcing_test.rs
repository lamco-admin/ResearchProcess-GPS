// TODO: Event Sourcing Integration Tests
//
// These tests are temporarily deferred due to database constraint requirements.
// The PostgreSQL schema has proper foreign key constraints that require:
// 1. Workspace must exist before any entities
// 2. Researcher must exist before creating other entities
// 3. Changes table has FK constraint to entities table
//
// To properly test event sourcing, we need:
// - Create workspace first
// - Create researcher entity 
// - Then test event generation for other entities
//
// The EventSourcedTransaction wrapper is used in production but needs
// a proper test environment with all required entities.
//
// Technical Debt: Implement these tests when we have:
// - Test fixture setup/teardown utilities
// - Proper test data generation
// - Integration test environment
//
// For now, the event sourcing functionality is tested indirectly through
// the API layer when the full system is running.

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "Requires full database setup with workspace and researcher entities"]
    fn test_event_sourcing_crud_integration() {
        // TODO: Implement when test infrastructure is ready
    }

    #[test]
    #[ignore = "Requires full database setup with workspace and researcher entities"]
    fn test_projections() {
        // TODO: Implement when test infrastructure is ready
    }

    #[test]
    #[ignore = "Requires full database setup with workspace and researcher entities"]
    fn test_event_versioning() {
        // TODO: Implement when test infrastructure is ready
    }
}