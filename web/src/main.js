import App from './App.svelte'

// Import WASM module
async function init() {
  try {
    // The WASM module will be loaded dynamically
    // This will be available after running npm run wasm:build
    const app = new App({
      target: document.getElementById('app')
    })

    return app
  } catch (error) {
    console.error('Failed to initialize application:', error)
    document.getElementById('app').innerHTML = `
      <div style="padding: 2rem; max-width: 800px; margin: 0 auto;">
        <h1>ResearchProcess-GPS</h1>
        <div style="background: #fee; padding: 1rem; border-radius: 4px; margin-top: 1rem;">
          <h2 style="color: #c00;">Initialization Error</h2>
          <p style="margin-top: 0.5rem;">${error.message}</p>
          <details style="margin-top: 1rem;">
            <summary>Build Instructions</summary>
            <pre style="background: #f5f5f5; padding: 1rem; margin-top: 0.5rem; overflow-x: auto;">
# Build the WASM module first:
npm run wasm:build

# Or for development:
npm run wasm:dev

# Then start the dev server:
npm run dev
            </pre>
          </details>
        </div>
      </div>
    `
  }
}

init()
