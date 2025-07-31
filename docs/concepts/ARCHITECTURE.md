# ResearchProcess-GPS Engine Architecture

## Overview

ResearchProcess-GPS is a **protocol and engine** for comprehensive genealogical research that enables:

- Version-controlled research with theory branching and merging
- Evidence-based genealogy with GPS compliance built-in
- Universal data exchange between all genealogy platforms
- Professional collaboration workflows
- Complete DNA analysis integration
- Privacy-first data handling

## Core Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│                    Client Applications                   │
│         (CLI, Desktop, Web, Mobile, IDE Plugins)        │
├─────────────────────────────────────────────────────────┤
│                    Protocol Layer                        │
│          (RGPS Protocol Specifications)                  │
├─────────────────────────────────────────────────────────┤
│                    Engine Core                           │
│  (Domain Models, Business Logic, Theory Management)      │
├─────────────────────────────────────────────────────────┤
│                 Adapter Layer                            │
│     (GRAMPS, GEDCOM, FamilySearch, Ancestry, etc.)     │
├─────────────────────────────────────────────────────────┤
│              Storage Abstraction Layer                   │
│        (Git, PostgreSQL, JIRA, Cloud Services)          │
└─────────────────────────────────────────────────────────┘
```

## Engine Components

### 1. Core Domain Engine (`engine/core/`)

The heart of ResearchProcess-GPS:

- **Entity Management**: Identity, Event, Evidence, Location, Relationship, Theory
- **Theory Engine**: Branching, merging, conflict resolution
- **Evidence Engine**: Floating evidence between theories, GPS compliance
- **Research Process**: Workflow management, task tracking, peer review
- **Publication Engine**: Multi-state publication model
- **Confidence Engine**: Complex confidence tracking with audit trails

### 2. Storage Abstraction (`engine/storage/`)

Pluggable storage backends:

- **Git Backend**: File-based versioning with encryption
- **Database Backend**: PostgreSQL + Apache AGE for graphs
- **JIRA Backend**: Research task management
- **Cloud Backend**: S3, Azure Blob, etc.
- **Hybrid Backend**: Combine multiple storage types

### 3. Protocol Definitions (`engine/protocols/`)

Formal specifications:

- **Object Protocols**: How entities are represented
- **Operation Protocols**: CRUD, versioning, merging
- **Exchange Protocols**: Import/export formats
- **Sync Protocols**: Multi-device/multi-user sync
- **Security Protocols**: Encryption, authentication, authorization

### 4. Adapter System (`engine/adapters/`)

Bidirectional data flow with external systems:

- **GRAMPS Adapter**: Full-fidelity import/export
- **GEDCOM Adapter**: Standard compliance with extensions
- **FamilySearch Adapter**: API integration
- **DNA Platform Adapters**: FTDNA, Ancestry, MyHeritage
- **Research Tool Adapters**: Evidentia, Clooz, etc.

## Key Design Patterns

### 1. Protocol-First Design

```python
# Everything implements protocols
class StorageProtocol:
    def store_entity(self, entity: Entity) -> EntityRef
    def retrieve_entity(self, ref: EntityRef) -> Entity
    def version_entity(self, entity: Entity) -> Version
    
class GitStorage(StorageProtocol):
    # Git-specific implementation
    
class PostgreSQLStorage(StorageProtocol):
    # Database-specific implementation
```

### 2. Theory-Centric Architecture

```python
# Theories are first-class containers
class Theory:
    def branch_from(self, parent: Theory) -> Theory
    def add_evidence(self, evidence: Evidence) -> None
    def merge_with(self, other: Theory) -> MergeResult
    def calculate_confidence(self) -> ConfidenceScore
```

### 3. Evidence Floating

```python
# Evidence exists independently
class Evidence:
    def applicable_to_theories(self) -> List[Theory]
    def support_level_for(self, theory: Theory) -> SupportLevel
    def conflicts_with(self, other: Evidence) -> List[Conflict]
```

### 4. Adapter Pattern

```python
# Clean separation of concerns
class GRAMPSAdapter:
    def import_from_gramps(self, xml_data: str) -> ResearchProject
    def export_to_gramps(self, project: ResearchProject) -> str
    def sync_bidirectional(self, gramps_db: DB, project: ResearchProject) -> SyncResult
```

## Implementation Modules

### Phase 1: Core Engine

1. **Domain Models**: Entity definitions with versioning
2. **Storage Interface**: Abstract storage protocol
3. **Basic Operations**: CRUD with versioning
4. **Theory Management**: Branch, merge, compare

### Phase 2: GRAMPS Integration

1. **GRAMPS Adapter**: Bidirectional data flow
2. **Data Mapping**: GRAMPS ↔ RGPS entities
3. **Sync Protocol**: Live synchronization
4. **Migration Tools**: Bulk import/export

### Phase 3: Advanced Features

1. **Graph Analytics**: Apache AGE integration
2. **DNA Analysis**: Full genetic genealogy support
3. **Plugin System**: Extensibility framework
4. **API Layer**: RESTful + GraphQL

## Why This Architecture?

1. **Protocol over Platform**: Anyone can implement RGPS
2. **Storage Agnostic**: Use what works for you
3. **Theory Native**: Not bolted on afterthought
4. **Professional Grade**: GPS compliance built-in
5. **Future Proof**: Extensible for unknown use cases