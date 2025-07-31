# Research-GPS Extension Findings Summary

## Executive Summary

Mining the BetterGEDCOM database revealed consistent user demand for research process support that current genealogy software fails to provide. Users need tools to document their research journey, not just conclusions. This aligns perfectly with our Research-GPS extension objectives.

## Key Findings

### 1. Universal Pain Points
- **"Software doesn't support genealogical research"** - Common refrain
- Cannot track what was searched and when
- No way to document reasoning process
- Evidence and conclusions conflated
- Research context lost in data exchange

### 2. Most Requested Features
1. **Research Log** (20+ mentions) - Track all searches
2. **Research Plan** - Document future work
3. **Proof Statements** - Explain reasoning
4. **Evidence Analysis** - Evaluate sources
5. **Hypothesis Tracking** - Test theories
6. **Task Management** - Track todo items
7. **GPS Compliance** - Meet professional standards

### 3. Professional Work Products Needed
- Research logs (BCG format)
- Proof arguments (narrative)
- Evidence analysis worksheets
- Research reports
- Timeline/correlation analysis
- Hypothesis documentation

## Implications for Research-GPS Extension

### Core Components Required

#### 1. Research Activity Tracking
```yaml
ResearchActivity:
  - activity_id
  - date_performed
  - repository/database
  - search_parameters
  - results_summary
  - negative_result_flag
  - linked_sources[]
  - next_steps
```

#### 2. Evidence Analysis Framework
```yaml
EvidenceAnalysis:
  - source_reference
  - information_extracted[]
  - evidence_type (direct/indirect/negative)
  - quality_assessment
  - conflicts_noted
  - gps_evaluation
```

#### 3. Proof Documentation
```yaml
ProofStatement:
  - research_question
  - evidence_presented[]
  - reasoning_text
  - conclusion
  - gps_compliance
  - review_status
```

#### 4. Hypothesis Management
```yaml
Hypothesis:
  - description
  - status (active/tested/proven/disproven)
  - supporting_evidence[]
  - contradicting_evidence[]
  - test_results
```

### Integration Strategy

#### Phase 1: Basic Research Tracking
- Simple research log entries
- Link to existing GEDCOM 7 structures
- Use NOTE records with typing
- Minimal disruption to current workflows

#### Phase 2: Evidence Framework
- Evidence quality assessment
- Source analysis tools
- Conflict documentation
- GPS compliance indicators

#### Phase 3: Full GPS Support
- Complete research workflow
- Hypothesis tracking
- Proof argument generation
- Professional report creation

## Alignment with User Needs

### What Users Want → What We'll Provide

1. **"Track my searches"** → Research activity log with negative results
2. **"Document my reasoning"** → Structured proof statements
3. **"Test different theories"** → Hypothesis tracking with evidence links
4. **"Meet GPS standards"** → Built-in GPS compliance checking
5. **"Share research process"** → Exportable research documentation
6. **"Analyze evidence quality"** → Evidence assessment framework

## Technical Approach

### GEDCOM 7 Extension Structure
```
0 @R1@ _RESEARCH
1 _ACTIVITY @A1@
2 DATE 29 JAN 2025
2 _REPO "FamilySearch.org" 
2 _SEARCH "John Smith born 1850 Ohio"
2 _RESULT "No matches found"
2 NOTE Checked all Ohio counties

0 @H1@ _HYPOTHESIS
1 _DESC "John Smith son of William Smith"
1 _STATUS TESTING
1 _EVIDENCE_FOR @S1@ @S2@
1 _EVIDENCE_AGAINST @S3@
```

### Data Model Extensions
- New record types for research entities
- Enhanced NOTE structure with subtypes
- Links between evidence and analysis
- Temporal tracking of research progress

## Next Steps

### Immediate Actions
1. Draft Research-GPS extension specification
2. Create example GEDCOM 7 files showing usage
3. Build proof-of-concept implementation
4. Test with real research scenarios

### Community Engagement
1. Share findings with GEDCOM community
2. Solicit feedback from professional genealogists
3. Coordinate with other extension developers
4. Align with Evidence extension work

### Development Priorities
1. **Must Have**: Research log, basic proof statements
2. **Should Have**: Evidence analysis, hypothesis tracking  
3. **Nice to Have**: Full GPS compliance checking, report generation

## Success Criteria

### User Perspective
- Can document complete research process
- Can share research with full context
- Can meet professional standards
- Improves research efficiency

### Technical Perspective
- Clean integration with GEDCOM 7
- Minimal complexity added
- Backward compatible where possible
- Extensible for future needs

## Conclusion

The BetterGEDCOM analysis confirms strong demand for research process support. Users need more than conclusion recording - they need tools to document how they reached those conclusions. Our Research-GPS extension can fill this critical gap by providing structured ways to capture research activities, analyze evidence, document reasoning, and track hypotheses. This positions Research-GPS as an essential complement to the Evidence extension, together enabling truly professional genealogical research within GEDCOM 7.