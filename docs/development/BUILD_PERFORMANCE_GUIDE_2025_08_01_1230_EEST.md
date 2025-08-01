# Build Performance Guide
## ResearchProcess-GPS Development
### Timestamp: 2025-08-01 12:30:00 EEST

---

## 🎯 PURPOSE

This guide documents build performance characteristics and optimization strategies for ResearchProcess-GPS, particularly focusing on the module system which has the heaviest dependencies.

---

## 📊 BUILD CHARACTERISTICS

### Dependency Overview

The ResearchProcess-GPS project includes several heavy dependencies that significantly impact build times:

1. **Wasmtime (25.0)**
   - Full WebAssembly runtime
   - Cranelift JIT compiler
   - ~100+ transitive dependencies

2. **SQLx**
   - Compile-time checked SQL
   - PostgreSQL driver
   - Async runtime integration

3. **Cryptographic Libraries**
   - ring (crypto primitives)
   - rustls (TLS implementation)
   - Various hash/signature libraries

4. **Unicode/Internationalization**
   - ICU libraries
   - Unicode normalization
   - Locale support

### Expected Build Times

| Build Type | Clean Build | Incremental | Notes |
|------------|-------------|-------------|--------|
| `cargo check` | 30-60s | 5-10s | Type checking only |
| `cargo build` | 3-5 min | 15-30s | Debug build |
| `cargo build --release` | 5-10 min | 30-60s | Optimized build |
| `cargo test --no-run` | 5-10 min | 20-40s | Compile tests |
| Full test suite | 10-15 min | 1-2 min | Compile + run |

*Times measured on 8-core VM with 16GB RAM*

---

## 🚀 OPTIMIZATION STRATEGIES

### 1. Environment Configuration

```bash
# Limit parallel jobs on constrained systems
export CARGO_BUILD_JOBS=2  # Adjust based on available cores

# Disable incremental compilation if having issues
export CARGO_INCREMENTAL=0

# Use faster linker (if available)
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

### 2. Build Profiles

Already configured in workspace `Cargo.toml`:

```toml
[profile.dev]
opt-level = 0
debug = 1
split-debuginfo = "unpacked"
incremental = false

[profile.test]
opt-level = 1
debug = 1
incremental = false

[profile.release]
opt-level = "z"
lto = false  # Disabled to reduce memory usage
codegen-units = 16
```

### 3. Staged Building

For resource-constrained environments:

```bash
# Step 1: Build dependencies only
cargo build --package rp-modules --lib

# Step 2: Build tests separately
cargo test --package rp-modules --no-run

# Step 3: Run tests
cargo test --package rp-modules
```

### 4. Caching Strategies

#### sccache Installation
```bash
# Install sccache for build caching
cargo install sccache

# Configure
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=~/.cache/sccache
export SCCACHE_CACHE_SIZE="10G"
```

#### cargo-cache Management
```bash
# Install cargo-cache
cargo install cargo-cache

# Clean old artifacts
cargo cache -a  # Show cache info
cargo cache -r all  # Clean everything
cargo cache -r registry-sources  # Clean source archives
```

---

## 🖥️ VM-SPECIFIC CONSIDERATIONS

### X11 Forwarding Impact

When building over X11 forwarding (e.g., NoMachine, VNC):

1. **High Xorg CPU Usage**
   - Normal when terminal shows build output
   - NOT indicative of actual system stress
   - Consider redirecting output: `cargo build > build.log 2>&1`

2. **Mitigation Strategies**
   ```bash
   # Build with minimal output
   cargo build --quiet
   
   # Or use a multiplexer
   tmux new-session -d 'cargo build'
   tmux attach
   ```

### Proxmox Considerations

- Resources scale dynamically
- Don't judge by momentary spikes
- Swap usage is normal for large builds
- Consider build containers with resource limits

---

## 🔧 TROUBLESHOOTING

### Symptom: Build Appears Hung

**Reality**: Likely just slow compilation

**Solutions**:
1. Check `ps aux | grep cargo` for activity
2. Monitor `~/.cargo` directory for file changes
3. Use `cargo build -vv` for detailed output
4. Extend timeouts in CI/CD

### Symptom: Out of Memory

**Solutions**:
1. Reduce parallel jobs: `CARGO_BUILD_JOBS=1`
2. Disable LTO in release builds
3. Increase swap space temporarily
4. Use `cargo check` instead of full builds

### Symptom: Xorg High CPU

**Reality**: Terminal rendering overhead

**Solutions**:
1. Build in tmux/screen session
2. Redirect output to file
3. Use SSH instead of GUI forwarding
4. Build with `--quiet` flag

---

## 📈 PERFORMANCE MONITORING

### Build Time Tracking

```bash
# Time your builds
time cargo build --package rp-modules

# Use cargo's built-in timings
cargo build --timings
# Opens HTML report showing bottlenecks
```

### Resource Monitoring

```bash
# Monitor during build
watch -n 1 'free -h; echo; ps aux | grep cargo | grep -v grep'

# Or use htop in another terminal
htop -d 10
```

---

## 🎯 BEST PRACTICES

### For Daily Development

1. **Use cargo check frequently**
   - Catches most errors
   - 10x faster than full build
   - Run before commits

2. **Incremental Testing**
   ```bash
   # Test only what changed
   cargo test --package rp-modules specific_test_name
   ```

3. **Feature Flags**
   ```bash
   # Build without WASM when not needed
   cargo build --no-default-features
   ```

### For CI/CD

1. **Cache Configuration**
   ```yaml
   cache:
     key: ${CI_COMMIT_REF_SLUG}
     paths:
       - target/
       - ~/.cargo/registry/
       - ~/.cargo/git/
   ```

2. **Timeout Settings**
   ```yaml
   build:
     timeout: 20 minutes  # Generous for first builds
   ```

3. **Parallel Jobs**
   ```yaml
   variables:
     CARGO_BUILD_JOBS: "4"  # Adjust for CI runner
   ```

---

## 📚 REFERENCES

- [Cargo Build Performance](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Incident Report: Compilation Resource Usage](./incidents/COMPILATION_RESOURCE_INCIDENT_2025_08_01_1228_EEST.md)
- [Rust Compilation Speed Tips](https://endler.dev/2020/rust-compile-times/)

---

*Last Updated: 2025-08-01  
ResearchProcess-GPS Development Team*