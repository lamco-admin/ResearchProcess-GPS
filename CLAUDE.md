# CLAUDE.md - ResearchProcess-GPS

**Project**: ResearchProcess-GPS - Research Process Management System  
**Type**: Research Management & GPS Integration  
**Created**: January 2025  

## 🎯 PROJECT CONTEXT

ResearchProcess-GPS is a comprehensive research management system that combines process tracking with geographic information system (GPS) capabilities for location-based research projects.

## 📐 CRITICAL STANDARDS

### NO FALLBACK POLICY (MANDATORY)

**Location**: `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md`  
**Status**: Zero tolerance - all systems must fail explicitly  
**Key Rule**: No silent fallbacks, no degraded operation, no assumptions

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

*ResearchProcess-GPS - Comprehensive Research Process Management with GPS Integration*