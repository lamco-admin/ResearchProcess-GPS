# GEDCOM 7 Extension Architecture Recommendations

## Executive Summary

After reviewing the three existing extensions (citations, occurrences, evidence) and analyzing the architectural options, I recommend a **Modular Architecture with Coordinated Design** approach. This balances flexibility, adoption potential, and technical coherence.

## Analysis of Current Extensions

### 1. gedcom-citations (dthaler)
**Strengths:**
- Well-developed, proven model with YAML definitions
- Addresses real citation portability problems
- Template system is powerful and flexible
- Already has some adoption/recognition

**Weaknesses:**
- Template syntax (bash-style) may be complex for vendors
- No element-level citation support for professional standards
- Limited integration with evidence tracking

### 2. gedcom-occurrences (glamberson)
**Strengths:**
- Solves critical event duplication problem
- Clean design with clear separation of concerns
- Maps well to GRAMPS and GEDCOM X
- Backward compatibility strategy included

**Issues:**
- Name `_OCUR` is unclear (should be `_EVENT`)
- Role vocabulary needs standardization
- Missing event-to-event relationships
- Participant structure could be simplified

### 3. gedcom-evidence-draft
**Strengths:**
- Addresses professional genealogy needs
- Supports GPS methodology
- Clear separation of evidence from conclusions
- Research process documentation

**Issues:**
- Complex structure may intimidate vendors
- Overlap with citation functionality unclear
- Need better integration with other extensions

## Architectural Recommendation: Modular with Core Dependencies

### Recommended Architecture

```
Core Extensions (Interdependent):
├── gedcom-citations-enhanced/     # Build on dthaler's work
├── gedcom-events/                 # Renamed from occurrences
└── gedcom-evidence/               # Simplified from draft

Optional Extensions:
├── gedcom-relationships/          # Enhanced relationships
├── gedcom-research/              # Research process tracking
└── gedcom-confidence/            # Confidence/quality framework
```

### Why Modular?

1. **Incremental Adoption**: Vendors can implement one extension at a time
2. **Clear Boundaries**: Each extension has a specific purpose
3. **Easier Testing**: Can validate extensions independently
4. **Community Development**: Different teams can work on different extensions

### Why Not Unified?

1. **Too Complex**: All-or-nothing adoption would kill vendor interest
2. **Maintenance Burden**: Single large spec harder to update
3. **Version Conflicts**: Updates to one area affect entire spec

## Critical Decisions and Recommendations

### 1. Citation Enhancement Strategy

**Recommendation**: Enhance dthaler's work rather than fork

**Approach**:
```yaml
# Add to dthaler's model:
- Element-level citations (_ELEM)
- Professional style support (_STYL)
- Citation quality indicators (_CQUAL)
```

**Rationale**:
- Builds on existing recognition
- Maintains compatibility
- Adds professional features incrementally

### 2. Evidence Framework Integration

**Recommendation**: Evidence references sources AND citations

**Design**:
```gedcom
0 @E1@ _EVID
1 SOUR @S1@              # Source reference
2 PAGE Sheet 5
2 _CITA @C1@            # Optional citation reference
1 _INFO John Smith, age 40
```

**Rationale**:
- Sources are required (evidence must come from somewhere)
- Citations are optional (may use templates or custom)
- Maintains flexibility for different workflows

### 3. Event Model Refinement

**Recommendation**: Rename to `gedcom-events` with `_EVENT` tag

**Changes**:
```gedcom
# Old:
0 @O1@ _OCUR

# New:
0 @E1@ _EVENT
1 TYPE Census
1 _PART @I1@
2 ROLE Head
```

**Additional Features**:
- Event relationships (`_EREL`)
- Standardized role vocabulary
- Event hierarchies

### 4. Minimum Viable Product (MVP)

**Phase 1 MVP (6 months)**:
1. Enhanced Citations (building on dthaler)
   - Add element-level support
   - Professional templates
   - Basic validation tools

2. Events (renamed occurrences)
   - Core independent events
   - Basic roles
   - GRAMPS compatibility

**Phase 2 (12 months)**:
3. Evidence Framework
   - Basic evidence containers
   - Simple evidence linking
   - Research notes

**Why this order**:
- Citations are most requested feature
- Events solve immediate duplication problem
- Evidence builds on both

## Technical Validation Plan

### Test Assumptions

1. **Citation Templates**
   ```bash
   # Test 1: Export from RootsMagic
   # Test 2: Import to FTM
   # Test 3: Professional review
   ```

2. **Event Independence**
   ```gedcom
   # Test: 1850 Census with 50 people
   # Measure: File size impact
   # Benchmark: Performance
   ```

3. **Evidence Containers**
   ```
   # Test: Complex proof argument
   # Validate: GPS compliance
   # Review: Usability
   ```

## Vendor Adoption Strategy

### Target Order
1. **GRAMPS** - Most aligned, open source
2. **RootsMagic** - Already uses templates
3. **Family Historian** - Has extension experience
4. **FamilySearch** - Influence on standard

### Adoption Barriers

**Technical**:
- Complexity of implementation
- Testing requirements
- Migration tools needed

**Business**:
- Development costs
- User education
- Competitive advantage loss

### Mitigation
- Provide reference implementations
- Create migration tools
- Emphasize user demand
- Show competitive advantage

## Next Steps (Immediate)

### Week 1-2: Architecture Finalization
1. Create repository structure
2. Define governance model
3. Set up CI/CD pipeline
4. Establish testing framework

### Week 3-4: Citation Enhancement
1. Fork/enhance dthaler's repository
2. Add element-level support
3. Create professional templates
4. Write migration guide

### Week 5-6: Event Model
1. Rename and restructure
2. Standardize vocabulary
3. Create examples
4. Test with GRAMPS

### Week 7-8: Evidence Framework
1. Simplify current design
2. Integrate with citations
3. Create basic examples
4. Professional review

## Risk Mitigation

### Over-Engineering
- Start simple, enhance later
- User feedback loops
- Incremental releases

### Under-Adoption
- Strong value proposition
- Migration tools
- Community engagement
- Vendor partnerships

### Technical Debt
- Clean architecture
- Comprehensive tests
- Documentation first
- Regular refactoring

## Success Metrics

### 6 Months
- 2+ implementations
- 100+ beta users
- Positive feedback
- No major issues

### 12 Months
- 5+ implementations
- 1000+ users
- Standard proposal
- Community adoption

### 24 Months
- Major vendor adoption
- Standard inclusion
- Wide usage
- Ecosystem growth

## Conclusion

The modular approach with coordinated design offers the best path forward. It allows incremental adoption while maintaining technical coherence. Starting with enhanced citations and independent events provides immediate value while building toward comprehensive evidence-based genealogy support.

The key is to start simple, test thoroughly, and iterate based on real-world usage. Success depends on balancing professional needs with practical implementation concerns.