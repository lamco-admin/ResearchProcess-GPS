# Unified Vision Summary - ResearchProcess-GPS
## 2025-07-30 20:45 EEST

## The Revolutionary Insight

After deep research into all documents, the revolutionary insight is clear: **ResearchProcess-GPS treats genealogical methodologies, standards, and work products as configurable metadata** rather than hard-coded constraints. This transforms genealogy software from rigid conclusion-tracking to flexible research-process modeling.

## How Everything Works Together

### 1. Standards as Configuration Files

Instead of hard-coding GPS or BCG standards:

```yaml
# config/standards/GPS-2025.yaml
standard:
  name: "Genealogical Proof Standard"
  elements:
    reasonably_exhaustive_research:
      requirements:
        repository_coverage: 0.90
        negative_search_documentation: required
    complete_citations:
      template_system: "Evidence_Explained"
```

**Impact**: Users can load ANY standard - GPS, BCG, NGS, custom institutional standards - without changing code.

### 2. Methodologies as Pluggable Modules

Research methodologies become configuration:

```yaml
# config/methodologies/FAN_Principle.yaml
methodology:
  name: "FAN Principle"
  description: "Friends, Associates, Neighbors"
  workflow:
    - identify_subject
    - enumerate_associates
    - research_network
    - analyze_patterns
```

**Impact**: New methodologies can be added as YAML files. The system adapts its workflow accordingly.

### 3. Work Products as Templates

Professional work products are template-driven:

```yaml
# config/work_products/proof_statement.yaml
work_product:
  name: "Proof Statement"
  sections:
    - research_question
    - evidence_presentation
    - analysis
    - conflict_resolution
    - conclusion
  gps_compliance_check: automatic
```

**Impact**: Generate BCG-compliant proof statements, research logs, or custom reports from the same data.

## The Complete Toolbox

### Planning Tools
- **Research Plan Generator**: Creates plans from Theory (Research Question) using configurable templates
- **Objective Tracker**: Monitors progress against GPS/BCG requirements
- **Repository Checklist**: Configurable by location/time period

### Collection Tools
- **Auto-Capture Browser Extension**: Logs searches automatically
- **Negative Result Tracker**: Documents what WASN'T found (GPS requirement)
- **Session Recorder**: Complete research trail with timestamps

### Analysis Tools
- **Evidence Correlator**: Builds matrices per Evidence Explained standards
- **Conflict Detector**: Identifies contradictions automatically
- **Quality Assessor**: Multi-dimensional evaluation (source/information/evidence)

### Documentation Tools
- **Proof Statement Builder**: GPS-compliant narrative generation
- **Research Log Formatter**: BCG-standard or custom formats
- **Report Generator**: Multiple output formats from same data

### Collaboration Tools
- **Theory Branching**: Git-like alternative hypothesis tracking
- **Peer Review Workflow**: Built-in review cycles
- **Attribution Tracker**: Every change credited

## Real-World Workflow Example

### 1. Start Research
```python
# User asks: "Who were the parents of John Smith b.1850?"
theory = create_theory(
    question="Who were the parents of John Smith b.1850?",
    standards=["GPS-2025", "BCG-3.4"]  # Loaded from YAML
)
```

### 2. System Automatically:
- Creates ResearchLog with BCG schema
- Loads GPS compliance checklist
- Initializes Confidence container
- Sets up work product templates

### 3. Research Process
```python
# Auto-captured from browser extension
log_entry = capture_search(
    repository="FamilySearch",
    collection="Massachusetts Births",
    parameters={"name": "John Smith", "year": "1850"},
    results=[]  # Negative result
)

# System tracks negative evidence (GPS requirement)
confidence.coverage.add_negative_search(log_entry)
```

### 4. Evidence Discovery
```python
# Found census record
evidence = create_evidence(
    source=census_source,
    extracted_data={"name": "John Smith", "age": "25"},
    quality_assessment=auto_assess_quality()  # Uses loaded standards
)

# Creates IdentityPersona automatically
identity = create_identity_from_evidence(evidence)
```

### 5. Analysis Phase
```python
# System generates correlation matrix
analysis = analyze_evidence(
    evidence_list=[evidence1, evidence2, evidence3],
    methodology="Evidence_Analysis_Worksheet"  # Loaded template
)

# Detects conflicts automatically
conflicts = detect_conflicts(analysis)
```

### 6. Confidence Building
```python
# Not just a score but a complete narrative
assessment = create_assessment(
    methodology="GPS-2025",
    narrative="""
    The 1850 census provides primary information about age...
    Marriage record corroborates... 
    Absence of birth record explained by courthouse fire...
    """,
    dimensions={
        "source_reliability": 0.85,
        "research_exhaustiveness": 0.75
    }
)

confidence.add_assessment(assessment)
```

### 7. Documentation
```python
# Generate GPS-compliant proof statement
proof = generate_proof_statement(
    theory=theory,
    template="BCG_Standard",
    format="markdown"
)

# System checks GPS compliance automatically
compliance_report = check_gps_compliance(proof)
```

## The Revolutionary Aspects in Practice

### 1. Process-First Design
- Every search logged (including negative results)
- Research journey documented, not just conclusions
- Thinking captured in Confidence narratives

### 2. Standards Without Hard-Coding
- Load GPS 2025 today, GPS 2030 tomorrow
- Support multiple standards simultaneously
- Custom institutional standards easy to add

### 3. True Flexibility
- IdentityPersona → Person promotion when ready
- Unlimited nesting for complex research
- Cultural adaptability built-in

### 4. Professional-Grade Confidence
- Complete research narratives
- Peer review workflows
- GPS compliance tracking
- Audit trails for certification

### 5. Collaborative Research
- Branch theories like Git branches
- Merge conflicting research
- Full attribution tracking
- Knowledge builds cumulatively

## Why This Matters

Traditional genealogy software asks: **"What do you know?"**

ResearchProcess-GPS asks: **"How do you know it?"**

This fundamental shift enables:
- Professional genealogists to maintain certification standards
- Researchers to document their process
- Institutions to enforce methodologies
- Teams to collaborate effectively
- Knowledge to build systematically

## Implementation Path

### Phase 1: Foundation (Now)
- Enhanced Confidence container ✓
- Theory (Research Question) entity ✓
- Standards configuration system ✓
- Basic state machines ✓

### Phase 2: Core Entities (Next)
- IdentityPersona with promotion
- ResearchLog with auto-capture
- Evidence with quality assessment
- Citation with states

### Phase 3: Process Tools
- Browser extension for auto-capture
- Analysis worksheet generators
- GPS compliance checkers
- Report builders

### Phase 4: Collaboration
- Git storage adapter
- Theory branching/merging
- Peer review workflows
- Publishing pipeline

## The Bottom Line

ResearchProcess-GPS doesn't tell genealogists how to research. Instead, it:
- **Adapts** to their standards and methodologies
- **Documents** their research process automatically
- **Validates** against their chosen standards
- **Generates** their required work products
- **Preserves** their research journey

This is genealogy software designed for how genealogists actually work - where the journey is as important as the destination, and confidence comes from comprehensive documentation, not simple scores.