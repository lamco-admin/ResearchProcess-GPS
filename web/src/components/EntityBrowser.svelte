<script>
  export let workspace
  export let wasmModule

  let entities = []
  let selectedEntity = null
  let entityType = 'Person'
  let propertyKey = ''
  let propertyValue = ''

  function refreshEntities() {
    try {
      entities = workspace.get_all_entities()
      console.log('Entities:', entities)
    } catch (error) {
      console.error('Failed to refresh entities:', error)
    }
  }

  function createEntity() {
    if (!entityType.trim()) {
      alert('Please enter an entity type')
      return
    }

    try {
      const entity = workspace.create_entity(entityType)
      const id = workspace.add_entity(entity)

      console.log('Created entity:', id)
      refreshEntities()

      selectedEntity = entity
    } catch (error) {
      alert('Failed to create entity: ' + error.message)
    }
  }

  function setProperty() {
    if (!selectedEntity) {
      alert('Please select or create an entity first')
      return
    }

    if (!propertyKey.trim() || !propertyValue.trim()) {
      alert('Please enter both key and value')
      return
    }

    try {
      selectedEntity.set_text(propertyKey, propertyValue)

      // Update the entity in workspace
      workspace.add_entity(selectedEntity)

      refreshEntities()

      // Clear inputs
      propertyKey = ''
      propertyValue = ''

      alert('Property set successfully')
    } catch (error) {
      alert('Failed to set property: ' + error.message)
    }
  }

  function selectEntity(entity) {
    selectedEntity = entity
  }

  function getEntityPreview(entity) {
    try {
      const json = typeof entity === 'object' ? entity : JSON.parse(JSON.stringify(entity))
      return JSON.stringify(json, null, 2)
    } catch (error) {
      return String(entity)
    }
  }

  // Initial load
  refreshEntities()
</script>

<style>
  .entity-browser {
    max-width: 1400px;
    margin: 0 auto;
  }

  h2 {
    margin-bottom: 1rem;
    color: #2c3e50;
  }

  .two-column {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
  }

  .panel {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .panel h3 {
    margin-bottom: 1rem;
    color: #2c3e50;
    font-size: 1.25rem;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #555;
  }

  input, select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  input:focus, select:focus {
    outline: none;
    border-color: #3498db;
  }

  button {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  button.primary {
    background: #3498db;
    color: white;
    width: 100%;
  }

  button.primary:hover {
    background: #2980b9;
  }

  button.secondary {
    background: #95a5a6;
    color: white;
  }

  button.secondary:hover {
    background: #7f8c8d;
  }

  .entity-list {
    max-height: 400px;
    overflow-y: auto;
    border: 1px solid #ddd;
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .entity-item {
    padding: 0.75rem;
    border-bottom: 1px solid #ecf0f1;
    cursor: pointer;
    font-size: 0.875rem;
    transition: background 0.2s;
  }

  .entity-item:last-child {
    border-bottom: none;
  }

  .entity-item:hover {
    background: #ecf0f1;
  }

  .entity-item.selected {
    background: #3498db;
    color: white;
  }

  .entity-preview {
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 0.75rem;
    max-height: 400px;
    overflow-y: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .empty-state {
    text-align: center;
    color: #7f8c8d;
    padding: 2rem;
    font-size: 0.875rem;
  }

  .property-editor {
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 4px;
    margin-top: 1rem;
  }

  .property-editor h4 {
    font-size: 1rem;
    margin-bottom: 0.75rem;
    color: #2c3e50;
  }

  .button-row {
    display: flex;
    gap: 0.5rem;
  }

  .count-badge {
    display: inline-block;
    background: #3498db;
    color: white;
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.75rem;
    margin-left: 0.5rem;
  }
</style>

<div class="entity-browser">
  <h2>
    Entity Browser
    <span class="count-badge">{entities.length} entities</span>
  </h2>

  <div class="two-column">
    <div class="panel">
      <h3>Create Entity</h3>

      <div class="form-group">
        <label for="entityType">Entity Type</label>
        <input
          id="entityType"
          type="text"
          bind:value={entityType}
          placeholder="e.g., Person, Family, Source"
        />
      </div>

      <button class="primary" on:click={createEntity}>
        Create Entity
      </button>

      <div class="property-editor">
        <h4>Edit Properties</h4>

        <div class="form-group">
          <label for="propertyKey">Property Key</label>
          <input
            id="propertyKey"
            type="text"
            bind:value={propertyKey}
            placeholder="e.g., name, birth_date"
          />
        </div>

        <div class="form-group">
          <label for="propertyValue">Property Value</label>
          <input
            id="propertyValue"
            type="text"
            bind:value={propertyValue}
            placeholder="Enter value"
          />
        </div>

        <button class="primary" on:click={setProperty}>
          Set Property
        </button>
      </div>
    </div>

    <div class="panel">
      <h3>All Entities</h3>

      {#if entities.length > 0}
        <div class="entity-list">
          {#each entities as entity, index}
            <div
              class="entity-item"
              class:selected={selectedEntity === entity}
              on:click={() => selectEntity(entity)}
            >
              Entity #{index + 1}
            </div>
          {/each}
        </div>

        {#if selectedEntity}
          <h4 style="margin-bottom: 0.5rem;">Selected Entity</h4>
          <div class="entity-preview">
            {getEntityPreview(selectedEntity)}
          </div>
        {/if}
      {:else}
        <div class="empty-state">
          No entities yet. Create your first entity!
        </div>
      {/if}
    </div>
  </div>
</div>
