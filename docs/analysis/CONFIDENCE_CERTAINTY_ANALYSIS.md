# Confidence/Certainty Analysis: Standards vs Implementation 🔬

**Category**: Research/GPS Extension (Phase 2)  
**Note**: Requires advanced application capabilities beyond traditional genealogy software

## Professional Standards Expectations

### Genealogical Proof Standard (GPS)
The GPS doesn't specify numeric confidence levels but requires:
1. **Reasonably exhaustive research**
2. **Complete citation of sources**
3. **Analysis and correlation of evidence**
4. **Resolution of conflicting evidence**
5. **Written conclusion**

**Key Insight**: GPS focuses on PROCESS, not numeric confidence

### Evidence Explained (Mills)
Three-tier evaluation:
1. **Sources**: Original / Derivative / Authored
2. **Information**: Primary / Secondary / Undetermined  
3. **Evidence**: Direct / Indirect / Negative

**Key Insight**: Quality assessment is multidimensional, not single score

### Academic History Standards
- Footnotes with full citations
- Historiographical context
- Source criticism methodology
- No numeric confidence scores

## Current Software Implementations

### GEDCOM 7 (QUAY)
```
0 = Unreliable evidence or estimated data
1 = Questionable reliability  
2 = Secondary evidence
3 = Direct and primary evidence
```
**Problem**: Conflates source quality with evidence type

### GRAMPS (0-4 Scale)
```
0 = Very Low
1 = Low
2 = Normal (default)
3 = High
4 = Very High
```
**Applied to**: Citations only
**From Database**: CONF_VERY_LOW through CONF_VERY_HIGH constants

### RootsMagic (1-5 Stars)
- Visual star rating
- "Proof" field on facts
- Surety on source quality
- Applied to: Facts, sources, relationships

### Family Tree Maker
- "Preferred" fact flag
- Confidence on specific assertions
- Source quality ratings
- Resolution notes for conflicts

### The Master Genealogist (Surety)
```
- = Negative (disproven)
0 = Not evaluated
1 = Possible
2 = Probable  
3 = Proven
```
**Unique**: Includes negative surety for disproven

### Legacy Family Tree
- 0-3 surety scale
- Separate source quality
- Applied per assertion

## Analysis: The Confidence Problem

### 1. Conflated Concepts
Current systems mix:
- Source quality (original vs derivative)
- Information type (primary vs secondary)
- Evidence strength (direct vs indirect)
- Conclusion confidence (how sure we are)

### 2. What Users Actually Need
Based on GPS and practice:
- **Source Quality**: Is this original/derivative?
- **Information Type**: Was informant present?
- **Evidence Type**: Direct/indirect/negative
- **Analysis Quality**: How thorough was research?
- **Conflict Resolution**: How were conflicts handled?
- **Conclusion Confidence**: Overall assessment

### 3. Why Single Scores Fail
Example: 1850 Census
- Source: Original (high quality)
- Information: Primary for residence, Secondary for age
- Evidence: Direct for residence, Indirect for birth
- Confidence: Varies by assertion

One "confidence score" can't capture this complexity.

## Proposed gedcom-confidence Extension

### Design Principles
1. **Separate concerns** - Don't conflate different quality measures
2. **Flexible application** - Allow on any assertion
3. **Support GPS** - Enable process documentation
4. **Include negative** - Allow "disproven" marking

### Structure
```gedcom
0 @I1@ INDI
1 NAME John /Smith/
2 _CONF
3 _TYPE conclusion           # What aspect
3 _VAL 85                   # Numeric value
3 _SCALE percent           # Scale used
3 _BASIS                   # Why this confidence
4 _EXHAUSTIVE Y            # GPS criterion 1
4 _SOURCED Y              # GPS criterion 2  
4 _ANALYZED Y             # GPS criterion 3
4 _CONFLICTS_RESOLVED Y   # GPS criterion 4
4 _WRITTEN Y              # GPS criterion 5

1 BIRT
2 DATE 1850
2 _CONF
3 _TYPE information
3 _VAL secondary
3 NOTE Age from 1870 census

1 FAMC @F1@
2 _CONF
3 _TYPE evidence
3 _VAL negative            # Disproven
3 NOTE DNA test excludes
```

### Benefits Over Current Systems
1. **Multidimensional** - Separates different quality aspects
2. **GPS Compliant** - Documents process criteria
3. **Negative Evidence** - Supports "disproven"
4. **Flexible** - Works on any structure
5. **Backward Compatible** - Doesn't break GEDCOM 7

### Migration Strategy
- Map GEDCOM QUAY → _CONF with _TYPE source
- Map software confidence → _CONF with _TYPE conclusion
- Preserve original values in _SCALE

## Recommendation

**CREATE CONFIDENCE EXTENSION**

But make it sophisticated enough to handle professional genealogy needs, not just another 0-5 scale. The extension should support GPS methodology and multidimensional quality assessment.