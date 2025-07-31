# CLAUDE.md - ResearchProcess-GPS

**Project**: ResearchProcess-GPS - Research Process Management System  
**Type**: Research Management & GPS Integration  
**Created**: January 2025  

## 🚨 MANDATORY REQUIREMENTS - READ FIRST 🚨

### 1. NO_FALLBACK_POLICY (ZERO TOLERANCE)
**THIS IS NON-NEGOTIABLE - VIOLATIONS WILL FAIL CODE REVIEW**
- **Location**: `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md`  
- **Requirement**: ALL errors must be handled explicitly
- **Zero tolerance for**:
  - Silent fallbacks
  - Degraded operation
  - Default assumptions
  - Swallowing errors
  - Optional chaining without explicit handling
- **Every operation must**: Succeed completely OR fail explicitly with clear error

### 2. TIMESTAMP REQUIREMENTS FOR DOCUMENTATION
**MANDATORY for all session documentation**:
1. **Always check current date/time** before creating any document:
   ```bash
   date "+%Y-%m-%d %H:%M:%S %Z"
   ```
2. **Include timestamp in ALL document filenames**:
   - Format: `DOCUMENT_NAME_YYYY_MM_DD_HHMM_TIMEZONE.md`
   - Example: `HANDOVER_2025_07_31_2218_EEST.md`
3. **Include timestamp in document header**:
   ```markdown
   ### Timestamp: 2025-07-31 22:18:00 EEST
   ```

## 🎯 PROJECT CONTEXT

ResearchProcess-GPS is a comprehensive research management system that combines process tracking with geographic information system (GPS) capabilities for location-based research projects.

## 📐 CRITICAL STANDARDS

### AUTHORITY MATRIX

**Location**: `/home/greg/ai-tools/docs/standards/AUTHORITY_MATRIX.md`  
**Purpose**: Defines AI autonomous actions vs human approval requirements  
**Key Rule**: AI handles routine execution, Human handles strategic decisions

## ⚡ QUICK START

### Session Initialization

```bash
# Switch to ResearchProcess-GPS context
project-switch research   # or: psw research

# Refresh Claude configuration
psw refresh-claude

# Start Claude with project context
claude
```

### Essential Commands

```bash
# Check current project
psw current

# View project structure
eza -la

# Search project files
fd "pattern"
rg "search-term"
```

## 📁 PROJECT STRUCTURE

```
ResearchProcess-GPS/
├── src/                  # Source code
├── docs/                 # Documentation
├── tests/                # Test suites
├── data/                 # Research data files
├── scripts/              # Utility scripts
└── config/               # Configuration files
```

## 🔧 KEY COMPONENTS

### Research Process Management

- Process tracking and workflow management
- Research documentation system
- Data collection and analysis tools
- GPS integration for location-based research

### GPS Integration

- Location data collection
- Geographic analysis tools
- Map-based visualization
- Spatial data processing

## 🚀 DEVELOPMENT GUIDELINES

### NO_FALLBACK_POLICY Implementation Examples

**❌ WRONG - Silent failures**:
```rust
let result = operation().unwrap_or_default();  // NO!
let data = fetch().ok()?;                      // NO!
if let Ok(value) = risky_op() { value } else { Default::default() } // NO!
```

**✅ CORRECT - Explicit error handling**:
```rust
let result = operation().map_err(|e| {
    error!("Operation failed: {}", e);
    ApiError::Internal(e.to_string())
})?;

match risky_op() {
    Ok(value) => Ok(value),
    Err(e) => {
        error!("Risky operation failed: {}", e);
        return Err(ApiError::from(e));
    }
}
```

### Tool Usage Preferences

Always use enhanced tools:

```bash
# File searching
fd pattern                # NOT find
rg "search"              # NOT grep
bat file.txt             # NOT cat
eza -la                  # NOT ls
```

### Git Workflow

```bash
# Standard commit
git add -A
git commit -m "type: description"
git push
```

### Testing

```bash
# Run tests (check project-specific test commands)
npm test                 # or appropriate test command
```

## 🔄 INTEGRATION POINTS

### With AI Tools

- Uses AI Tools infrastructure for configuration management
- Integrated with project switching system
- MCP server support for documentation

### With Other Systems

- Can integrate with KnowledgePersistence for research data storage
- Compatible with other LAMCO ecosystem projects

## ⚠️ IMPORTANT NOTES

1. **Project Context**: Always ensure correct project is active before making changes
2. **Configuration**: Project-specific settings override base configurations
3. **Documentation**: Keep research documentation updated in the docs/ directory
4. **Data Management**: Follow established patterns for research data organization

## 📚 DOCUMENTATION

### Key Resources

- **Project README**: `README.md` - Main project documentation
- **Research Guides**: `docs/guides/` - Research methodology documentation
- **API Documentation**: `docs/api/` - Technical API references
- **Data Schemas**: `docs/schemas/` - Data structure definitions

## 🛡️ OPERATIONAL GUIDELINES

### Development Best Practices

1. **Code Quality**: Maintain high code standards with proper error handling
2. **Documentation**: Document all research processes and methodologies
3. **Version Control**: Commit regularly with descriptive messages
4. **Testing**: Ensure all tests pass before committing changes

### Security Considerations

1. **Data Privacy**: Handle research data according to privacy policies
2. **Access Control**: Manage permissions for sensitive research data
3. **API Keys**: Never commit API keys or credentials to the repository

---

## 🔴 FINAL REMINDERS

1. **NO_FALLBACK_POLICY**: Zero violations tolerated. Every error must be handled explicitly.
2. **TIMESTAMPS**: Always check date/time and use timestamps in document filenames.
3. **TESTING**: All code must be tested before committing.
4. **DOCUMENTATION**: Keep comprehensive records of all work done.

*ResearchProcess-GPS - Comprehensive Research Process Management with GPS Integration*