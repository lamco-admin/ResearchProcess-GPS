# Technical Debt: Build System Optimization
## Dependency Management and Compilation Performance
### Timestamp: 2025-08-01 12:43:00 EEST

---

## 📊 DEBT SUMMARY

**Type**: Performance / Architecture  
**Priority**: Medium (becomes High when builds exceed 15 minutes)  
**Impact**: Developer productivity, CI/CD costs  
**Effort**: Medium to High depending on approach  
**Risk**: Low to Medium (feature flag complexity)  

### Current State
- Monolithic dependency tree with all features always included
- 200+ crates compile regardless of what's being developed
- Storage backends included but not implemented
- Both native and WASM module systems always compiled
- Build times: 5-10 minutes clean, 1-2 minutes incremental

### Desired State
- Modular build with feature flags
- Core functionality builds in <1 minute
- Optional features loaded on demand
- Clear dependency boundaries
- Optimized CI/CD pipelines

---

## 🔍 DETAILED ANALYSIS

### 1. Storage Layer Bloat

**Current Issues**:
```
rp-storage/           # Abstract interface
rp-storage-postgres/  # ✅ Implemented and used
rp-storage-git/       # ❌ Placeholder, but dependencies loaded
rp-storage-fs/        # ❌ Placeholder, but dependencies loaded
```

**Impact**:
- Unnecessary database drivers
- File system abstractions
- Git libraries (libgit2)
- ~20-30 extra crates

**Proposed Solution**:
```toml
[workspace.dependencies]
rp-storage-postgres = { path = "../rp-storage-postgres", optional = true }
rp-storage-git = { path = "../rp-storage-git", optional = true }
rp-storage-fs = { path = "../rp-storage-fs", optional = true }

[features]
default = ["storage-postgres"]
storage-postgres = ["dep:rp-storage-postgres", "sqlx/postgres"]
storage-git = ["dep:rp-storage-git", "git2"]
storage-fs = ["dep:rp-storage-fs"]
all-storage = ["storage-postgres", "storage-git", "storage-fs"]
```

### 2. Module System Dependencies

**Current Issues**:
- Wasmtime (100+ crates) always included
- libloading always included
- WASI preview1 full support
- Cranelift JIT compiler

**Impact**:
- Longest compilation component
- 2-3 minutes of build time
- Large binary size
- Memory usage during compilation

**Proposed Solution**:
```toml
[dependencies.wasmtime]
version = "25.0"
optional = true

[dependencies.libloading]
version = "0.8"
optional = true

[features]
default = ["native-modules"]
native-modules = ["libloading"]
wasm-modules = ["wasmtime", "wasmtime-wasi"]
all-modules = ["native-modules", "wasm-modules"]
```

### 3. Tokio Feature Sprawl

**Current Issues**:
```toml
tokio = { version = "1.40", features = ["full"] }
```

**Should Be**:
```toml
tokio = { version = "1.40", features = [
    "rt-multi-thread",
    "macros",
    "sync",
    "time",
    "signal",
    "net",
    # Add only what's needed
]}
```

### 4. Development vs Production Dependencies

**Current Issues**:
- All dev dependencies compiled for every crate
- Test utilities in production builds
- Debug symbols in release builds

**Proposed Solution**:
- Separate test utilities crate
- Profile-specific dependencies
- Conditional compilation

---

## 📋 IMPLEMENTATION ROADMAP

### Phase 1: Quick Wins (1-2 days)
1. **Tokio features reduction**
   - Audit actual usage
   - Remove "full" feature
   - Specify only needed features

2. **Dev profile optimization**
   ```toml
   [profile.dev-quick]
   inherits = "dev"
   opt-level = 0
   debug = 0  # No debug symbols
   ```

3. **Parallel compilation**
   ```toml
   [profile.dev]
   split-debuginfo = "unpacked"
   ```

### Phase 2: Feature Flags (1 week)
1. **Storage backend features**
   - Make postgres default
   - Other backends optional
   - Update CI/CD

2. **Module system features**
   - Separate native/WASM
   - Optional by default
   - Feature-gated tests

3. **Documentation**
   - Feature flag guide
   - Build optimization docs
   - CI/CD updates

### Phase 3: Architectural Changes (2-4 weeks)
1. **Workspace restructuring**
   - Core workspace (minimal deps)
   - Modules workspace (heavy deps)
   - Tools workspace (CLI, etc)

2. **Dynamic loading investigation**
   - Storage backends as plugins
   - Module systems as services
   - Runtime capability negotiation

3. **Build caching infrastructure**
   - sccache setup
   - Docker layer optimization
   - Dependency pre-building

---

## 🎯 METRICS

### Success Criteria
- Core build time < 1 minute
- Incremental builds < 30 seconds
- CI/CD time reduced by 50%
- Binary size reduced by 30%

### Monitoring
- Track build times per profile
- Measure developer productivity
- Monitor CI/CD costs
- Binary size tracking

---

## ⚠️ RISKS

### Technical Risks
1. **Feature flag complexity**
   - Combinatorial testing burden
   - Runtime failures
   - Documentation overhead

2. **Breaking changes**
   - API compatibility
   - Build script updates
   - Developer workflow changes

3. **Hidden dependencies**
   - Transitive feature requirements
   - Platform-specific issues
   - Optional dependency conflicts

### Mitigation Strategies
1. **Gradual rollout**
   - Start with storage backends
   - Test thoroughly
   - Document everything

2. **Compatibility layer**
   - Keep "all-features" option
   - Default to current behavior
   - Opt-in optimization

3. **Automated testing**
   - Feature combination matrix
   - CI/CD validation
   - Performance regression tests

---

## 💰 COST/BENEFIT

### Costs
- Implementation time: 2-4 weeks
- Testing overhead: Ongoing
- Documentation: 1 week
- Training: Minimal

### Benefits
- Developer time saved: 2-5 minutes per build
- CI/CD cost reduction: ~50%
- Better architecture: Priceless
- Faster onboarding: New devs can build core quickly

### ROI Calculation
- Current: 10 builds/day × 5 min = 50 min/day wasted
- Optimized: 10 builds/day × 1 min = 10 min/day
- Savings: 40 min/day × 20 days = 13 hours/month

---

## 📝 RECOMMENDATIONS

### Immediate Actions
1. **Document current dependencies**
   - Run `cargo tree --duplicates`
   - Identify unused features
   - Create dependency graph

2. **Set thresholds**
   - Alert when build > 10 min
   - Track build time trends
   - Monitor dependency count

3. **Prototype one feature flag**
   - Start with storage backends
   - Measure impact
   - Refine approach

### Long-term Strategy
1. **Adopt plugin architecture**
   - Core remains stable
   - Features as plugins
   - Runtime composition

2. **Invest in tooling**
   - Build performance dashboard
   - Automated dependency audits
   - Feature flag management

3. **Consider alternatives**
   - Microservices for heavy features
   - WASM for all modules
   - Dynamic linking strategies

---

## 🔄 REVIEW SCHEDULE

- **Monthly**: Review build times and adjust priority
- **Quarterly**: Evaluate architectural changes
- **Yearly**: Major refactoring consideration

---

*This technical debt is acknowledged and tracked. Action will be taken when build times exceed acceptable thresholds or development velocity is significantly impacted.*