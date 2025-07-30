# Label Governance Standards

**Status**: MANDATORY
**Version**: 1.0
**Last Updated**: 2025-01-22
**Enforcement**: STRICT - NO EXCEPTIONS

## 🚨 CRITICAL RULES

1. **NO LABEL CREATION** - Only approved labels may be used
2. **NO CUSTOM LABELS** - All labels must be from approved list
3. **NO LABEL VARIATIONS** - Use exact spelling and case
4. **FAIL ON VIOLATION** - Any unapproved label = build failure

## Approved Labels

### Priority Labels (Mutually Exclusive)
- `priority:critical` - System down, data loss risk
- `priority:high` - Major feature blocked
- `priority:medium` - Normal priority
- `priority:low` - Nice to have

### Type Labels (Mutually Exclusive)
- `type:bug` - Something broken
- `type:feature` - New functionality
- `type:enhancement` - Improvement to existing
- `type:documentation` - Docs only
- `type:security` - Security issue
- `type:performance` - Performance issue
- `type:refactor` - Code cleanup
- `type:test` - Test only changes

### Status Labels (Mutually Exclusive)
- `status:ready` - Ready to work on
- `status:in-progress` - Being worked on
- `status:blocked` - Blocked by dependency
- `status:review` - In review
- `status:done` - Completed

### Component Labels (Multiple Allowed)
- `component:sync` - Sync infrastructure
- `component:kp` - Knowledge Persistence
- `component:ai` - AI features
- `component:api` - API related
- `component:ui` - UI/Frontend
- `component:db` - Database
- `component:docs` - Documentation

### Special Labels
- `good-first-issue` - Good for newcomers
- `help-wanted` - Need help
- `wontfix` - Will not fix
- `duplicate` - Duplicate issue

## Label Mapping

### GitHub to Redmine Tracker
```json
{
  "type:bug": 1,
  "type:feature": 2,
  "type:enhancement": 2,
  "type:documentation": 3,
  "type:security": 1,
  "type:performance": 1
}
```

### GitHub to Redmine Priority
```json
{
  "priority:critical": 4,
  "priority:high": 3,
  "priority:medium": 2,
  "priority:low": 1
}
```

## Enforcement Mechanisms

### 1. Pre-commit Hook
All commits creating issues MUST pass label validation

### 2. CI/CD Check
All PRs with label changes MUST pass validation

### 3. API Validation
All API calls creating issues MUST validate labels

### 4. Sync Validation
Sync scripts MUST only use approved labels

## NO FALLBACK POLICY

- **Invalid label = ERROR** - No silent failures
- **Missing type = ERROR** - Type label required
- **Multiple types = ERROR** - Only one type allowed
- **Case mismatch = ERROR** - Exact case required

## Examples

### ✅ CORRECT
```yaml
labels:
  - type:bug
  - priority:high
  - component:sync
```

### ❌ WRONG
```yaml
labels:
  - bug              # Missing prefix
  - Bug              # Wrong case
  - type:bugfix      # Not approved
  - urgent           # Not approved
  - Priority:High    # Wrong case
```

## Implementation

All tools MUST:
1. Load approved labels from this file
2. Validate before any operation
3. Fail loudly on violation
4. Never create new labels
5. Never accept variations

## Audit Trail

All label violations MUST be:
1. Logged with timestamp
2. Reported to user
3. Blocked from proceeding
4. Tracked for patterns