# Research Process Feature Compilation from BetterGEDCOM Analysis

## Executive Summary

Based on mining 78 wiki pages, 300+ discussions, and 540+ discussion replies from the BetterGEDCOM database, users consistently expressed needs for research process support that current genealogy software lacks. The most frequently requested features center around documenting the research process itself, not just conclusions.

## Core Research Work Products Requested

### 1. Research Log
**Frequency**: Most mentioned feature (20+ distinct mentions)
**User Need**: Track what sources have been searched, when, and what was found/not found
**Key Quote**: "research log capability built into software... can be used to record snippets or full transcriptions from sources, together with your own comments and notes" - GeneJ

### 2. Research Plan  
**User Need**: Document future research objectives and strategies
**Integration**: Should link to specific research questions and hypotheses

### 3. Proof Statements/Arguments
**User Need**: Formal documentation of reasoning that leads to conclusions
**Key Quote**: "Free-format notes, with accompanying optional citations... The Parish Register of St. Mary's has the baptism on..." - AdrianB38

### 4. Proof Summary
**User Need**: Concise summation of evidence and reasoning for a conclusion
**Distinction**: One work-portion has exactly one objective and gives rise to one proof

### 5. Evidence Analysis
**User Need**: Structured evaluation of source quality, information, and evidence
**GPS Alignment**: Critical for meeting Genealogical Proof Standard requirements

### 6. Task/Todo Lists
**User Need**: Track research tasks, both completed and pending
**Integration**: Should link to research plans and specific individuals/questions

### 7. Hypothesis Tracking
**User Need**: Document and test alternative theories about relationships
**Key Quote**: "When I decide an hypothesis is incorrect, I can rearrange the personas into a different set of persons" - ttwetmore

## Key Software Limitations Identified

### Current Software Cannot:
1. Distinguish between evidence and conclusions properly
2. Track the research process (what was searched, when, results)
3. Document reasoning and analysis
4. Support iterative hypothesis testing
5. Export/import research process data between programs

### User Frustrations:
- "software doesn't support genealogical research by not distinguishing between evidence and conclusions" - ttwetmore
- Loss of research context when sharing data
- No standard way to document GPS compliance
- Cannot track negative evidence (searches that found nothing)

## GPS (Genealogical Proof Standard) Requirements

Users repeatedly mentioned need for GPS compliance features:
1. **Reasonably exhaustive search** - Track all searches performed
2. **Complete and accurate citations** - Already partially supported
3. **Analysis and correlation** - Need evidence analysis tools
4. **Resolution of conflicts** - Document conflicting evidence
5. **Soundly reasoned conclusion** - Proof statements/arguments

## Data Model Implications

### 1. Research Process Entities Needed
- Research Log entries (searches performed)
- Research Plans (future work)
- Research Questions/Objectives
- Hypotheses (with status: active, disproven, proven)
- Analysis documents (evidence evaluation)
- Proof arguments (reasoning documentation)

### 2. Relationships Required
- Evidence → Analysis → Conclusion chains
- Source → Search Log → Found/Not Found status
- Person/Persona → Hypothesis → Conclusion Person
- Research Question → Research Plan → Research Log → Result

### 3. Status Tracking
- Research tasks: planned, in-progress, completed, blocked
- Hypotheses: proposed, testing, proven, disproven
- Evidence: supports, contradicts, neutral, irrelevant

### 4. Integration Points
- Link research logs to specific sources and repositories
- Connect proof arguments to conclusion data
- Associate tasks with specific research questions
- Map hypotheses to evidence items

## Priority Features for Implementation

### Phase 1: Core Research Documentation
1. Research Log (searches, results, dates)
2. Simple task/todo tracking
3. Basic proof statement attachments

### Phase 2: Evidence Analysis
1. Evidence evaluation framework
2. Conflict documentation
3. Hypothesis tracking

### Phase 3: Advanced Features
1. Research plan templates
2. GPS compliance checking
3. Collaborative research features

## Standards Alignment Needed

### Professional Standards
- GPS (Genealogical Proof Standard)
- BCG Standards
- APG methodology
- Evidence Explained citation formats

### Work Product Templates
- Research logs (BCG format)
- Proof arguments (narrative style)
- Evidence analysis worksheets
- Research reports

## User Workflow Example

1. **Define Research Question**: "Who were the parents of John Smith b.1850?"
2. **Create Research Plan**: List repositories, record types to search
3. **Log Searches**: Document each search performed, results
4. **Analyze Evidence**: Evaluate each source/information item
5. **Track Hypotheses**: "Could be son of William & Mary Smith"
6. **Document Reasoning**: Write proof argument
7. **Record Conclusion**: Link to supporting evidence and analysis

## Next Steps

1. Define schema for research process entities
2. Create example data showing complete research workflow
3. Map to existing GEDCOM 7 structures where possible
4. Design extensions for missing capabilities
5. Test with real genealogical research scenarios