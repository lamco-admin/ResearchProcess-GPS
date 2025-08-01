# NEXT SESSION PROMPT
## ResearchProcess-GPS - Critical Resource Issue Resolution
### Generated: 2025-08-01 11:45:00 EEST

---

## 🚨 CRITICAL CONTEXT - START HERE

You are continuing work on ResearchProcess-GPS where a CRITICAL resource exhaustion issue is blocking all module system development. The previous session discovered severe CPU and memory spikes during module compilation that make development impossible.

**MANDATORY READING**:
1. `/home/greg/ResearchProcess-GPS/COMPREHENSIVE_HANDOVER_2025_08_01_1145_EEST.md` - Full session context
2. `/home/greg/ResearchProcess-GPS/MODULE_COMPILATION_ISSUE_2025_08_01_1130_EEST.md` - Issue details
3. `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md` - ZERO tolerance policy

---

## 🔴 TOP PRIORITY ISSUE

### Resource Exhaustion During Module Compilation
- **Symptoms**: Xorg at 100% CPU, swap full (974MB/976MB)
- **Impact**: System becomes unresponsive, development impossible
- **Pattern**: Recurring from yesterday with native modules
- **NOT caused by**: WASM (predates WASM work)

**User's Assessment**: "there's no reason for it to be happening besides a flawed implementation of the entire module implementation"

---

## 📋 YOUR IMMEDIATE TASKS

### 1. INVESTIGATE AND FIX Resource Issue (CRITICAL)
```bash
# First, check system state
free -h
top -b -n 1 | head -20

# Then systematically examine the module system from the beginning
cd /home/greg/ResearchProcess-GPS
```

**Approach**:
- Start fresh examination of module architecture
- Look for fundamental design flaws
- Check for recursive dependencies
- Examine build configurations
- Consider if complete reimplementation is needed

### 2. DO NOT PROCEED with other tasks until fixed
The following are BLOCKED:
- WASM module testing
- Module SDK development
- Any other module system work

---

## ⚠️ CRITICAL REQUIREMENTS

### User Directives (ABSOLUTE)
1. **NO simplification** - Fix problems properly
2. **ALL errors must be fixed** - No exceptions
3. **NO workarounds** - Solve root causes
4. **NO_FALLBACK_POLICY** - Zero tolerance

### From User's Own Words
- "all warnings must be fixed. all errors must be fixed. we do not simplify tests or work around problems. we fix them before proceeding. in all cases."
- "absolutely not! we fix all errors before proceeding!!! always!!"

---

## 🎯 SUCCESS CRITERIA

1. **Module compilation works without resource spikes**
2. **All tests pass without system stress**
3. **Root cause identified and documented**
4. **Permanent fix implemented**

---

## 📁 CURRENT STATE

### Project Status
- Phase 4 (Module System): 45% complete but BLOCKED
- Research Log WASM module: Created but untested
- All compilation errors: Fixed
- Resource issue: BLOCKING EVERYTHING

### Key Paths
- Module system: `/home/greg/ResearchProcess-GPS/crates/rp-modules/`
- WASM module: `/home/greg/ResearchProcess-GPS/modules/research-log-wasm/`
- Tests: `/home/greg/ResearchProcess-GPS/crates/rp-modules/tests/`

---

## 🚀 BEGIN WITH

```bash
# Start by reading the comprehensive handover
cat /home/greg/ResearchProcess-GPS/COMPREHENSIVE_HANDOVER_2025_08_01_1145_EEST.md

# Then read the issue documentation
cat /home/greg/ResearchProcess-GPS/MODULE_COMPILATION_ISSUE_2025_08_01_1130_EEST.md

# Check current system state
free -h
ps aux | grep -E "cargo|rustc|rust-analyzer" | grep -v grep

# Then begin systematic investigation
```

---

## 🔑 REMEMBER

- This issue MUST be fixed before ANY other work
- The user has been clear: NO workarounds, fix it properly
- This is affecting the entire development environment
- Consider that the module system design itself may be flawed

---

*Your mission: Resolve the critical resource exhaustion issue that is blocking ResearchProcess-GPS module system development. Nothing else matters until this is fixed.*