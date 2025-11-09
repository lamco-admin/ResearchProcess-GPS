<script>
  import { onMount } from 'svelte'
  import Workspace from './components/Workspace.svelte'
  import SchemaManager from './components/SchemaManager.svelte'
  import EntityBrowser from './components/EntityBrowser.svelte'

  let wasmModule = null
  let workspace = null
  let loading = true
  let error = null
  let activeTab = 'workspace'

  onMount(async () => {
    try {
      // Load WASM module
      wasmModule = await import('../../pkg/rp_wasm.js')
      await wasmModule.default()

      console.log('WASM module loaded successfully')

      // Create a workspace
      workspace = await wasmModule.WasmWorkspace.new('My Research')

      console.log('Workspace created:', workspace.name())

      loading = false
    } catch (err) {
      console.error('Failed to load WASM module:', err)
      error = err.message || String(err)
      loading = false
    }
  })
</script>

<style>
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  header {
    background: #2c3e50;
    color: white;
    padding: 1rem 2rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  h1 {
    font-size: 1.5rem;
    font-weight: 600;
  }

  .subtitle {
    font-size: 0.875rem;
    opacity: 0.8;
    margin-top: 0.25rem;
  }

  nav {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
    border-bottom: 2px solid rgba(255,255,255,0.1);
  }

  .tab {
    padding: 0.5rem 1rem;
    background: none;
    border: none;
    color: rgba(255,255,255,0.7);
    cursor: pointer;
    font-size: 0.875rem;
    border-bottom: 2px solid transparent;
    margin-bottom: -2px;
    transition: all 0.2s;
  }

  .tab:hover {
    color: white;
    background: rgba(255,255,255,0.1);
  }

  .tab.active {
    color: white;
    border-bottom-color: #3498db;
  }

  main {
    flex: 1;
    overflow: auto;
    padding: 2rem;
  }

  .loading, .error {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    flex-direction: column;
    gap: 1rem;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .error-box {
    background: #fee;
    border: 1px solid #fcc;
    padding: 1rem;
    border-radius: 4px;
    max-width: 600px;
  }

  .error-box h2 {
    color: #c00;
    margin-bottom: 0.5rem;
  }
</style>

<div class="app">
  <header>
    <h1>ResearchProcess-GPS</h1>
    <div class="subtitle">Universal Meta-Model Research Platform</div>

    {#if workspace}
      <nav>
        <button
          class="tab"
          class:active={activeTab === 'workspace'}
          on:click={() => activeTab = 'workspace'}
        >
          Workspace
        </button>
        <button
          class="tab"
          class:active={activeTab === 'schema'}
          on:click={() => activeTab = 'schema'}
        >
          Schemas
        </button>
        <button
          class="tab"
          class:active={activeTab === 'entities'}
          on:click={() => activeTab = 'entities'}
        >
          Entities
        </button>
      </nav>
    {/if}
  </header>

  <main>
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading WASM module...</p>
      </div>
    {:else if error}
      <div class="error">
        <div class="error-box">
          <h2>Failed to Load</h2>
          <p>{error}</p>
          <p style="margin-top: 1rem; font-size: 0.875rem;">
            Make sure you've built the WASM module with: <code>npm run wasm:build</code>
          </p>
        </div>
      </div>
    {:else if workspace}
      {#if activeTab === 'workspace'}
        <Workspace {workspace} {wasmModule} />
      {:else if activeTab === 'schema'}
        <SchemaManager {workspace} {wasmModule} />
      {:else if activeTab === 'entities'}
        <EntityBrowser {workspace} {wasmModule} />
      {/if}
    {/if}
  </main>
</div>
