# BetterGEDCOM Integration Analysis: Learning from the Past to Build the Future

## Executive Summary

The BetterGEDCOM project (2010-2013) represents the genealogy community's most comprehensive attempt to identify and solve data interchange problems. Through analysis of 318 wiki pages, 594 discussions (4,970 replies), 47 documents, and contributions from 74 participants, combined with our current understanding of GEDCOM 7 extensions and modern genealogy software needs, we can chart a clear path forward for practical, adoptable extensions.

## Key Insights from BetterGEDCOM

### 1. Citations Were THE Critical Issue
- **218 discussions, 2,243 replies** - more than any other topic
- Users desperately wanted:
  - Element-level citation data (not just text blobs)
  - Support for layered citations (source within source)
  - Separation of data storage from presentation
  - Multiple citation style support (Chicago, Evidence Explained, etc.)

### 2. Evidence vs. Conclusion Separation
- **61 dedicated discussions** on this topic
- Tom Wetmore's DeadEnds model proposed tree structures where:
  - Evidence records are immutable leaves
  - Conclusion records group evidence records
  - Each grouping decision is documented
  - Process is fully reversible
- This maps directly to professional genealogy methodology

### 3. Research Process Support
- Adrian Bruce's detailed research process model
- Need for research plans, objectives, tasks
- Proof summaries and arguments
- Handling of conflicting evidence
- Documentation of negative evidence

### 4. Community Leaders' Visions
- **Tom Wetmore (ttwetmore)**: Evidence/conclusion trees, process modeling
- **GeneJ**: Citation expert, Evidence Explained integration
- **AdrianB38**: Research process, "Missing Link" concept
- **Geir Thorud (gthorud)**: Detailed citation data models
- **Louis Kessler**: GEDCOM compatibility focus

## Current State Analysis (2025)

### What Exists Now

#### 1. GEDCOM 7 (2021)
- Basic source/repository improvements
- Limited citation model
- No evidence/conclusion separation
- No research process support

#### 2. GEDCOM X (2012-2013)
- Person/PersonView for evidence modeling
- Attribution framework
- Limited adoption outside FamilySearch

#### 3. Current Extensions
- **gedcom-citations** (dthaler): Well-designed citation framework
- **gedcom-occurrences**: Draft for event independence
- Various proprietary extensions in software

### What's Still Missing

1. **Flexible Citation System**
   - Templates for common source types
   - Style-specific formatting
   - Element-level data capture

2. **Evidence/Conclusion Framework**
   - Explicit modeling of reasoning
   - Support for conflicting evidence
   - Documentation of proof arguments

3. **Research Process Support**
   - Research objectives and plans
   - Task management
   - GPS compliance tracking

4. **Professional Features**
   - Negative evidence documentation
   - Hypothesis tracking
   - Collaboration support

## Integrated Extension Design

Based on BetterGEDCOM insights and current needs, here's an integrated approach:

### Priority 1: Enhanced Citation System

**Build on existing gedcom-citations, adding:**

```yaml
# Citation Template Extension
%YAML 1.2
---
type: structure
uri: https://github.com/genealogy-standards/gedcom-citations-enhanced/template-v1
standard tag: _CITTPL
label: 'Citation Template'

specification:
  - Template for formatting citations in specific styles
  
substructures:
  - _STYLE: "{1:1}" # CHICAGO|EVIDENCE|MLA|APA|CUSTOM
  - _FORMAT: "{1:M}" # Format strings with placeholders
  - _ELEMENTS: "{1:M}" # Required elements for this template
```

**Benefits:**
- Addresses BetterGEDCOM's #1 concern
- Enables faithful data transfer
- Supports multiple citation styles
- Professional genealogy compliance

### Priority 2: Evidence & Conclusion Framework

**Inspired by Tom Wetmore's DeadEnds model:**

```yaml
# Evidence Container Extension
%YAML 1.2
---
type: structure
uri: https://github.com/genealogy-standards/gedcom-evidence/evidence-v1
standard tag: _EVID
label: 'Evidence Container'

specification:
  - Groups source-based assertions as evidence
  
payload: "@<XREF>@" # Reference to source

substructures:
  - _ASSERT: "{1:M}" # Assertions from this evidence
  - _EXTRACT: "{0:1}" # Extracted/transcribed text
  - _ANALYSIS: "{0:1}" # Researcher's analysis
  - _CONF: "{0:1}" # Confidence assessment
```

```yaml
# Conclusion Structure
%YAML 1.2
---
type: structure
uri: https://github.com/genealogy-standards/gedcom-evidence/conclusion-v1
standard tag: _CONC
label: 'Conclusion'

specification:
  - Documents reasoning from evidence to conclusion
  
substructures:
  - _EVID: "{1:M} @<XREF:_EVID>@" # Supporting evidence
  - _REASONING: "{1:1}" # Proof argument
  - _CONFLICTS: "{0:M}" # Conflicting evidence
  - _CONF: "{0:1}" # Overall confidence
```

### Priority 3: Research Process Extension

**Based on Adrian Bruce's research process model:**

```yaml
# Research Plan Extension
%YAML 1.2
---
type: record
uri: https://github.com/genealogy-standards/gedcom-research/plan-v1
standard tag: _RPLAN
label: 'Research Plan'

specification:
  - Documents research objectives and progress
  
substructures:
  - _GOAL: "{1:1}" # Focused research goal
  - _PLAN: "{0:M}" # Work portions
  - _LOG: "{0:M}" # Research log entries
  - _STATUS: "{0:1}" # Current status
  - _GPS: "{0:1}" # GPS compliance tracking
```

### Priority 4: Event Independence (from occurrences)

**Refined from gedcom-occurrences draft:**

```yaml
# Independent Event Record
%YAML 1.2
---
type: record
uri: https://github.com/genealogy-standards/gedcom-events/event-v1
standard tag: _EVENT
label: 'Independent Event'

specification:
  - Event as first-class object with participants
  
substructures:
  - TYPE: "{1:1}"
  - DATE: "{0:1}"
  - PLAC: "{0:1}"
  - _PART: "{0:M}" # Participants with roles
  - _EVID: "{0:M}" # Evidence for this event
```

## Implementation Strategy

### Phase 1: Foundation (Months 1-2)
1. **Adopt gedcom-citations** as base
2. **Add citation templates** for style support
3. **Create evidence container** structure
4. **Document BetterGEDCOM learnings**

### Phase 2: Core Extensions (Months 3-4)
1. **Evidence/Conclusion framework**
2. **Research process support**
3. **Enhanced relationships**
4. **Confidence extensions**

### Phase 3: Advanced Features (Months 5-6)
1. **Event independence**
2. **Negative evidence**
3. **Hypothesis tracking**
4. **Collaboration support**

### Phase 4: Integration (Months 7-8)
1. **GRAMPS patches**
2. **Reference implementations**
3. **Migration tools**
4. **Validation suite**

## Why This Will Succeed Where Others Failed

### 1. Incremental Approach
- Start with existing work (gedcom-citations)
- Add features gradually
- Maintain backward compatibility
- Allow partial adoption

### 2. Community Lessons Learned
- Address real pain points (citations!)
- Support existing workflows
- Don't force methodology changes
- Enable professional standards

### 3. Technical Pragmatism
- Simple, understandable extensions
- Clear migration paths
- Reference implementations
- Extensive examples

### 4. Standards Alignment
- GPS compliance built-in
- Evidence Explained support
- Professional genealogy methods
- Academic research standards

## Measuring Success

### Adoption Metrics
- 3+ major applications implementing within 18 months
- 50% reduction in citation data loss complaints
- Active development community
- FamilySearch engagement

### Technical Metrics
- Zero data loss for round-trip transfers
- Performance impact <5%
- Validation tool accuracy >95%
- Test coverage >80%

### User Satisfaction
- Professional genealogists endorsement
- Reduced support tickets
- Positive community feedback
- Conference presentations

## Conclusion

BetterGEDCOM showed us what the genealogy community desperately needs. Current technology and the GEDCOM 7 extension framework give us the tools to deliver it. By learning from past attempts, building on existing work, and focusing on incremental, practical improvements, we can finally solve the data interchange problems that have plagued genealogy software for decades.

The key is not to create the perfect standard, but to create adoptable extensions that solve real problems. Start with citations (the community's biggest pain point), add evidence/conclusion support (for professionals), and gradually build out the full ecosystem.

BetterGEDCOM's 74 contributors spent 3 years documenting what genealogists need. Let's honor their work by building practical solutions that the community can actually use.