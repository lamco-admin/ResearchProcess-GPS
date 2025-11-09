# ResearchProcess-GPS

A revolutionary protocol and engine for genealogical research that transforms how professional genealogists conduct, document, and share their research. ResearchProcess-GPS is "GitHub for Genealogy" - not just another platform, but a **protocol** that enables version-controlled research with theory branching, evidence floating, and professional collaboration.

## 🚀 New: Universal Meta-Model Implementation

**The flexible data model is now live!** We've implemented a complete working system that adapts to ANY genealogical data model through schema definitions.

### Quick Start

```bash
# Install dependencies
cd web && npm install

# Build WASM module
npm run wasm:build

# Start development server
npm run dev

# Open http://localhost:3000
```

### What's New

- ✅ **Universal Meta-Model**: Work with GEDCOM, GRAMPS, or custom models
- ✅ **Web Interface**: Complete Svelte-based UI running in browser
- ✅ **Offline-First**: All data in IndexedDB, no server required
- ✅ **Schema System**: Define your own data models with YAML
- ✅ **WASM-Powered**: Native performance in the browser

See [Getting Started Guide](docs/guides/GETTING_STARTED.md) for complete setup instructions.

## 🎯 Vision

ResearchProcess-GPS addresses a fundamental gap in genealogy software: current tools focus on recording conclusions, not the research process. This platform revolutionizes genealogical research by:

- **Tracking HOW** research was conducted, not just the results
- **Supporting uncertainty** with flexible personas and identities instead of fixed persons
- **Enabling theory exploration** with "What if?" scenarios and version control
- **Documenting confidence comprehensively** with complete audit trails, not just numeric scores
- **Facilitating collaboration** with multi-researcher attribution and peer review

## 🏗️ Architecture Overview

### Three-Pillar Foundation

1. **Secure Data Archiving** (.rgps filesystem format)
   - Self-contained research packages with all dependencies
   - Cryptographic signatures and encryption
   - Long-term preservation with format migration
   - Offline-first operation

2. **Streaming/Syncing System** 
   - Event-sourced architecture tracking all changes
   - Real-time synchronization with conflict resolution
   - Selective sync with privacy controls
   - Distributed collaboration support

3. **Collaborative Engine**
   - Multi-modal communication (text, voice, annotations)
   - Real-time presence and activity awareness
   - Peer review workflows
   - Attribution preservation

### Key Architectural Principles

- **Standards-as-Configuration**: Professional standards (BCG, GPS) as versioned configs
- **Plugin-First Design**: Core research framework with extensible specialized features
- **Professional-Grade**: GPS compliance and BCG standards built-in from foundation
- **Semantic Preservation**: Complete meaning preservation across all operations

## 🚀 Revolutionary Features

### Theory Versioning (The Killer Feature)
Transform static GEDCOM persons into flexible personas/identities that can be reassigned between different theoretical family structures:
- Build multiple family tree versions ("What if Sam died in 1853?")
- Compare theories side-by-side with impact analysis
- Evidence supports multiple theories simultaneously
- Git-like version control for genealogy

### Comprehensive Confidence Framework
Replace meaningless numeric scores with complete research audit trails:
- Document exactly which repositories and collections were searched
- Track temporal and geographic coverage
- GPS element compliance verification
- Peer review trails and confidence evolution
- Granular auditing: "Did you check the Boise Library obituary collection?"

### GEDCOM Liberation
- Import GEDCOM 7+ with extensions
- Decompose persons into flexible personas and identities
- Float evidence between theories
- Maintain relationships while exploring alternatives

## 📁 Project Structure

### Core Specifications (12 Major Documents)
- `PLATFORM_ARCHITECTURE_VISION.md` - Overall system design
- `IDENTITY_PERSONA_SYSTEM.md` - Revolutionary identity management
- `FLEXIBLE_LINK_SYSTEM.md` - Relationships beyond GEDCOM
- `RESEARCH_LOG_SPECIFICATION.md` - GPS-compliant research tracking
- `DNA_MODELS_SPECIFICATION.md` - Multi-type genetic genealogy
- `SYNC_AND_OPERATION_MODES.md` - Offline to real-time collaboration
- [And 6 more comprehensive specifications...]

### Architecture Documents
- `HIGH_LEVEL_ARCHITECTURE_20250729_160300.md` - Three-pillar system design
- `COMPREHENSIVE_CONFIDENCE_FRAMEWORK.md` - Revolutionary confidence model
- `THEORY_VERSIONING_AND_TREE_BUILDING.md` - Dynamic genealogy features
- `CORE_DATA_MODEL_VS_APPLICATION_FEATURES.md` - Clean architecture separation

### Analysis & Research
- `COMPREHENSIVE_MARKET_ANALYSIS_2025.md` - Professional genealogist needs
- `SEMANTIC_LOSS_ANALYSIS.md` - Data loss in current systems
- `RESEARCH_PROCESS_FEATURE_COMPILATION.md` - User requirements from BetterGEDCOM
- Multiple extension analyses and integration studies

## 🎯 Target Users

### Primary: Professional Genealogists
- BCG-certified genealogists and candidates
- Professional researchers requiring GPS compliance
- Genealogists handling complex identity resolution
- Researchers working with uncertain or conflicting evidence

### Secondary: Serious Researchers
- Advanced hobbyists following professional standards
- Collaborative research teams
- Academic genealogy programs
- Genealogical societies and organizations

## 🛠️ Technology Stack (Planned)

- **Backend**: PostgreSQL with event sourcing, Node.js/Python
- **Storage**: Hybrid file/stream architecture with .rgps packages
- **Sync**: WebSocket real-time, CRDT for conflict-free merge
- **Security**: End-to-end encryption, digital signatures
- **Standards**: GEDCOM 7 compatible with extensive extensions

## 📊 Current Status

### ✅ Phase 1 Completed: Universal Meta-Model Core
- **Universal Meta-Model**: Four primitives (Entity, Relationship, Context, Certainty)
- **Schema System**: YAML-based schema definitions with validation
- **WASM Bridge**: Complete WebAssembly bindings for browser
- **Web Interface**: Svelte-based UI with offline-first architecture
- **IndexedDB Storage**: Browser-based persistence
- **Example Schemas**: GEDCOM 7.0 and GRAMPS complete schemas
- **Comprehensive Documentation**: Getting Started, Developer Guide, Schema Guide

### 🔄 Phase 2 In Progress: Adapters and Ecosystem
- GEDCOM importer/exporter
- GRAMPS connector
- Plugin marketplace foundations
- Advanced query system

### ⏳ Next Steps
- Production deployment
- Professional genealogist validation
- Community schema contributions
- Native desktop app (iced-based GUI)

## 🤝 Contributing

This project is currently in the architecture and design phase. We welcome:
- Feedback from professional genealogists
- Use case contributions
- Standards compliance expertise
- Technical architecture review

## 📚 Key Concepts

### Research Process Focus
Unlike traditional genealogy software that records conclusions, ResearchProcess-GPS tracks:
- Research questions and hypotheses
- Search strategies and coverage
- Evidence analysis and conflicts
- Decision reasoning and alternatives
- Negative findings and exhausted searches

### Professional Standards Integration
- BCG (Board for Certification of Genealogists) compliance
- GPS (Genealogical Proof Standard) methodology
- NGS (National Genealogical Society) guidelines
- International genealogy standards support

### Collaboration-Native Design
- Multi-researcher attribution
- Peer review workflows
- Version control and conflict resolution
- Real-time and asynchronous collaboration

## 🔗 Related Projects

- Parent: `/genealogy-ai` - AI-powered genealogy research tools
- Related: GEDCOM 7 extensions work
- Integration: Various genealogy platform connectors

## 📖 Getting Started

To understand the project vision and architecture:

1. Read `PLATFORM_ARCHITECTURE_VISION.md` for overall design
2. Review `SESSION_HANDOVER_20250729_164500.md` for current status
3. Explore `HIGH_LEVEL_ARCHITECTURE_20250729_160300.md` for technical details
4. Check `COMPREHENSIVE_PROJECT_SUMMARY_20250729_164500.md` for complete overview

## 🚦 Roadmap

### Phase 1: Core Platform (Current)
- Data model design
- Standards framework
- Basic plugin architecture

### Phase 2: Professional Features
- Theory versioning implementation
- Confidence framework
- Collaboration engine

### Phase 3: Ecosystem Development
- Plugin marketplace
- API ecosystem
- Professional tool integrations

### Phase 4: Platform Maturity
- Enterprise features
- International expansion
- Advanced analytics

---

*ResearchProcess-GPS: Transforming genealogical research from a conclusion-recording exercise to a dynamic, collaborative investigation platform.*