# 🚨 START HERE - CRITICAL CONTEXT FOR RESEARCHPROCESS-GPS SESSION
## Session Initialization: 2025-08-01 11:50:00 EEST
## CRITICAL: Resource Exhaustion Issue BLOCKING All Development

---

## 🔴 MANDATORY INITIAL READING SEQUENCE

You MUST read these documents IN THIS EXACT ORDER before taking any action:

### 1. Project Instructions & Standards
```bash
cat /home/greg/ResearchProcess-GPS/CLAUDE.md
```
**Pay special attention to**:
- NO_FALLBACK_POLICY (ZERO TOLERANCE)
- Timestamp requirements for documentation
- Authority Matrix location

### 2. NO_FALLBACK_POLICY Details
```bash
cat /home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md
```
**Remember**: Zero tolerance for silent failures, ALL errors must be handled explicitly

### 3. Comprehensive Session Handover
```bash
cat /home/greg/ResearchProcess-GPS/COMPREHENSIVE_HANDOVER_2025_08_01_1145_EEST.md
```
**Critical**: Contains details about the BLOCKING resource exhaustion issue

### 4. Issue Documentation
```bash
cat /home/greg/ResearchProcess-GPS/MODULE_COMPILATION_ISSUE_2025_08_01_1130_EEST.md
```
**Focus on**: Symptoms, patterns, and previous debugging attempts

### 5. Current Master Plan
```bash
cat /home/greg/ResearchProcess-GPS/RESEARCHPROCESS_GPS_MASTER_PLAN_v3.0_2025_08_01_1145_EEST.md
```
**Note**: Phase 4 is 45% complete but BLOCKED by critical issue

### 6. Module System Design
```bash
cat /home/greg/ResearchProcess-GPS/docs/MODULE_DESIGN_2025_07_31_2212_EEST.md
```
**Understand**: The architecture that may have fundamental flaws

---

## 🚨 CRITICAL ISSUE SUMMARY

### Resource Exhaustion During Module Compilation
**THIS MUST BE FIXED BEFORE ANY OTHER WORK**

**Symptoms**:
- Xorg process consuming 100% CPU
- Swap memory nearly full (974.9MB/976MB)
- System becomes unresponsive
- Makes development impossible

**Key Facts**:
- Pattern recurring from yesterday's native module work
- NOT caused by WASM (predates WASM implementation)
- User assessment: "flawed implementation of the entire module implementation"
- Blocks ALL Phase 4 progress

---

## 📋 USER DIRECTIVES (ABSOLUTE REQUIREMENTS)

From previous session, the user was explicit:

1. **"all warnings must be fixed. all errors must be fixed. we do not simplify tests or work around problems. we fix them before proceeding. in all cases."**

2. **"absolutely not! we fix all errors before proceeding!!! always!! all errors and warning must be fixed immediately."**

3. **"there's no reason for it to be happening besides a flawed implementation of the entire module implementation"**

4. **NO simplification, NO workarounds, NO degraded operation**

---

## 🎯 YOUR MISSION

### Priority 1: FIX THE RESOURCE EXHAUSTION ISSUE
Nothing else matters until this is resolved.

### Approach:
1. **Start Fresh** - Systematically examine from the beginning
2. **Consider Fundamental Flaws** - The architecture itself may be wrong
3. **No Band-Aids** - Fix the root cause, not symptoms
4. **Document Everything** - Track your investigation thoroughly

### What NOT to do:
- ❌ DO NOT proceed with WASM testing
- ❌ DO NOT continue SDK development
- ❌ DO NOT work around the issue
- ❌ DO NOT simplify or skip anything

---

## 🔧 TECHNICAL CONTEXT

### Current State:
- **Working Directory**: `/home/greg/ResearchProcess-GPS`
- **Module System**: `/home/greg/ResearchProcess-GPS/crates/rp-modules/`
- **WASM Module**: `/home/greg/ResearchProcess-GPS/modules/research-log-wasm/`
- **Tests**: `/home/greg/ResearchProcess-GPS/crates/rp-modules/tests/`

### What Was Completed:
- ✅ Research Log WASM module created
- ✅ All compilation errors fixed
- ✅ NO_FALLBACK_POLICY compliance achieved
- ❌ Cannot test due to resource issue

### Module Architecture:
- Native modules via libloading
- WASM modules via Wasmtime 25.0
- Message-passing communication
- Capability-based security
- Resource limiting framework

---

## 🚀 INITIAL ACTIONS

After reading all documents above, begin with:

```bash
# Check current system state
free -h
ps aux | grep -E "cargo|rustc|rust-analyzer" | grep -v grep
top -b -n 1 | head -20

# Navigate to project
cd /home/greg/ResearchProcess-GPS

# Check for any running processes
lsof | grep -i cargo || echo "No cargo processes found"
lsof | grep -i rust || echo "No rust processes found"

# Then begin systematic investigation of the module system
```

---

## 📊 SUCCESS CRITERIA

You will know the issue is fixed when:
1. Module compilation completes without CPU/memory spikes
2. All tests can run without system stress
3. Development can proceed normally
4. Root cause is identified and documented

---

## ⚠️ FINAL REMINDERS

1. **This is a BLOCKING issue** - Nothing else can proceed until fixed
2. **User has been clear** - No workarounds, fix it properly
3. **Pattern is recurring** - This happened yesterday too
4. **Consider redesign** - The entire module system may be flawed
5. **Follow NO_FALLBACK_POLICY** - Always, without exception

---

## 📝 WHEN YOU'RE READY

Once you've read all the documents and understand the context:

1. Acknowledge that you've read and understood the critical issue
2. Begin systematic investigation from first principles
3. Consider if the module system architecture itself is fundamentally flawed
4. Document your findings as you go
5. Implement a proper fix, not a workaround

**Remember**: The user's patience is limited. This issue has been blocking development across multiple sessions. A real solution is needed, not patches.

---

*Your expertise is needed to unblock ResearchProcess-GPS development. The module system may need fundamental redesign. Proceed with careful analysis and decisive action.*