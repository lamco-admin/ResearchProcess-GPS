# Extension Deep Dive Analysis

## 1. Tagging Systems Across Genealogy Software

### Current Implementations

#### GRAMPS
- **Independent Tag Objects**: Tags are first-class objects
- **Hierarchical**: Can be organized in tree structures
- **Universal Application**: Can tag any record type (person, event, source, etc.)
- **Properties**: Name, color, priority
- **Use Cases**: Research status, DNA matches, review needed, verified

#### Ancestry.com
- **Groups**: Similar to tags but limited functionality
- **Color Coding**: Visual markers on tree view
- **Custom Groups**: User-defined categories
- **Limitations**: Not exportable via GEDCOM

#### Family Tree Maker
- **Color Coding**: 16 predefined colors
- **Flags**: Research flags (follow-up, verified, etc.)
- **Custom Facts**: Can simulate tags via custom fact types
- **Export**: Lost in GEDCOM export

#### RootsMagic
- **Groups**: Named collections of people
- **Color Coding**: 15 color sets
- **To-Do Lists**: Task-based tagging
- **Research Logs**: Separate from main data

#### MyHeritage
- **Smart Matches**: System-generated tags
- **Labels**: User-defined markers
- **Privacy Settings**: Public/private tags

#### Legacy Family Tree
- **Research Tags**: Predefined research status
- **To-Do Items**: Attached to individuals
- **Color Coding**: Visual markers

### Common Tag Use Cases
1. **Research Status**: New, In Progress, Verified, Disputed
2. **DNA Matches**: Confirmed DNA, Y-DNA line, mtDNA line
3. **Privacy**: Living, Private, Public
4. **Data Quality**: Needs Review, Verified, Questionable
5. **Projects**: Smith Line, Immigration Research, Military
6. **Special Interest**: Direct Ancestor, Famous, Immigrant

### GEDCOM 7 Current State
- **NOTE with TYPE**: Can simulate simple tags
- **_CUSTOM**: Vendor-specific implementations
- **QUAY**: Quality/certainty (0-3) but not flexible tagging
- **RESN**: Restriction (privacy) but limited values

### Proposed gedcom-tags Extension

```gedcom
0 HEAD
1 SCHMA
2 TAG _TAG https://github.com/glamberson/gedcom-tags

0 @T1@ _TAG
1 NAME DNA Match
1 _COLOR #00FF00
1 _ICON dna
1 _DESC Confirmed through DNA testing

0 @T2@ _TAG  
1 NAME Research
1 _CHILD @T3@
1 _CHILD @T4@

0 @T3@ _TAG
1 NAME In Progress
1 _PARENT @T2@

0 @I1@ INDI
1 NAME John /Smith/
1 _TAG @T1@
2 DATE 15 JAN 2025
2 NOTE 3rd cousin match on Ancestry
1 _TAG @T3@
```

## 2. Formatting Capabilities Analysis

### GEDCOM 7 Current Formatting

#### Supported HTML Subset
- `<b>` bold
- `<i>` italic  
- `<u>` underline
- `<sup>` superscript
- `<sub>` subscript
- `<br>` line break
- `<a href="">` links

#### Example
```gedcom
0 @N1@ NOTE This is <b>bold</b> and <i>italic</i> text.
1 CONT With <sup>superscript</sup> notation.
1 CONT See <a href="https://example.com">source</a>.
```

### Software Formatting Capabilities

#### GRAMPS StyledText
- **Full Markup**: Bold, italic, underline, fonts, colors
- **Complex Styles**: Font families, sizes, backgrounds
- **Links**: Internal and external
- **Format**: Custom XML-like markup

#### Word Processors (FTM, RM)
- **RTF Support**: Full rich text in notes
- **Tables**: Some support structured data
- **Images**: Inline image support
- **Lists**: Bullets, numbering

#### Web-Based (Ancestry, MyHeritage)
- **HTML Editor**: WYSIWYG editing
- **Media Embedding**: Images, documents inline
- **Formatting Toolbar**: Standard web formatting

### Analysis: Is GEDCOM 7 Sufficient?

#### What's Missing
1. **Font Control**: No font family, size, color
2. **Alignment**: No center, right, justify
3. **Lists**: No bullets or numbering
4. **Tables**: No structured data presentation
5. **Styles**: No headers, emphasis levels

#### Use Cases Requiring More
1. **Transcriptions**: Preserving original formatting
2. **Research Reports**: Professional documentation
3. **Source Citations**: Complex formatting standards
4. **Correspondence**: Email/letter formatting

#### Recommendation
GEDCOM 7's HTML subset covers 80% of needs. Rather than extending, advocate for:
1. Full HTML5 subset adoption in GEDCOM 7.1
2. Markdown support as alternative
3. Focus on structured data over formatting

## 3. Confidence/Certainty Systems

### Current Standards

#### GEDCOM 7 QUAY (Quality)
```
0 = Unreliable evidence or estimated data
1 = Questionable reliability
2 = Secondary evidence
3 = Direct and primary evidence
```

#### Evidence Explained (Mills)
- **Sources**: Original, Derivative, Authored
- **Information**: Primary, Secondary, Undetermined
- **Evidence**: Direct, Indirect, Negative

#### Genealogical Proof Standard (GPS)
1. Reasonably exhaustive research
2. Complete citation of sources
3. Analysis and correlation
4. Resolution of conflicts
5. Written conclusion

### Software Implementations

#### GRAMPS (0-4 scale)
```
0 = Very Low
1 = Low  
2 = Normal
3 = High
4 = Very High
```

#### RootsMagic (1-5 stars)
- Proof field on facts
- Source quality ratings
- Surety levels

#### Family Tree Maker
- Preferred facts
- Confidence on relationships
- Source reliability ratings

#### Master Genealogist (TMG)
- Surety values (-, 0, 1, 2, 3)
- Negative surety for disproven
- Separate citation surety

### Issues with Current Systems
1. **Conflated Concepts**: Source quality vs conclusion confidence
2. **Limited Granularity**: 4-5 levels insufficient
3. **No Negative Evidence**: Can't record "disproven"
4. **Binary Application**: Only on certain structures

### Proposed Confidence Extension

```gedcom
0 HEAD
1 SCHMA
2 TAG _CONF https://github.com/glamberson/gedcom-confidence

0 @I1@ INDI
1 NAME John /Smith/
2 _CONF
3 _VAL 85
3 _SCAL percent
3 _METH GPS-based analysis
3 NOTE Multiple sources confirm

1 BIRT
2 DATE ABT 1850
2 _CONF  
3 _VAL 2
3 _SCAL 0-4
3 _RSLT questionable
3 NOTE Only census ages, vary ±5 years

1 FAMC @F1@
2 _CONF
3 _VAL -1
3 _SCAL proven-false
3 NOTE DNA disproves relationship
```

## 4. Enhanced Relationships Deep Dive

### Current GEDCOM 7 Relationships
- **INDI-FAM**: Parent-child, spouses only
- **ASSO**: Associated person with TYPE
- **Limited Types**: Godparent, Witness, Informant

### Software Relationship Models

#### GRAMPS PersonRef
- **Any-to-Any**: Between any two people
- **Types**: Godparent, Employer, Friend, DNA match
- **Attributes**: Confidence, dates, notes
- **Directional**: A→B different from B→A

#### Ancestry ThruLines
- **DNA Relationships**: Genetic distance
- **Hypothetical**: Potential connections
- **Confidence Scores**: Statistical probability

#### FamilySearch Relationships
- **Biological**: Birth, adopted, foster
- **Step Relations**: Step-parent, step-child
- **Guardian**: Legal guardian
- **Time-Bounded**: Start/end dates

#### WikiTree Connections
- **DNA Confirmation**: Verified connections
- **Relationship Degrees**: 1st cousin 2x removed
- **Relationship Paths**: Multiple paths tracked

### Relationship Needs Analysis

#### Universal Needs (All Software)
1. **Non-Family Relations**: Godparents, guardians, witnesses
2. **DNA Matches**: Genetic relationships with cM values
3. **Time Boundaries**: When relationships began/ended
4. **Confidence Levels**: Certainty of connection

#### GRAMPS-Specific Needs
1. **PersonRef Model**: Direct person-to-person links
2. **Arbitrary Types**: User-defined relationship types
3. **Attributes**: Additional relationship properties
4. **Research Relations**: "Possibly same person"

#### Specialized Needs
1. **Adoption**: Multiple parent sets with types
2. **Slavery**: Enslaver/enslaved relationships
3. **Noble Titles**: Succession relationships
4. **Military**: Command structure

### Implementation Alternatives

#### Option 1: Simple Extension
```gedcom
0 @I1@ INDI
1 _REL @I2@
2 _TYPE Godfather
2 DATE FROM 1850 TO 1870
2 _CONF 3
```

#### Option 2: Relationship Records
```gedcom
0 @R1@ _RELN
1 _PERS @I1@
2 ROLE Godchild
1 _PERS @I2@  
2 ROLE Godfather
1 DATE FROM 1850
1 TYPE Religious
```

#### Option 3: Enhanced ASSO
```gedcom
0 @I1@ INDI
1 ASSO @I2@
2 TYPE Godfather
2 _STAR 1850
2 _END 1870
2 _CONF high
2 _DNA 1200 cM
```

### Recommendation

A two-tier approach:
1. **Basic**: Extend ASSO with dates and confidence
2. **Advanced**: New _RELN record for complex cases

This provides:
- Simple upgrade path
- Backward compatibility  
- Full flexibility for advanced users
- Natural GRAMPS mapping

## Conclusions

### Tags Extension
**Verdict**: Low-hanging fruit, proceed
- Universal need across all software
- Simple data model
- High user value
- Easy backward compatibility

### Formatting Extension  
**Verdict**: Not needed as extension
- GEDCOM 7 HTML subset adequate for 80% 
- Better to advocate for GEDCOM 7.1 improvements
- Focus effort elsewhere

### Confidence Extension
**Verdict**: Medium complexity, high value
- Addresses real GPS compliance needs
- Fixes GEDCOM's conflated concepts
- Supports negative evidence
- More complex than appears

### Enhanced Relationships
**Verdict**: High complexity, high value
- Universal need but varying implementations
- Requires careful design for compatibility
- Consider phased approach
- Could become GEDCOM 8 feature