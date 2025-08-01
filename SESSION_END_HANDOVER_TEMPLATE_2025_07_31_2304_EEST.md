# Session End Handover Template

### Timestamp: 2025-07-31 23:04:51 EEST

### Purpose: Standardized prompt for ending development sessions

---

## 🎯 OPTIMIZED SESSION END PROMPT

Copy and use this prompt at the end of each session:

```
Please perform a comprehensive session handover following these steps:

1. CHECK CURRENT TIME: Run `date "+%Y-%m-%d %H:%M:%S %Z"` before creating any documents

2. UPDATE MASTER PLAN:
   - Copy RESEARCHPROCESS_GPS_MASTER_PLAN_v*.md to new version with current timestamp
   - Update Phase Progress percentages based on today's work
   - Add any Architecture Decisions made this session
   - Update Entity Implementation Matrix if entities were modified
   - Note any deferred work in Deferred Work Log
   - Update Change Log with version and summary
   - Archive the previous version in archive/plans/

3. CREATE COMPREHENSIVE HANDOVER:
   - Filename: COMPREHENSIVE_HANDOVER_[YYYY_MM_DD_HHMM_TIMEZONE].md
   - Include all sections from the handover you started with
   - Update implementation status with today's accomplishments
   - Document any technical decisions or discoveries
   - List specific next tasks with clear instructions
   - Include any workarounds or important context

4. GENERATE NEXT SESSION PROMPT:
   - Create NEXT_SESSION_PROMPT_[YYYY_MM_DD_HHMM_TIMEZONE].md
   - Include instruction to read the comprehensive handover FIRST
   - Reference the updated Master Plan
   - List the specific next tasks to tackle
   - Include all critical context (DB connection, ports, etc.)
   - reproduce the prompt in the chat so opening the file isn't necessary

5.  GIT OPERATIONS:
   - Stage all changes: git add -A
   - Create detailed commit message summarizing session work
   - Commit with: git commit -m "[message]"
   - Push to origin: git push

Remember to maintain NO_FALLBACK_POLICY throughout all work.
```

---

## 📋 ENHANCED CHECKLIST VERSION

For more detailed control, use this expanded version:

```
Please complete the following session end tasks:

## 1. TIMESTAMP CHECK
- [ ] Run: date "+%Y-%m-%d %H:%M:%S %Z"
- [ ] Use this timestamp in all document names

## 2. MASTER PLAN UPDATE
- [ ] Copy current RESEARCHPROCESS_GPS_MASTER_PLAN to new timestamped version
- [ ] Update sections:
  - [ ] Phase completion percentages
  - [ ] Architecture Decisions Log (if any decisions made)
  - [ ] Entity Implementation Matrix (if entities changed)
  - [ ] Deferred Work Log (if anything skipped/postponed)
  - [ ] Technical Specifications (if APIs/schemas changed)
  - [ ] Change Log entry for this version
- [ ] Archive previous version in archive/plans/

## 3. COMPREHENSIVE HANDOVER
- [ ] Create COMPREHENSIVE_HANDOVER_[timestamp].md with:
  - [ ] Current project state summary
  - [ ] Today's accomplishments (detailed list)
  - [ ] Technical decisions and rationale
  - [ ] Known issues and workarounds
  - [ ] Specific next tasks (numbered list)
  - [ ] Key file modifications
  - [ ] Test status and results
  - [ ] Critical context for next session

## 4. ARCHIVE OLD DOCUMENTS
- [ ] Move outdated handovers to archive/deprecated/handovers/
- [ ] Move old summaries to archive/deprecated/summaries/
- [ ] Keep only the latest comprehensive handover

## 5. GIT COMMIT & PUSH
- [ ] git add -A
- [ ] git commit -m "feat/fix/docs: [description of session work]

      - List key accomplishments
      - Note any important changes

      Session handover: COMPREHENSIVE_HANDOVER_[timestamp].md"
- [ ] git push

## 6. NEXT SESSION PROMPT
- [ ] Create NEXT_SESSION_PROMPT_[timestamp].md with:
  - [ ] "CRITICAL: Start by reading COMPREHENSIVE_HANDOVER_[timestamp].md"
  - [ ] "Then read RESEARCHPROCESS_GPS_MASTER_PLAN_v[X.X]_[timestamp].md"
  - [ ] Current phase and completion percentage
  - [ ] Specific next tasks from handover
  - [ ] Technical context (DB: 192.168.10.90, Port: 8080, etc.)
  - [ ] NO_FALLBACK_POLICY reminder

## 7. FINAL SUMMARY
- [ ] Provide brief summary of session accomplishments
- [ ] List any blockers or concerns
- [ ] Confirm all documents created with timestamps
```

---

## 🚀 QUICK VERSION

For routine sessions with no major changes:

```
End this session with:
1. Update Master Plan (version, progress, change log)
2. Create timestamped comprehensive handover
3. Archive old docs
4. Git commit and push everything
5. Generate next session prompt

Use timestamps in all filenames (check current time first).
```

---

## 📝 KEY IMPROVEMENTS

This template improves on the original by:

1. **Master Plan Updates**: Ensures the living document stays current
2. **Structured Sections**: Clear checklist prevents missing items
3. **Archive Management**: Keeps workspace clean
4. **Version Control**: Proper version numbering for Master Plan
5. **Timestamp Enforcement**: Explicit reminder to check time first
6. **NO_FALLBACK_POLICY**: Included in reminders

---

## 🔧 CUSTOMIZATION

Adjust based on session type:

- **Major Feature**: Use enhanced checklist
- **Bug Fixes**: Use quick version
- **Architecture Changes**: Definitely update Architecture Decisions Log
- **Entity Changes**: Update Entity Implementation Matrix

---

*Save this template and customize as needed for your workflow.*