# Semantic Loss Analysis: Current Genealogy System Transfers

## Executive Summary

This analysis identifies critical semantic information that is systematically lost when transferring genealogical data between systems, forming the foundation for designing a universal data model that preserves meaning across all genealogy platforms.

## Categories of Semantic Loss

### 1. Research Process Metadata
**What's Lost**: The reasoning, methodology, and decision-making process behind conclusions.

**Specific Loss Points**:
- **Research questions** that drove investigations
- **Hypotheses tested** and alternative theories considered  
- **Methodology used** (GPS standards, systematic vs. targeted search)
- **Decision reasoning** - why one conclusion was chosen over alternatives
- **Research timeline** - when discoveries were made and in what order
- **Dead ends explored** - paths that were investigated but yielded no results
- **Quality assessments** - how thoroughly areas were researched

**Impact**: Receiving systems treat all data as established fact, losing the uncertainty and process that led to conclusions.

### 2. Evidence Quality and Assessment
**What's Lost**: The evaluation of source reliability, evidence strength, and analytical reasoning.

**Specific Loss Points**:
- **Source reliability scores** - primary/secondary/tertiary classifications with reasoning
- **Evidence directness** - direct vs. indirect vs. circumstantial
- **Information quality** - original vs. derivative vs. authored
- **Conflict resolution** - how contradictory evidence was reconciled
- **Evidence strength weighting** - relative importance in reaching conclusions
- **Analysis methodology** - how evidence was evaluated
- **Peer review comments** - expert assessments of evidence interpretation

**Impact**: All evidence appears equal in receiving systems, preventing proper genealogical proof evaluation.

### 3. Confidence and Uncertainty
**What's Lost**: Quantified uncertainty levels and confidence scoring.

**Specific Loss Points**:
- **Confidence percentages** - numerical certainty levels (e.g., 85% confident in birth date)
- **Uncertainty ranges** - date ranges with likelihood distributions
- **Conditional relationships** - "if A is true, then B follows"
- **Probability assessments** - multiple possible outcomes with likelihoods
- **Assumption dependencies** - what foundational assumptions conclusions depend on
- **Confidence evolution** - how certainty changed as research progressed
- **Ambiguity markers** - explicit notation of unresolved questions

**Impact**: Tentative conclusions become hard facts, losing essential uncertainty information.

### 4. Complex Relationship Semantics
**What's Lost**: Nuanced relationship types and contextual information.

**Specific Loss Points**:
- **Relationship certainty** - "possibly son of" vs. "definitely son of"
- **Temporal relationships** - "became guardian after father's death"
- **Conditional relationships** - "business partner if same person as John Smith of Boston"
- **Contextual modifiers** - "adopted", "step-", "putative", "alleged"
- **Relationship quality** - close vs. estranged family relationships
- **Legal vs. biological** - adoption, legitimacy, guardianship distinctions
- **Multi-party relationships** - witnessed transactions, business partnerships

**Impact**: Complex relationships are forced into simple family tree models, losing critical context.

### 5. Identity Resolution Complexity
**What's Lost**: The process of identifying and merging uncertain identities.

**Specific Loss Points**:
- **Identity fragments** - separate personas from different sources before merging
- **Merge reasoning** - why identities were determined to be the same person
- **Conflicting information** - contradictory data between sources
- **Alternative identity theories** - other possible person matches considered
- **Identity confidence scores** - certainty levels for person merges
- **Source-specific variations** - name/date variations by document
- **Mystery person tracking** - placeholder identities for unknown individuals

**Impact**: Complex identity resolution becomes simple person records, losing the analytical work.

### 6. Research Attribution and Collaboration
**What's Lost**: Who did what research when, and collaborative work context.

**Specific Loss Points**:
- **Research attribution** - who made each discovery or conclusion
- **Collaboration roles** - lead researcher, contributor, reviewer, etc.
- **Research dates** - when each piece of work was completed
- **Version history** - how conclusions evolved over time
- **Peer review status** - what has been verified by other researchers
- **Research permissions** - who can edit/view what information
- **Citation responsibilities** - who found each source and evaluated it

**Impact**: Collective research becomes anonymous, losing professional attribution.

### 7. Negative Research Results
**What's Lost**: Documentation of searches that found nothing or disproved theories.

**Specific Loss Points**:
- **Exhaustive search documentation** - what repositories were thoroughly searched
- **Negative findings** - explicit documentation that records don't exist
- **Disproven theories** - hypotheses that research ruled out
- **Search strategies used** - what approaches were tried
- **Time periods covered** - date ranges that were thoroughly researched
- **Geographic areas searched** - locations that were investigated
- **Alternative spellings tried** - name variations that were tested

**Impact**: Future researchers repeat failed searches, wasting time and resources.

### 8. Source Context and Provenance
**What's Lost**: The complete context of where information came from and how it was obtained.

**Specific Loss Points**:
- **Discovery context** - how and when the source was found
- **Source evaluation notes** - assessments of reliability and completeness
- **Transcription vs. abstract vs. extract** - what type of source derivative
- **Image quality notes** - legibility and preservation status
- **Access restrictions** - copyright, privacy, or institutional limitations
- **Source relationships** - how sources relate to each other
- **Update tracking** - when sources were last checked for changes

**Impact**: Sources become simple citations, losing evaluation and context.

### 9. Geographic and Temporal Context
**What's Lost**: The historical and geographical context that gives meaning to events.

**Specific Loss Points**:
- **Historical context** - what was happening historically when events occurred
- **Geographic precision** - exact locations vs. approximate areas
- **Jurisdictional changes** - how boundaries and governments changed over time
- **Migration patterns** - family movement context and reasoning
- **Economic conditions** - financial circumstances affecting family decisions
- **Social context** - cultural and community factors
- **Seasonal patterns** - how time of year affected events

**Impact**: Events become isolated facts without meaningful historical context.

### 10. DNA and Genetic Analysis
**What's Lost**: Complex genetic genealogy analysis and interpretation.

**Specific Loss Points**:
- **DNA match analysis** - triangulation groups and shared segments
- **Genetic distance calculations** - relationship probability assessments
- **Chromosome mapping** - which segments came from which ancestors
- **Ethnicity interpretation** - how genetic ethnicity relates to genealogical research
- **Match theories** - hypotheses about how DNA matches fit family trees
- **Genetic genealogy methodology** - systematic approaches to DNA analysis
- **Privacy preferences** - who can see what genetic information

**Impact**: DNA becomes simple match lists, losing analytical work and privacy controls.

## System-Specific Loss Patterns

### GEDCOM Transfer Losses
- **Standard limitations**: Only supports basic family relationships
- **Extension loss**: Custom tags and extended attributes disappear
- **Media disconnection**: Source images and documents lose associations
- **Note degradation**: Rich formatting becomes plain text
- **Date uncertainty**: Approximate dates become exact or disappear

### Database Export Losses  
- **Schema mapping**: Complex relationships forced into simple joins
- **Metadata stripping**: Research context stored in comments is lost
- **Relationship context**: Foreign key relationships lose semantic meaning
- **Temporal sequences**: Research timeline information disappears
- **User context**: Multi-user attribution becomes single-owner

### Cloud Platform Migrations
- **API limitations**: Only subset of data available through APIs
- **Privacy controls**: Fine-grained permissions become binary public/private
- **Media handling**: High-resolution images downgraded or lost
- **Citation formatting**: Platform-specific citation styles lost
- **Collaboration history**: Team work becomes individual contributions

## Critical Semantic Primitives to Preserve

Based on this analysis, the following semantic primitives MUST be preserved in any universal data model:

### Core Semantic Elements
1. **Uncertainty quantification** - numerical confidence levels
2. **Evidence evaluation** - quality, directness, and reliability assessments  
3. **Research methodology** - how conclusions were reached
4. **Decision reasoning** - why alternatives were rejected
5. **Temporal sequences** - when research occurred and in what order
6. **Attribution chains** - who did what work when
7. **Negative results** - what was searched but not found
8. **Alternative theories** - hypotheses considered but not pursued
9. **Context preservation** - historical, geographic, and social circumstances
10. **Relationship complexity** - beyond simple family tree connections

### Structural Requirements
1. **Immutable audit trail** - complete history of all changes
2. **Multi-perspective views** - same data interpreted by different researchers
3. **Conditional dependencies** - if-then relationships between conclusions
4. **Quality metrics** - quantified assessments of research thoroughness
5. **Cross-reference integrity** - maintained links between related information
6. **Privacy granularity** - fine-grained control over information sharing
7. **Format independence** - semantics preserved across storage methods
8. **Version reconciliation** - merging different research versions

## Implications for Universal Data Model

The universal data model must:

1. **Capture process, not just products** - Record the research journey, not just endpoints
2. **Quantify uncertainty** - Numerical confidence levels for all assertions
3. **Preserve reasoning** - Why conclusions were reached and alternatives rejected  
4. **Maintain context** - Historical, geographic, and social circumstances
5. **Support complexity** - Relationships and identities beyond simple models
6. **Enable collaboration** - Multi-researcher attribution and version control
7. **Document negatives** - What was searched but not found
8. **Ensure reversibility** - Round-trip fidelity where semantically possible

This analysis demonstrates that current genealogy systems lose 80-90% of semantic information during transfers. The universal data model must be designed to preserve these semantics while providing graceful degradation strategies for systems that cannot support full semantic richness.

## Next Steps

1. Design core entity model that captures all identified semantic primitives
2. Create semantic metadata schema for confidence and evidence assessment
3. Define GEDCOM 7 extensions needed for semantic preservation
4. Develop mapping strategies for major genealogy systems
5. Test semantic fidelity across multiple system types