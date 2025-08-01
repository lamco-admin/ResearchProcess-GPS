# ResearchProcess-GPS Next Session Prompt
## Post-OpenAPI Implementation & Documentation Update
### Timestamp: 2025-08-01 03:30:00 EEST

---

## 🚨 START HERE - CRITICAL CONTEXT

**MANDATORY READING**:
1. **Session Handover**: `sessions/handovers/SESSION_HANDOVER_2025_08_01_0328_EEST.md`
2. **Master Plan v2.8**: `RESEARCHPROCESS_GPS_MASTER_PLAN_v2.8_2025_08_01_0325_EEST.md`
3. **CLAUDE.md**: Project-specific instructions and NO_FALLBACK_POLICY

**Current Status**:
- Phase 3: 98% complete (only optional rate limiting remains)
- OpenAPI: 100% implemented and working
- Entity count: Correctly documented as 18 (not 19)
- NO_FALLBACK_POLICY: Maintained at 100%
- All tests passing (153 tests)

---

## 🎯 PRIORITY TASKS FOR NEXT SESSION

### 1. Complete Phase 3 (OPTIONAL)
- [ ] Implement rate limiting with tower-governor
- [ ] Test rate limiting with concurrent requests
- [ ] Document rate limiting configuration

### 2. Begin Phase 4: Module System Design
Start with research and design before implementation:

- [ ] **Research WASM runtimes**:
  - Compare wasmtime vs wasmer for module sandboxing
  - Evaluate security models and resource limits
  - Check Rust ecosystem integration

- [ ] **Design module API**:
  - Define module manifest format
  - Design module communication protocol
  - Plan resource limits and capabilities
  - Create module lifecycle hooks

- [ ] **Create `rp-modules` crate structure**:
  ```bash
  cargo new --lib crates/rp-modules
  ```

- [ ] **Prototype simple module**:
  - Start with native Rust module
  - Create Research Log module as example
  - Test hot-reload capability

### 3. Documentation & Planning
- [ ] Create MODULE_SYSTEM_DESIGN.md with architectural decisions
- [ ] Document module security model
- [ ] Plan module registry structure

---

## 💻 ENVIRONMENT SETUP

### Database Connection
```bash
export DB_NAME=researchprocess_gps
export DB_USER=researchprocess_gps
export DB_PASSWORD=researchprocess_gps
export DB_HOST=192.168.10.90
export DB_PORT=5432
```

### Run Server
```bash
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server
```

### Test Endpoints
- Health: `http://localhost:8080/health`
- OpenAPI JSON: `http://localhost:8080/api-docs/openapi.json`
- Swagger UI: `http://localhost:8080/swagger-ui/`

---

## ⚠️ CRITICAL REMINDERS

1. **NO_FALLBACK_POLICY**: Maintain 100% compliance - no shortcuts!
2. **Entity Count**: 18 entities (Note is not an entity)
3. **Test First**: Run `cargo test` before any major changes
4. **Documentation**: Update docs as you implement

---

## 📊 PROJECT METRICS

- **Phase 1**: 100% complete ✅
- **Phase 2**: 100% complete ✅
- **Phase 3**: 98% complete (OpenAPI done, rate limiting optional)
- **Entities**: 18 implemented
- **Tests**: 153 passing
- **NO_FALLBACK**: 100% compliant

---

## 🔗 KEY REFERENCES

- **NO_FALLBACK_POLICY**: `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md`
- **Phase Details**: See Master Plan v2.8 for Phases 4-8
- **OpenAPI Patterns**: See recent handler files for annotation examples

---

## 📋 TECHNICAL DECISIONS NEEDED

From Phase 4 planning:
1. **WASM vs Native Modules**: Security vs performance trade-off
2. **Module Communication**: Direct function calls vs message passing
3. **Resource Limits**: CPU time, memory usage, I/O operations
4. **Module Packaging**: Single file vs directory structure

---

## 🚀 QUICK START COMMANDS

```bash
# Switch to project
psw research

# Check current status
git status

# Run tests
cargo test

# Build and run server
cargo build --bin rp-server
cargo run --bin rp-server

# Check OpenAPI
curl http://localhost:8080/api-docs/openapi.json | jq
```

---

Start by reading the session handover, then review the Master Plan v2.8 for Phase 4 details. 
Focus on module system research and design before jumping into implementation.