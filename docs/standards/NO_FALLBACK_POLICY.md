---
title: NO FALLBACK POLICY
description: Mandatory zero-tolerance policy requiring explicit failure handling with no silent fallbacks
status: active
author: Greg / Claude (Anthropic)
date: 2025-07-20
category: policy
tags: [policy, no-fallback, error-handling, mandatory, zero-tolerance]
---

# NO FALLBACK POLICY

**Version**: 1.1  
**Effective Date**: July 20, 2025  
**Status**: MANDATORY - Zero Tolerance Policy  
**Applies To**: All AI-Tools Infrastructure and Integrated Projects

## Executive Summary

The NO FALLBACK POLICY mandates that all systems must fail explicitly when required resources, configurations, or dependencies are unavailable. Silent fallbacks to defaults are strictly prohibited. This policy ensures system reliability, security, and operational transparency across all AI-Tools managed projects.

## Core Principles

### 1. FAIL FAST
- Systems must terminate immediately upon encountering missing requirements
- No attempts to continue with partial functionality
- Exit codes must be non-zero for all failures

### 2. FAIL LOUD
- All failures must be logged with clear error messages
- Critical failures must trigger notifications
- Error messages must specify what is missing and how to fix it

### 3. NO ASSUMPTIONS
- Never assume default values for critical configurations
- Never assume localhost or default ports
- Never assume empty strings or None are acceptable

### 4. NO DEGRADATION
- Systems must not operate in degraded mode
- Partial functionality is not acceptable
- All-or-nothing operation only

## AI-Tools Specific Requirements

### Project Switching
```bash
# ❌ PROHIBITED - Silent fallback to default project
project="${CURRENT_PROJECT:-ai-tools}"

# ✅ REQUIRED - Explicit project validation
if [ -z "$CURRENT_PROJECT" ]; then
    echo "❌ FATAL: No project selected"
    echo "💡 Run: psw <project-name>"
    exit 1
fi
```

### Environment Loading
```bash
# ❌ PROHIBITED - Continue without environment
source ~/.env.kp-ai 2>/dev/null || true

# ✅ REQUIRED - Environment must load
if ! source ~/.env.kp-ai; then
    echo "❌ FATAL: Failed to load KP environment"
    echo "💡 Check ~/.env.kp-ai exists and is valid"
    exit 1
fi
```

### MCP Server Configuration
```python
# ❌ PROHIBITED - Default MCP configuration
mcp_config = load_mcp_config() or {"servers": {}}

# ✅ REQUIRED - MCP config must exist
mcp_config_path = os.path.expanduser("~/.claude/mcp.json")
if not os.path.exists(mcp_config_path):
    print("❌ FATAL: MCP configuration not found")
    print("💡 Run: psw refresh-claude")
    sys.exit(1)
```

## Implementation Requirements

### Environment Variables
```python
# ❌ PROHIBITED - Silent fallback
db_host = os.getenv('DB_HOST', 'localhost')

# ✅ REQUIRED - Explicit failure
db_host = os.getenv('DB_HOST')
if not db_host:
    print("❌ FATAL: DB_HOST environment variable not set")
    print("💡 Set with: export DB_HOST='your-database-host'")
    sys.exit(1)
```

### Database Connections
```python
# ❌ PROHIBITED - Auto-create schema
CREATE TABLE IF NOT EXISTS ...

# ✅ REQUIRED - Schema must exist
# Tables must be created through proper migrations
# Connection must fail if schema is missing
```

### Error Handling
```python
# ❌ PROHIBITED - Silent failure
try:
    critical_operation()
except:
    pass

# ✅ REQUIRED - Explicit error handling
try:
    critical_operation()
except Exception as e:
    logger.error(f"Critical operation failed: {e}")
    notify_admin(f"FATAL: {e}")
    sys.exit(1)
```

### Configuration Files
```python
# ❌ PROHIBITED - Missing config fallback
config = load_config('app.conf') or {}

# ✅ REQUIRED - Config must exist
if not os.path.exists('app.conf'):
    print("❌ FATAL: Configuration file 'app.conf' not found")
    sys.exit(1)
config = load_config('app.conf')
```

## Scope

This policy applies to:
- All AI-Tools infrastructure components
- All Python applications and scripts
- All shell scripts and automation
- All database operations
- All API integrations
- All deployment configurations
- All CI/CD pipelines
- All integrated projects (KP, NavyCMMS, etc.)

## Enforcement

### Code Reviews
- All PRs must be reviewed for fallback violations
- Automated linting rules detect common patterns
- NO FALLBACK compliance is merge-blocking

### Testing
- Tests must verify failure behavior
- Tests must ensure non-zero exit codes
- Tests must validate error messages

### Monitoring
- Systems must alert on fallback attempts
- Logs must capture all failure events
- Metrics must track policy compliance

## Common Violations

### 1. Localhost Defaults
```python
# ❌ VIOLATION
host = config.get('host', 'localhost')
```

### 2. Empty String Fallbacks
```python
# ❌ VIOLATION
password = os.getenv('PASSWORD', '')
```

### 3. Silent Schema Creation
```sql
-- ❌ VIOLATION
CREATE TABLE IF NOT EXISTS users ...
```

### 4. Ignored Exceptions
```python
# ❌ VIOLATION
except Exception:
    continue
```

### 5. Optional Dependencies
```python
# ❌ VIOLATION
try:
    import numpy
    HAS_NUMPY = True
except ImportError:
    HAS_NUMPY = False
```

## AI-Tools Specific Violations

### 6. Silent Project Defaults
```bash
# ❌ VIOLATION
psw ${1:-kp-ai}
```

### 7. Missing Claude Refresh
```bash
# ❌ VIOLATION
psw navycmms && claude  # Missing refresh-claude
```

### 8. Ignored Hook Failures
```python
# ❌ VIOLATION
try:
    run_hook()
except:
    logger.warning("Hook failed, continuing...")
```

## Migration Guide

### Phase 1: Discovery
```bash
# Find environment fallbacks
rg "os\.getenv\([^)]+,[^)]+\)" --type py

# Find shell parameter defaults
rg '\$\{[^}]+:-[^}]+\}' --type sh

# Find localhost references
rg "localhost" --type py

# Find silent exceptions
rg "except.*:\s*pass" --type py
```

### Phase 2: Remediation
1. Remove all default values from getenv calls
2. Add explicit validation for all configs
3. Replace try/except pass with proper error handling
4. Remove all IF NOT EXISTS from SQL
5. Add startup validation scripts
6. Update project switching to require explicit selection

### Phase 3: Validation
1. Run compliance checker
2. Test all failure scenarios
3. Verify error messages
4. Confirm non-zero exit codes

## Exceptions

There are NO exceptions to this policy. Any perceived need for fallback behavior must be:
1. Documented as a critical issue
2. Approved by system architect
3. Implemented with explicit user consent
4. Logged as a policy violation

## Compliance Tracking

Systems must maintain compliance metrics:
- Fallback attempts (must be 0)
- Failed validations
- Missing configurations
- Schema creation attempts
- Project switching failures

## Related Policies

- [Documentation Organization Standards](../standards/DOCUMENTATION_ORGANIZATION_STANDARDS.md)
- [Emergency Commands](../EMERGENCY_COMMANDS.md)
- Error Handling Standards
- Security Configuration Policy
- Database Schema Management

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-07-16 | Claude/Greg | Initial policy creation |
| 1.1 | 2025-07-20 | Greg | Adapted for AI-Tools with specific examples |

## Approval

This policy is effective immediately upon commit to the repository.

---
**Policy Owner**: Greg  
**Technical Implementation**: Claude (Anthropic)  
**Enforcement**: Automated + Manual Review  
**AI-Tools Integration**: Mandatory for all managed projects