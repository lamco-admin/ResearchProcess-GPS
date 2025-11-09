# ResearchProcess-GPS Web Interface

Web-based interface for ResearchProcess-GPS built with WebAssembly and Svelte.

## Prerequisites

1. **Rust** with wasm32-unknown-unknown target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **wasm-pack** for building WASM modules:
   ```bash
   cargo install wasm-pack
   ```

3. **Node.js** (v18 or higher) and npm

## Quick Start

### 1. Install Dependencies

```bash
cd web
npm install
```

### 2. Build WASM Module

```bash
npm run wasm:build
```

Or for development (faster builds, larger files):
```bash
npm run wasm:dev
```

### 3. Start Development Server

```bash
npm run dev
```

The application will be available at `http://localhost:3000`

## Project Structure

```
web/
├── src/
│   ├── main.js              # Application entry point
│   ├── App.svelte           # Main application component
│   └── components/
│       ├── Workspace.svelte     # Workspace management
│       ├── SchemaManager.svelte  # Schema loading and management
│       └── EntityBrowser.svelte  # Entity creation and browsing
├── public/                  # Static assets
├── index.html              # HTML entry point
├── vite.config.js          # Vite configuration
└── package.json            # NPM dependencies

```

## Building for Production

```bash
# Build WASM module in release mode
npm run wasm:build

# Build web application
npm run build

# Preview production build
npm run preview
```

The production build will be in the `dist/` directory.

## Features

### Universal Meta-Model
- Flexible entity and relationship primitives
- Schema-based validation
- Support for multiple data models (GEDCOM, GRAMPS, etc.)

### Offline-First
- All data stored in IndexedDB
- Works completely offline
- No server required

### Developer-Friendly
- Hot module replacement in development
- TypeScript support (optional)
- Comprehensive error messages

## Schemas

Schema files are located in `/schemas` directory. Available schemas:

- `gedcom-basic.yaml` - Basic GEDCOM-compatible schema
- More schemas coming soon...

Load schemas via the "Schemas" tab in the web interface.

## Troubleshooting

### WASM Module Not Found

If you see "Failed to load WASM module", make sure you've built it:
```bash
npm run wasm:build
```

### Build Errors

Clean and rebuild:
```bash
# Clean WASM build
rm -rf pkg/

# Clean npm build
rm -rf node_modules/ dist/

# Reinstall and rebuild
npm install
npm run wasm:build
npm run dev
```

### Memory Issues

If you encounter memory issues during WASM build, try:
```bash
# Build with more memory
NODE_OPTIONS="--max-old-space-size=4096" npm run wasm:build
```

## Development

### Adding New Components

1. Create component in `src/components/`
2. Import in `App.svelte`
3. Add to tab navigation if needed

### Modifying WASM Bindings

1. Edit Rust code in `../crates/rp-wasm/`
2. Rebuild WASM: `npm run wasm:dev`
3. Refresh browser

### Hot Reload

Vite provides hot module replacement for Svelte components. WASM changes require a rebuild.

## License

MIT OR Apache-2.0
