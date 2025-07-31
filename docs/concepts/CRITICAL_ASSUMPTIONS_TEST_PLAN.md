# Critical Assumptions Test Plan

## Overview

This document outlines specific tests to validate critical assumptions about the GEDCOM 7 extensions before full implementation. Each test includes methodology, success criteria, and risk assessment.

## Test 1: Citation Template Portability

### Assumption
Citation templates with bash-style syntax will solve formatting issues across systems.

### Test Design

#### Phase 1: Template Creation
```yaml
# Create test template (Census example)
uri: https://test.gedcom.io/templates/Census
sfields:
  - variable: Year
    label: Census Year
  - variable: Jurisdiction  
    label: Location
dfields:
  - variable: Page
    label: Page Number
  - variable: Line
    label: Line Number
formats:
  - name: Evidence Explained
    titl: ${Year} ${Jurisdiction} Census
    scite: ${Year} ${Jurisdiction} Census
    page: ${Page:+page ${Page}}${Line:+, line ${Line}}
```

#### Phase 2: System Testing

**Test A: RootsMagic Export**
1. Create source using template in mock-up
2. Export to GEDCOM with extension
3. Verify structure:
   ```gedcom
   0 @S1@ SOUR
   1 _TPLT https://test.gedcom.io/templates/Census
   2 _FIEL Year
   3 TEXT 1850
   2 _FIEL Jurisdiction
   3 TEXT Boston, MA
   ```

**Test B: Cross-System Import**
1. Import into FTM simulator
2. Check if citation elements preserved
3. Test formatting output

**Test C: Professional Review**
1. Generate 10 different citation types
2. Submit to professional genealogist
3. Evaluate against Evidence Explained standards

### Success Criteria
- ✓ Templates parse correctly (no syntax errors)
- ✓ Elements round-trip without data loss
- ✓ Formatted output matches professional standards
- ✓ Performance impact < 10ms per citation

### Risk Assessment
- **High Risk**: Bash syntax too complex for vendors
- **Mitigation**: Consider simpler syntax alternatives
- **Fallback**: Use simple key-value pairs without conditionals

## Test 2: Event Independence Performance

### Assumption
Independent events improve data quality without significant performance/size penalties.

### Test Design

#### Test Scenario: 1850 Census with 50 People

**Traditional GEDCOM**:
```gedcom
# 50 individuals, each with census event
0 @I1@ INDI
1 CENS
2 DATE 1850
2 PLAC Ward 5, Boston
# ... repeated 50 times
```

**With Event Independence**:
```gedcom
# One event
0 @E1@ _EVENT
1 TYPE Census  
1 DATE 1850
1 PLAC Ward 5, Boston
1 _PART @I1@
2 ROLE Head
# ... 49 more participants

# 50 individuals with references
0 @I1@ INDI
1 _EVREF @E1@
2 ROLE Head
```

#### Metrics to Measure

1. **File Size Comparison**
   ```python
   traditional_size = measure_gedcom_size(traditional)
   independent_size = measure_gedcom_size(independent)
   overhead = (independent_size - traditional_size) / traditional_size
   ```

2. **Query Performance**
   - Find all people in census: time both approaches
   - Update census date: measure update time
   - Add new participant: measure insertion time

3. **Memory Usage**
   - Load both files into parser
   - Measure peak memory usage
   - Test with 1000, 5000, 10000 people

### Success Criteria
- ✓ File size overhead < 20%
- ✓ Query performance within 2x
- ✓ Memory usage scales linearly
- ✓ Update operations faster with independent events

### Risk Assessment
- **Medium Risk**: File size increase unacceptable
- **Mitigation**: Compression recommendations
- **Fallback**: Hybrid approach for large events only

## Test 3: Evidence Container GPS Compliance

### Assumption
Evidence containers can represent all GPS (Genealogical Proof Standard) elements.

### Test Design

#### Complex Proof Argument Test

**Scenario**: Proving parentage through indirect evidence

1. **Evidence Collection**
   ```gedcom
   # Evidence 1: Census showing household
   0 @E1@ _EVID
   1 _TYPE Person
   1 _INFO John Smith household
   2 _ROLE Description
   1 _INFO Mary Smith, age 10, daughter
   2 _ROLE Relationship
   1 SOUR @S1@
   2 PAGE 1850 Census, p. 247
   
   # Evidence 2: Church record
   0 @E2@ _EVID
   1 _TYPE Event
   1 _INFO Mary Smith baptism
   2 _ROLE Description
   1 _INFO Parents: John and Jane Smith
   2 _ROLE Relationship
   1 SOUR @S2@
   ```

2. **Analysis Documentation**
   ```gedcom
   0 @R1@ _RDOC
   1 _TITL Proof of Mary Smith's Parentage
   1 _TEXT The 1850 census shows Mary Smith, age 10...
   2 CONT The baptism record from 1840 confirms...
   1 _EVID @E1@
   1 _EVID @E2@
   1 _CONC
   2 _SUBJ @I1@  # Mary Smith
   2 _TEXT Mary Smith is the daughter of John and Jane
   ```

3. **GPS Elements Checklist**
   - [ ] Reasonably exhaustive research
   - [ ] Complete citation of sources  
   - [ ] Analysis and correlation
   - [ ] Resolution of conflicts
   - [ ] Written proof summary

### Success Criteria
- ✓ All GPS elements representable
- ✓ Clear separation of evidence/analysis/conclusion
- ✓ Machine-readable for validation
- ✓ Understandable by genealogists

### Risk Assessment
- **High Risk**: Too complex for average users
- **Mitigation**: Graduated complexity levels
- **Fallback**: Simplified evidence linking only

## Test 4: Vendor Implementation Feasibility

### Assumption
Vendors can implement extensions without prohibitive cost.

### Test Design

#### Prototype Implementation

1. **Minimal Parser** (Python)
   ```python
   class GedcomExtensionParser:
       def parse_citation_template(self, node):
           # Time implementation effort
           pass
           
       def parse_event_independence(self, node):
           # Measure complexity
           pass
           
       def parse_evidence_container(self, node):
           # Assess difficulty
           pass
   ```

2. **Complexity Metrics**
   - Lines of code required
   - Development time estimate
   - Test coverage needs
   - Documentation requirements

3. **Vendor Survey**
   - Send spec to 5 vendors
   - Ask for implementation estimate
   - Identify major concerns
   - Gather improvement suggestions

### Success Criteria
- ✓ Implementation < 40 hours per extension
- ✓ No architectural changes required
- ✓ 3+ vendors express interest
- ✓ Clear migration path identified

### Risk Assessment
- **High Risk**: Vendors see no ROI
- **Mitigation**: User demand demonstration
- **Fallback**: Open source reference implementation

## Test 5: User Acceptance

### Assumption
Professional genealogists will adopt and use these extensions.

### Test Design

1. **Focus Groups**
   - 5 professional genealogists
   - 10 serious researchers
   - 15 casual users

2. **Usability Testing**
   - Create sample files
   - Task-based evaluation
   - Time to complete tasks
   - Error rate measurement

3. **Feature Priority Survey**
   - Which extensions most valuable?
   - What's missing?
   - What's too complex?
   - Integration preferences?

### Success Criteria
- ✓ 80% find value in extensions
- ✓ No major usability issues
- ✓ Clear priority order emerges
- ✓ Professionals endorse approach

## Implementation Timeline

### Week 1-2: Test Preparation
- Set up test environments
- Create test data
- Recruit participants
- Develop metrics tools

### Week 3-4: Execute Tests
- Run technical tests
- Conduct user sessions
- Gather vendor feedback
- Document results

### Week 5-6: Analysis & Decisions
- Analyze results
- Identify needed changes
- Make go/no-go decisions
- Plan modifications

## Decision Matrix

| Test | Pass | Partial | Fail |
|------|------|---------|------|
| Citation Templates | Proceed | Simplify syntax | Use key-value only |
| Event Independence | Proceed | Optimize | Traditional events |
| Evidence GPS | Proceed | Reduce complexity | Basic linking only |
| Vendor Feasibility | Proceed | Provide tools | Open source only |
| User Acceptance | Proceed | Iterate | Redesign |

## Risk Mitigation Summary

### Technical Risks
1. **Complexity**: Graduated implementation
2. **Performance**: Optimization guidelines
3. **Compatibility**: Dual export options

### Adoption Risks
1. **Vendor resistance**: Reference implementations
2. **User confusion**: Clear documentation
3. **Migration issues**: Automated tools

### Strategic Risks
1. **Scope creep**: Strict MVP definition
2. **Standards conflict**: Early GEDCOM engagement
3. **Community split**: Inclusive development

## Conclusion

These tests will validate our core assumptions before significant development investment. The results will guide final design decisions and implementation priorities. Success criteria are deliberately stringent to ensure we build extensions that genuinely solve problems and gain adoption.