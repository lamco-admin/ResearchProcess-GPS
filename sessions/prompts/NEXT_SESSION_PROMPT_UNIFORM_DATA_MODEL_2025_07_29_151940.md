# Next Session Prompt: Uniform Data Model Architecture

## Context

ResearchProcess-GPS is a revolutionary genealogical research platform with comprehensive specifications completed for all major systems. We now need to design the uniform data model that makes this vision technically feasible while ensuring perfect semantic fidelity across all systems and use cases.

## Previous Work Completed

We have created detailed specifications for:
- Identity/Persona system for uncertain people
- Flexible link system beyond GEDCOM relationships  
- Research logging with auto-capture and GPS compliance
- Multi-type DNA models with privacy controls
- Theory versioning and collaborative workflows
- Sync modes from offline to real-time collaborative
- Research certification and negative findings
- Cross-platform linkage to any genealogy system
- Professional composition and publishing tools

## Primary Objective

Design a uniform data model that supports ALL the functionality we've specified while ensuring:
1. **Zero data loss** when switching between applications/services
2. **Perfect semantic preservation** - meaning is never lost or degraded
3. **Universal interoperability** - any system can faithfully exchange data
4. **GEDCOM 7 compliance** - can be perfectly represented in standard format
5. **Future extensibility** - can grow without breaking compatibility

## Key Requirements

### Semantic Fidelity
The data model must capture not just facts but:
- WHY conclusions were reached (reasoning chains)
- HOW confident we are (quantified uncertainty) 
- WHAT alternatives were considered (hypothesis branching)
- WHO made decisions and WHEN (full provenance)
- WHAT evidence supports each claim (evidence linking)

### System Interoperability
Must support exchange between:
- ResearchProcess-GPS instances
- Traditional genealogy software (FTM, RootsMagic, etc.)
- Online platforms (FamilySearch, Ancestry, WikiTree)
- Research databases and archives
- Academic and professional systems

### Technical Flexibility
The model must work with:
- File-based storage (.rgps archives)
- Streaming/event-sourced architectures
- Traditional relational databases
- Graph databases for relationships
- Document stores for flexible schemas

## Specific Areas to Address

### 1. Core Entity Model
Design the fundamental entities that capture:
- Persons (resolved individuals)
- Identities (uncertain people fragments)
- Relationships (flexible, beyond family)
- Evidence (with quality assessments)
- Sources (with full provenance)
- Research activities (the process itself)

### 2. Semantic Metadata Schema
Define how to capture:
- Confidence levels and scoring
- Evidence quality assessments
- Research methodology used
- Decision reasoning
- Alternative theories considered
- Peer review and collaboration data

### 3. GEDCOM 7 Extension Strategy  
Determine what extensions are needed to carry:
- Research process metadata
- Uncertainty and confidence data
- Flexible relationship types
- Evidence analysis results
- Negative findings
- Cross-platform links

### 4. Data Loss Prevention
Analyze potential loss points:
- What happens to research metadata in "dumb" systems?
- How to preserve semantics in systems without native support?
- What's the degradation strategy for various target systems?
- How to detect and warn about potential data loss?

### 5. Universal Schema Design
Create a schema that:
- Is rich enough for all ResearchProcess-GPS features
- Can be mapped to any existing genealogy system
- Preserves maximum semantics in translations
- Supports round-trip fidelity where possible
- Gracefully degrades when necessary

## Research Questions to Explore

1. **What are the irreducible semantic primitives** that must be preserved?
2. **How do we handle semantic concepts** that don't exist in target systems?
3. **What's the minimum viable extension set** for GEDCOM 7 compliance?
4. **How do we maintain referential integrity** across distributed systems?
5. **What's the optimal granularity** for version control and sync?

## Expected Deliverables

### Technical Specifications
- Universal data model schema (JSON Schema/GraphQL/SQL)
- GEDCOM 7 extension specification
- System mapping strategies
- Data loss analysis and mitigation
- Reference implementation design

### Implementation Guidance  
- Database schema designs
- API specifications
- File format definitions
- Migration strategies
- Quality assurance procedures

## Current Project State

**Location**: `/home/greg/genealogy-ai/ResearchProcess-GPS/`

**Essential Session Context Documents**:
- **COMPREHENSIVE_SESSION_HANDOVER_2025_07_29_151940.md** - Complete summary of all work done this session
- **NEXT_SESSION_PROMPT_UNIFORM_DATA_MODEL_2025_07_29_151940.md** - This document with detailed directions

**Key Files**:
- All platform specifications are complete (12 major documents)
- Analysis of user needs and current system gaps
- Evidence extension work and relationship modeling
- Database integration analysis

**Next Phase**: Technical foundation design - the uniform data model that makes the vision possible.

## Session Direction

Focus on the technical architecture that enables universal genealogical data exchange without semantic loss. We need to solve the fundamental interoperability problem that has plagued genealogy software for decades - how to move complete research contexts between systems without losing meaning.

This is the foundation that will enable researchers to:
- Use any combination of tools without lock-in
- Switch systems with zero data loss  
- Collaborate across platform boundaries
- Preserve research integrity permanently
- Build on others' work with full context

Start by analyzing what semantic information is lost in current system transfers, then design the universal model that prevents this loss while maintaining practical compatibility with existing systems.