# NO_FALLBACK_POLICY Pattern Guide
## Best Practices and Examples from ResearchProcess-GPS
### Timestamp: 2025-08-01 02:30:00 EEST

---

## 🎯 OVERVIEW

This guide documents the successful patterns used to achieve 100% NO_FALLBACK_POLICY compliance in the ResearchProcess-GPS codebase. Use these patterns as reference for future development.

---

## 📐 CORE PRINCIPLES

### 1. Explicit Error Handling
Every operation that can fail MUST return a Result type and propagate errors up the call stack.

### 2. No Silent Failures
Never use unwrap_or, unwrap_or_default, or unwrap_or_else without a clear comment explaining why it's safe.

### 3. Required Data Must Be Required
If data is required for correct operation, fail fast rather than using defaults.

### 4. Named Constants Over Magic Numbers
All default values should be explicit constants with descriptive names.

---

## ✅ APPROVED PATTERNS

### Pattern 1: Explicit Constants for Defaults

**❌ WRONG:**
```rust
let page_size = params.page_size.unwrap_or(100);
let offset = params.offset.unwrap_or(0);
```

**✅ CORRECT:**
```rust
const DEFAULT_PAGE_SIZE: i32 = 100;
const DEFAULT_OFFSET: i32 = 0;

let page_size = params.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
let offset = params.offset.unwrap_or(DEFAULT_OFFSET);
```

### Pattern 2: Result-Returning Factory Methods

**❌ WRONG:**
```rust
impl DomainEvent {
    pub fn new(entity_id: EntityId, event_type: String) -> (Event, Metadata) {
        let metadata = Metadata::default(); // What if this fails?
        (Event { entity_id, event_type }, metadata)
    }
}
```

**✅ CORRECT:**
```rust
impl DomainEvent {
    pub fn new(entity_id: EntityId, event_type: String) -> Result<(Event, Metadata), Error> {
        let metadata = Metadata::new()?;
        Ok((Event { entity_id, event_type }, metadata))
    }
}
```

### Pattern 3: Fail on Missing Required Fields

**❌ WRONG:**
```rust
let name = data.name.unwrap_or_else(|| "Unknown".to_string());
```

**✅ CORRECT:**
```rust
let name = data.name.ok_or_else(|| {
    error!("Missing required field: name");
    ApiError::MissingRequiredField("name")
})?;
```

### Pattern 4: Documented Display Fallbacks

**✅ ACCEPTABLE (with comment):**
```rust
impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display functions are allowed to have fallbacks for UI purposes
        let name = self.display_name.as_ref().unwrap_or(&self.id.to_string());
        write!(f, "{}", name)
    }
}
```

### Pattern 5: Security Defaults with Comments

**✅ ACCEPTABLE (with comment):**
```rust
// Security: Default to denying access when permission is unknown
let has_permission = permissions.get(&resource).copied().unwrap_or(false);
```

### Pattern 6: Validated Parameters

**❌ WRONG:**
```rust
pub fn get_page(page: Option<i32>) -> Vec<Item> {
    let page = page.unwrap_or(1);
    // What if page is negative?
}
```

**✅ CORRECT:**
```rust
pub fn get_page(page: Option<i32>) -> Result<Vec<Item>, Error> {
    let page = page.unwrap_or(1);
    if page < 1 {
        return Err(Error::InvalidParameter("page must be positive"));
    }
    // ... rest of implementation
}
```

---

## 🚫 BANNED PATTERNS

### 1. Silent Swallowing of Errors
```rust
// NEVER DO THIS
if let Ok(value) = risky_operation() {
    value
} else {
    Default::default()
}
```

### 2. Optional Chaining Without Handling
```rust
// NEVER DO THIS
let result = obj.method()?.another_method()?;
// Without proper error context
```

### 3. Unwrap in Production Code
```rust
// NEVER DO THIS (except in tests)
let value = some_option.unwrap();
```

---

## 📋 MIGRATION CHECKLIST

When updating code to be NO_FALLBACK compliant:

1. **Search for unwrap_or patterns:**
   ```bash
   rg "unwrap_or" --type rust
   ```

2. **For each occurrence, ask:**
   - Is this a required field? → Use `ok_or_else` with error
   - Is this a safe default? → Use named constant with comment
   - Is this for display only? → Add comment explaining safety
   - Is this a security decision? → Document the choice

3. **Update function signatures:**
   - Change `fn foo() -> T` to `fn foo() -> Result<T, Error>`
   - Propagate errors up the call stack

4. **Add validation:**
   - Check parameters before use
   - Return errors for invalid states

5. **Test error paths:**
   - Write tests that trigger error conditions
   - Ensure errors are properly propagated

---

## 💡 REAL EXAMPLES FROM THE CODEBASE

### Example 1: Event Store Pagination
```rust
// Before:
let limit = options.limit.unwrap_or(1000);

// After:
const DEFAULT_LIST_LIMIT: i64 = 1000;
let limit = options.limit.unwrap_or(DEFAULT_LIST_LIMIT);
```

### Example 2: Factory Method Update
```rust
// Before:
pub fn theory_created(theory: Theory) -> (DomainEvent, EventMetadata) {
    let event = TheoryCreated { theory };
    (DomainEvent::TheoryCreated(event), EventMetadata::default())
}

// After:
pub fn theory_created(theory: Theory) -> Result<(DomainEvent, EventMetadata), DomainError> {
    let event = TheoryCreated { theory };
    let metadata = EventMetadata::new()?;
    Ok((DomainEvent::TheoryCreated(event), metadata))
}
```

### Example 3: Required Field Validation
```rust
// Before:
let title = form.title.unwrap_or_default();

// After:
let title = form.title.ok_or_else(|| {
    ApiError::ValidationError("Title is required".to_string())
})?;
```

---

## 🔍 VERIFICATION

To verify NO_FALLBACK compliance:

```bash
# Check for unwrap_or patterns
cargo clippy --all-targets -- -W clippy::all | grep unwrap_or

# Run all tests
cargo test

# Check specific files
rg "unwrap_or" path/to/file.rs
```

---

## 📚 REFERENCES

- **Policy Location**: `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md`
- **Completion Report**: `NO_FALLBACK_POLICY_COMPLETE_2025_08_01_0215_EEST.md`
- **Original Violations**: `NO_FALLBACK_POLICY_VIOLATIONS_2025_08_01_0140_EEST.md`

---

*This guide represents patterns proven successful in achieving 100% NO_FALLBACK compliance in ResearchProcess-GPS.*