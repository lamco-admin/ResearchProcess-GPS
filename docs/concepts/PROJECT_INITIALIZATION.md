# ResearchProcess-GPS Project Initialization

## Project Genesis

After 15 years of attempting to integrate research methodology into traditional genealogy data models, we've reached a critical realization: **research process and genealogy databases are fundamentally different domains requiring separate solutions**.

## Why This Project Exists

### The Failed Integration Attempts
- BetterGEDCOM (2010-2013): Tried to add research to GEDCOM
- GEDCOM X: Created parallel evidence model, limited adoption
- Various software: Added research logs as afterthoughts
- Our extensions: Even "floating evidence" is still person-centric ultimately

### The Core Incompatibility
**Traditional Genealogy Software**: Person → Family → Events → Sources
**Research Process**: Question → Hypothesis → Evidence → Analysis → Conclusion → Person (maybe)

These are inverse workflows that cannot be reconciled without fundamental disruption to an established ecosystem.

## Project Vision

ResearchProcess-GPS is a **research methodology platform** that:
- Treats research as the primary workflow, not data entry
- Supports GPS (Genealogical Proof Standard) natively
- Complements rather than replaces genealogy software
- Exchanges data with GEDCOM but uses its own model

## Key Differentiators

### What This IS:
- Research question management system
- Evidence accumulation platform
- Hypothesis tracking tool
- GPS compliance assistant
- Analysis documentation system
- Collaboration framework

### What This IS NOT:
- Another genealogy database
- GEDCOM replacement
- Family tree software
- Person-centric system

## Foundational Concepts (From Data Models Work)

### 1. Evidence-First Architecture
- Evidence exists independently
- Identity emerges from analysis
- Conclusions are provisional
- Conflicting evidence is normal

### 2. Research Process Tracking
- Research plans and objectives
- Search strategies and scope
- Negative evidence documentation
- Exhaustive search verification

### 3. GPS Methodology Support
- Reasonably exhaustive search
- Complete source citations
- Analysis and correlation
- Resolution of conflicts
- Soundly reasoned conclusions

### 4. Time-Aware Research
- Research happens over decades
- Hypotheses evolve
- New evidence changes everything
- Progress tracking essential

## Technical Approach

### Data Model Principles
- **Not GEDCOM-based** (but can export/import)
- **Graph-based** (questions → evidence → analysis)
- **Version-controlled** (track hypothesis evolution)
- **Collaboration-native** (multiple researchers)

### Integration Strategy
- Import from GEDCOM 7 (especially with our extensions)
- Export conclusions to GEDCOM
- API for genealogy software integration
- Initially target GRAMPS (open source, accessible)

## Initial Development Phases

### Phase 1: Core Concepts
- Define research process data model
- Separate from person-centric thinking
- Document GPS workflow patterns
- Create proof-of-concept schemas

### Phase 2: Prototype
- Simple research question tracking
- Evidence accumulation interface
- Basic hypothesis management
- GPS checklist integration

### Phase 3: Integration
- GEDCOM import (with extensions)
- GRAMPS plugin/integration
- Export proven conclusions
- Collaboration features

## Success Metrics

### Short Term
- Model research process accurately
- Support basic GPS workflow
- Import/export with GEDCOM 7
- Demonstrate value proposition

### Long Term
- Professional genealogist adoption
- Methodology standardization
- Industry recognition
- Sustainable development

## Relationship to Other Projects

### Genealogy Data Models
- Grew from this work
- Identified the incompatibility
- Provided evidence extension foundation
- Remains separate concern

### GEDCOM 7 Extensions
- Complementary not competitive
- Extensions help data exchange
- Research process uses different model
- Can leverage citation/evidence work

### genealogy-ai Umbrella
- One project among many
- Focused on methodology
- May use AI for analysis later
- Distinct from data model work

## Next Steps

1. Create dedicated repository
2. Document research process patterns
3. Design initial data model
4. Build proof-of-concept
5. Engage professional genealogists

## Conclusion

ResearchProcess-GPS acknowledges what 15 years of attempts have shown: research methodology cannot be forced into person-centric data models. By creating a purpose-built platform for genealogical research processes, we can finally support how professionals actually work, while maintaining compatibility with existing tools.

**This is not another genealogy database. This is a research methodology platform.**

---
*Born from the Genealogy Data Models work, but destined to chart its own course.*