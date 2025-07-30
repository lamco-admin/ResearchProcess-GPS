# Hybrid Storage Architecture: Files, Databases, and Everything Between

## Core Principle: Storage Agnostic

The ResearchProcess-GPS protocol doesn't care WHERE data lives - only HOW it's structured.

## Universal Storage Adapter Layer

```yaml
StorageAdapter:
  # Interface all storage engines must implement
  interface:
    # Basic CRUD
    - create(entity_type, data) → entity_id
    - read(entity_type, entity_id) → data
    - update(entity_type, entity_id, data) → success
    - delete(entity_type, entity_id) → success
    
    # Query Operations
    - find(entity_type, criteria) → [entities]
    - find_related(entity_id, relationship) → [entities]
    
    # Version Operations
    - get_history(entity_id) → [versions]
    - get_version(entity_id, version) → data
    - create_checkpoint(message) → checkpoint_id
    
    # Theory Operations  
    - list_theories() → [theory_names]
    - switch_theory(theory_name) → success
    - create_theory(name, from_theory) → success
    - merge_theories(from, to) → conflicts[]
```

## Storage Implementations

### 1. Git File Storage (Default)

```yaml
GitFileAdapter:
  configuration:
    root_path: "./research-project"
    file_format: "yaml"  # or json, xml
    
  implementation:
    create: "Write YAML file + git add + git commit"
    read: "Read YAML file"
    update: "Modify file + git commit"
    get_history: "git log for file"
    create_theory: "git checkout -b"
    
  benefits:
    - "Works offline"
    - "No database needed"
    - "Version control built in"
    - "Human readable"
```

### 2. SQL Database Storage

```yaml
SQLDatabaseAdapter:
  configuration:
    connection: "postgresql://localhost/rgps"
    schema: "research"
    
  tables:
    entities:
      - id: UUID
      - type: string
      - data: JSONB
      - version: integer
      - created_at: timestamp
      - updated_at: timestamp
      
    versions:
      - id: UUID
      - entity_id: UUID
      - version: integer
      - data: JSONB
      - author: string
      - message: string
      - created_at: timestamp
      
    theories:
      - id: UUID
      - name: string
      - parent_theory: UUID
      - status: string
      
    theory_entities:
      - theory_id: UUID
      - entity_id: UUID
      - entity_version: integer
      
  benefits:
    - "Fast queries"
    - "Concurrent users"
    - "ACID transactions"
    - "Scalable"
```

### 3. JIRA Integration

```yaml
JIRAAdapter:
  configuration:
    server: "https://mycompany.atlassian.net"
    project: "GENR"
    
  mapping:
    Identity → JIRA Issue:
      issue_type: "Person"
      fields:
        summary: "${given_name} ${surname}"
        description: "YAML data in description"
        custom_field_10001: "birth_date"
        custom_field_10002: "death_date"
        
    Evidence → JIRA Issue:
      issue_type: "Evidence"
      attachments: "Document images"
      
    Theory → JIRA Version:
      version_name: "Theory: ${name}"
      issues: "Entities in theory"
      
    Research_Task → JIRA Task:
      issue_type: "Task"
      parent: "Epic for research project"
      
  benefits:
    - "Project management integration"
    - "Team collaboration"
    - "Workflow automation"
    - "Time tracking"
```

### 4. Hybrid Git+Database

```yaml
HybridAdapter:
  configuration:
    files: "./research-files"  # Git repo
    database: "postgresql://localhost/rgps"
    
  strategy:
    metadata: "Database"  # Fast queries
    content: "Files"      # Version control
    indices: "Database"   # Relationships
    media: "Files"       # Large binaries
    
  sync_mechanism:
    - "File changes trigger DB update"
    - "DB changes write to files"
    - "Git commits update DB versions"
    - "Conflicts resolved by rules"
    
  benefits:
    - "Best of both worlds"
    - "Query performance"
    - "Version control"
    - "Offline capability"
```

### 5. Cloud Service Adapters

```yaml
CloudAdapters:
  GoogleDrive:
    files: "Drive folder structure"
    metadata: "Google Sheets"
    sharing: "Native Google sharing"
    
  Dropbox:
    files: "Dropbox sync"
    versions: "Dropbox version history"
    
  OneDrive:
    files: "OneDrive folders"
    metadata: "SharePoint lists"
    
  S3:
    objects: "S3 buckets"
    metadata: "DynamoDB"
    query: "Athena"
```

## Adapter Selection Logic

```yaml
AdapterSelector:
  # Auto-detect best adapter
  detection_order:
    1. check_for_git_repo: "Use GitFileAdapter"
    2. check_for_database_config: "Use configured DB"
    3. check_for_jira_config: "Use JIRA"
    4. check_for_cloud_sync: "Use cloud adapter"
    5. fallback: "Create new Git repo"
    
  # User can override
  explicit_config:
    storage_adapter: "hybrid"
    primary_storage: "database"
    backup_storage: "git"
    media_storage: "s3"
```

## Standalone Mode Architecture

```yaml
StandaloneMode:
  # For single researchers
  embedded_database:
    engine: "SQLite"
    file: "research.db"
    
  benefits:
    - "No server needed"
    - "Fast queries"
    - "Single file"
    - "Works offline"
    
  sync_options:
    - "Export to Git periodically"
    - "Backup to cloud"
    - "Share via export"
```

## Data Portability

```yaml
ImportExport:
  # Move between storage types
  export_formats:
    - "Git repository (files)"
    - "SQL dump"
    - "JIRA backup"
    - "JSON archive"
    - "GEDCOM 7"
    
  migration_tools:
    git_to_database:
      command: "rgps migrate git-to-sql"
      preserves: "All history"
      
    database_to_git:
      command: "rgps migrate sql-to-git"
      creates: "Git history from versions"
      
    jira_sync:
      command: "rgps sync-jira"
      bidirectional: true
```

## Example Configurations

### Solo Researcher

```yaml
# .rgps/config.yaml
storage:
  adapter: "git"
  format: "yaml"
  auto_commit: true
  backup:
    - type: "google_drive"
      folder: "Genealogy/Research"
```

### Research Team

```yaml
# .rgps/config.yaml
storage:
  adapter: "hybrid"
  database:
    type: "postgresql"
    host: "team-server.local"
  files:
    type: "git"
    remote: "git@github.com:team/research.git"
  media:
    type: "s3"
    bucket: "team-research-media"
```

### Enterprise Integration

```yaml
# .rgps/config.yaml
storage:
  adapter: "jira"
  server: "https://enterprise.atlassian.net"
  project: "FAMILY_RESEARCH"
  backup:
    adapter: "sql"
    connection: "oracle://backup-server"
```

## Benefits of This Architecture

1. **Use What You Have**: JIRA at work? Use it. Just files? Perfect.
2. **No Lock-in**: Switch storage anytime
3. **Scale as Needed**: Start with files, move to database
4. **Team Friendly**: Multiple storage options for different needs
5. **Tool Agnostic**: Storage doesn't dictate tools

The key is that ResearchProcess-GPS defines the **data model and operations**, not the storage mechanism. This lets genealogists use whatever storage makes sense for their situation.