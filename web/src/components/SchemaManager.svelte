<script>
  export let workspace
  export let wasmModule

  let schemaYaml = ''
  let schemaFiles = [
    { name: 'GEDCOM Basic', path: '/schemas/gedcom-basic.yaml' }
  ]
  let selectedFile = null
  let loadedSchemas = []

  async function loadSchemaFile(path) {
    try {
      const response = await fetch(path)
      if (!response.ok) throw new Error('Failed to fetch schema')

      schemaYaml = await response.text()
    } catch (error) {
      alert('Failed to load schema file: ' + error.message)
    }
  }

  async function loadSchema() {
    if (!schemaYaml.trim()) {
      alert('Please enter or load a schema YAML first')
      return
    }

    try {
      const schemaId = workspace.load_schema_yaml(schemaYaml)
      alert(`Schema loaded successfully: ${schemaId}`)

      // Refresh loaded schemas list
      await refreshSchemas()

      // Clear the input
      schemaYaml = ''
    } catch (error) {
      alert('Failed to load schema: ' + error.message)
    }
  }

  async function setActiveSchema(schemaId) {
    try {
      workspace.set_active_schema(schemaId)
      alert(`Active schema set to: ${schemaId}`)
    } catch (error) {
      alert('Failed to set active schema: ' + error.message)
    }
  }

  async function refreshSchemas() {
    // This would require exposing a list_schemas method on WasmWorkspace
    // For now, we'll just track the active schema
    const active = workspace.active_schema()
    loadedSchemas = active ? [active] : []
  }

  function selectFile(file) {
    selectedFile = file
    loadSchemaFile(file.path)
  }
</script>

<style>
  .schema-manager {
    max-width: 1200px;
    margin: 0 auto;
  }

  h2 {
    margin-bottom: 1rem;
    color: #2c3e50;
  }

  .two-column {
    display: grid;
    grid-template-columns: 250px 1fr;
    gap: 1.5rem;
    margin-top: 1.5rem;
  }

  .sidebar {
    background: white;
    padding: 1rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    height: fit-content;
  }

  .sidebar h3 {
    font-size: 1rem;
    margin-bottom: 0.75rem;
    color: #2c3e50;
  }

  .file-list {
    list-style: none;
  }

  .file-item {
    padding: 0.5rem;
    cursor: pointer;
    border-radius: 4px;
    margin-bottom: 0.25rem;
    font-size: 0.875rem;
    transition: background 0.2s;
  }

  .file-item:hover {
    background: #ecf0f1;
  }

  .file-item.selected {
    background: #3498db;
    color: white;
  }

  .main-area {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  textarea {
    width: 100%;
    min-height: 400px;
    padding: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    resize: vertical;
  }

  textarea:focus {
    outline: none;
    border-color: #3498db;
  }

  .button-group {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
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

  .loaded-schemas {
    background: #e8f4f8;
    padding: 1rem;
    border-radius: 4px;
    margin-top: 1rem;
  }

  .loaded-schemas h4 {
    font-size: 0.875rem;
    color: #2c3e50;
    margin-bottom: 0.5rem;
  }

  .schema-badge {
    display: inline-block;
    background: #3498db;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    font-size: 0.875rem;
    margin-right: 0.5rem;
    cursor: pointer;
  }

  .schema-badge:hover {
    background: #2980b9;
  }

  .schema-badge.active {
    background: #27ae60;
  }

  .empty-state {
    text-align: center;
    color: #7f8c8d;
    font-size: 0.875rem;
  }
</style>

<div class="schema-manager">
  <h2>Schema Management</h2>

  <div class="two-column">
    <div class="sidebar">
      <h3>Example Schemas</h3>
      <ul class="file-list">
        {#each schemaFiles as file}
          <li
            class="file-item"
            class:selected={selectedFile === file}
            on:click={() => selectFile(file)}
          >
            {file.name}
          </li>
        {/each}
      </ul>
    </div>

    <div class="main-area">
      <h3>Schema YAML</h3>
      <textarea
        bind:value={schemaYaml}
        placeholder="Paste schema YAML here, or select an example from the sidebar..."
      ></textarea>

      <div class="button-group">
        <button class="primary" on:click={loadSchema}>
          Load Schema
        </button>
        <button class="secondary" on:click={() => schemaYaml = ''}>
          Clear
        </button>
      </div>

      {#if loadedSchemas.length > 0}
        <div class="loaded-schemas">
          <h4>Loaded Schemas</h4>
          {#each loadedSchemas as schema}
            <span
              class="schema-badge active"
              on:click={() => setActiveSchema(schema)}
            >
              {schema} (Active)
            </span>
          {/each}
        </div>
      {:else}
        <div class="loaded-schemas">
          <div class="empty-state">
            No schemas loaded yet
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
