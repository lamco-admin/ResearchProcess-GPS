---
title: Universal Project Organization Policy
description: Standards for maintaining clean and organized project structure across all LAMCO projects
status: active
author: Greg
date: 2025-07-26
category: standards
tags: [organization, structure, standards, universal, github, redmine]
---

# Universal Project Organization Policy

**Last Updated**: 2025-07-26  
**Purpose**: Standards for maintaining clean and organized project structure  
**Scope**: All LAMCO projects managed through AI Tools infrastructure  
**Critical For**: Consistent development, GitHub/Redmine integration, easy navigation

---

## 🎯 EXECUTIVE SUMMARY

**Problem Solved**: Inconsistent project structures make cross-project work difficult, complicate integrations, and reduce development efficiency.

**Solution**: Universal directory structure with designated locations for all file types, standard integration points for GitHub/Redmine, and consistent organization patterns.

**Key Components**: Standard directory structure, integration directories, documentation standards, archive procedures.

---

## 📋 UNIVERSAL DIRECTORY STRUCTURE

### **Core Project Structure**
Every project MUST follow this base structure:

```
project-root/
├── .venv/                  # Python virtual environment (project-specific)
├── archive/                # Organized storage for outdated content
├── config/                 # Configuration files
├── data/                   # Data files and datasets (if applicable)
├── docs/                   # Active documentation
│   ├── guides/            # How-to guides
│   ├── procedures/        # Step-by-step procedures
│   ├── standards/         # Universal standards (auto-synced from ai-tools)
│   └── troubleshooting/   # Troubleshooting guides
├── logs/                   # All log files (organized by type)
├── redmine-integration/    # Redmine sync infrastructure
│   ├── config/            # Redmine configuration
│   ├── logs/              # Sync logs
│   ├── scripts/           # Sync scripts
│   └── state/             # Sync state files
├── reports/                # Analysis reports and outputs
├── scripts/                # Utility and operational scripts
├── sessions/               # Session handoff documents
├── src/                    # Source code (language-specific structure)
├── tests/                  # All test files and fixtures
└── tools/                  # Development and operational tools
```

### **Archive Organization**
All projects MUST use this archive structure:

```
archive/
├── deprecated/             # Deprecated but historically important
│   ├── scripts/           # Old scripts
│   ├── configs/           # Old configurations
│   └── docs/              # Outdated documentation
├── backups/               # Backup files
│   └── YYYY-MM-DD/        # Date-organized
└── test-artifacts/        # Test outputs
    └── YYYY-MM/           # Month-organized
```

---

## 📝 ROOT DIRECTORY FILES

### **Required Root Files**
Every project MUST have:
- `README.md` - Project overview with setup instructions
- `CLAUDE.md` - AI assistant instructions (generated from ai-tools template)
- `.gitignore` - Version control exclusions
- Requirements file (`requirements.txt`, `package.json`, `Gemfile`, etc.)

### **Optional Root Files**
Projects MAY have:
- `CRITICAL_COMMANDS.md` - Project-specific emergency commands
- `DOCUMENTATION_INDEX.md` - If docs/ becomes extensive
- `.env.example` - Environment variable template
- `docker-compose.yml` - If using Docker
- `Makefile` - Build automation

### **Prohibited in Root**
These MUST NOT be in root:
- Test files (use `tests/`)
- Log files (use `logs/`)
- Temporary files or scripts
- Personal notes or drafts
- Generated reports (use `reports/`)

---

## 🔗 GITHUB INTEGRATION STANDARDS

### **Repository Setup**
All projects MUST:
1. Use the standard label set from `LABEL_GOVERNANCE.md`
2. Configure branch protection for main/master
3. Enable issue templates
4. Set up PR templates
5. Configure GitHub Actions for CI/CD

### **Required Files**
```
.github/
├── ISSUE_TEMPLATE/
│   ├── bug_report.md
│   ├── feature_request.md
│   └── task.md
├── PULL_REQUEST_TEMPLATE.md
├── workflows/
│   ├── ci.yml              # Continuous Integration
│   └── label-check.yml     # Label validation
└── CODEOWNERS
```

### **Branch Strategy**
- `main` or `master` - Production-ready code
- `develop` - Integration branch (if using GitFlow)
- Feature branches: `feature/description`
- Bugfix branches: `bugfix/description`
- Hotfix branches: `hotfix/description`

---

## 📊 REDMINE INTEGRATION STANDARDS

### **Required Structure**
Projects using Redmine MUST have:

```
redmine-integration/
├── config/
│   ├── sync_config.json           # Sync configuration
│   ├── custom_field_mappings.json # Field mappings
│   └── redmine.env               # Redmine credentials (gitignored)
├── logs/
│   ├── sync_issues.log           # Issue sync logs
│   └── sync_prs.log              # PR sync logs
├── scripts/
│   ├── core/                     # Core sync scripts
│   │   ├── setup_project.py      # Initial setup
│   │   ├── sync_issues.py        # Issue synchronization
│   │   └── sync_prs.py           # PR synchronization
│   └── utils/                    # Utility scripts
└── state/
    └── sync_state.json           # Sync state tracking
```

### **Integration Requirements**
1. Two-way sync between GitHub Issues and Redmine
2. Custom field mapping for project-specific data
3. Automated status updates
4. Comment synchronization
5. Label/tag mapping

---

## 📚 DOCUMENTATION STANDARDS

### **Required Documentation**
Every project MUST maintain:
- `README.md` - Setup and overview
- `docs/INDEX.md` - Documentation navigation
- `docs/guides/SETUP_GUIDE.md` - Detailed setup
- `docs/troubleshooting/COMMON_ISSUES.md` - Known problems

### **Documentation Structure**
```
docs/
├── INDEX.md                 # Navigation and overview
├── guides/                  # How-to guides
│   ├── SETUP_GUIDE.md
│   ├── DEVELOPMENT_GUIDE.md
│   └── DEPLOYMENT_GUIDE.md
├── procedures/              # Step-by-step procedures
│   ├── RELEASE_PROCEDURE.md
│   └── BACKUP_PROCEDURE.md
├── architecture/            # System design docs
├── api/                     # API documentation
└── troubleshooting/        # Problem-solving guides
```

---

## 🛡️ STANDARDS INCORPORATION

### **Required Standards**
All projects MUST incorporate these universal standards from ai-tools:

1. **NO_FALLBACK_POLICY** - Zero tolerance for silent failures
2. **AUTHORITY_MATRIX** - AI vs Human decision boundaries
3. **DOCUMENTATION_ORGANIZATION_STANDARDS** - Doc structure rules
4. **LABEL_GOVERNANCE** - GitHub label standards
5. **AI_DATE_BIAS_PREVENTION** - Prevent AI date errors

### **Standards Auto-Sync**
Standards are now automatically synced when running:
```bash
psw refresh-claude  # Automatically syncs standards to current project
```

### **Manual Sync (if needed)**
```bash
# Copy latest standards from ai-tools
cp /home/greg/ai-tools/docs/standards/* ./docs/standards/

# Verify date check script
cp /home/greg/ai-tools/scripts/check_current_date.sh ./scripts/
```

---

## 🔄 PROJECT LIFECYCLE

### **Project Creation**
1. Create repository following structure
2. Run `psw refresh-claude` to sync standards
3. Generate CLAUDE.md from ai-tools template
4. Set up GitHub integration (labels, templates)
5. Configure Redmine integration
6. Initialize with ai-tools project-switch

### **Ongoing Maintenance**
- Weekly: Clean test artifacts
- Monthly: Archive old logs
- Quarterly: Review and update documentation
- Annually: Major archive cleanup

### **Project Archival**
When a project is completed:
1. Create final documentation snapshot
2. Archive all active work
3. Update README with archival notice
4. Remove from active project list

---

## 🚫 ANTI-PATTERNS TO AVOID

1. **Scattered Scripts** - Scripts in root instead of `scripts/`
2. **Unorganized Logs** - Logs scattered throughout project
3. **Missing Integration** - No Redmine/GitHub sync setup
4. **Policy Drift** - Not syncing universal policies
5. **Documentation Decay** - Outdated or missing docs
6. **Test Pollution** - Test files outside `tests/`
7. **Config Chaos** - Configurations not in `config/`

---

## ✅ COMPLIANCE CHECKLIST

- [ ] Directory structure matches template
- [ ] All required root files present
- [ ] GitHub integration configured
- [ ] Redmine integration set up (if applicable)
- [ ] Universal standards synced (via refresh-claude)
- [ ] Documentation structure complete
- [ ] Archive directories created
- [ ] CLAUDE.md generated from template
- [ ] Project added to ai-tools switching

---

**Remember**: Consistent structure enables efficient cross-project work and seamless tool integration!