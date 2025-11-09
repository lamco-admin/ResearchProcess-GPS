<script>
  export let workspace
  export let wasmModule

  import { onMount } from 'svelte'

  let stats = null
  let storage = null

  onMount(async () => {
    // Get workspace stats
    stats = workspace.stats()

    // Initialize storage
    storage = new wasmModule.WasmStorage('research-gps-db')
    await storage.init()
  })

  async function saveWorkspace() {
    if (!storage) return

    try {
      const workspaceData = workspace.stats()
      await storage.save_workspace(workspaceData)
      alert('Workspace saved to IndexedDB!')
    } catch (error) {
      alert('Failed to save: ' + error.message)
    }
  }

  async function loadWorkspace() {
    if (!storage) return

    try {
      const data = await storage.load_workspace()
      console.log('Loaded workspace:', data)
      alert('Workspace loaded from IndexedDB!')
    } catch (error) {
      alert('Failed to load: ' + error.message)
    }
  }

  async function clearStorage() {
    if (!storage) return

    if (confirm('Are you sure you want to clear all stored data?')) {
      try {
        await storage.clear()
        alert('Storage cleared!')
      } catch (error) {
        alert('Failed to clear: ' + error.message)
      }
    }
  }
</script>

<style>
  .workspace {
    max-width: 1200px;
    margin: 0 auto;
  }

  h2 {
    margin-bottom: 1rem;
    color: #2c3e50;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .stat-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .stat-label {
    font-size: 0.875rem;
    color: #7f8c8d;
    margin-bottom: 0.5rem;
  }

  .stat-value {
    font-size: 2rem;
    font-weight: 700;
    color: #2c3e50;
  }

  .actions {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .button-group {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
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

  button.danger {
    background: #e74c3c;
    color: white;
  }

  button.danger:hover {
    background: #c0392b;
  }

  .info-section {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    margin-top: 2rem;
  }

  .info-section h3 {
    margin-bottom: 1rem;
    color: #2c3e50;
  }

  .info-section p {
    line-height: 1.6;
    color: #555;
  }

  code {
    background: #f5f5f5;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: 'Courier New', monospace;
  }
</style>

<div class="workspace">
  <h2>Workspace: {stats?.name || 'Loading...'}</h2>

  {#if stats}
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">Entities</div>
        <div class="stat-value">{stats.entity_count || 0}</div>
      </div>

      <div class="stat-card">
        <div class="stat-label">Relationships</div>
        <div class="stat-value">{stats.relationship_count || 0}</div>
      </div>

      <div class="stat-card">
        <div class="stat-label">Active Schema</div>
        <div class="stat-value" style="font-size: 1.2rem;">
          {stats.active_schema || 'None'}
        </div>
      </div>
    </div>
  {/if}

  <div class="actions">
    <h3>Storage Actions</h3>
    <div class="button-group">
      <button class="primary" on:click={saveWorkspace}>
        Save to IndexedDB
      </button>
      <button class="secondary" on:click={loadWorkspace}>
        Load from IndexedDB
      </button>
      <button class="danger" on:click={clearStorage}>
        Clear Storage
      </button>
    </div>
  </div>

  <div class="info-section">
    <h3>About This Workspace</h3>
    <p>
      This workspace is running entirely in your browser using WebAssembly.
      All data is stored locally in IndexedDB and never leaves your device.
    </p>
    <p style="margin-top: 1rem;">
      The workspace uses a <strong>Universal Meta-Model</strong> that can adapt to any data structure
      through schema definitions. Load a schema in the "Schemas" tab to get started.
    </p>
    <p style="margin-top: 1rem;">
      Current Features:
    </p>
    <ul style="margin-left: 1.5rem; margin-top: 0.5rem;">
      <li>Universal Entity and Relationship primitives</li>
      <li>Schema-based validation</li>
      <li>Offline-first architecture with IndexedDB</li>
      <li>Calendar-agnostic temporal values</li>
      <li>Multiple uncertainty models (Quantum, Fuzzy, Bayesian)</li>
    </ul>
  </div>
</div>
