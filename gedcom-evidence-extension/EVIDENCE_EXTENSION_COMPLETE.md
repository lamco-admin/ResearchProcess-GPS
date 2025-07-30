# GEDCOM Evidence Extension - Complete Documentation Package

## Overview

This package provides comprehensive documentation for the GEDCOM 7 Evidence Extension, based on 15 years of community discussions and real-world genealogical research needs.

## Documentation Structure

### 1. Core Documentation
- **[README.md](README.md)** - Overview and quick start
- **[specification.md](specification.md)** - Technical specification and YAML definitions
- **[design-rationale.md](design-rationale.md)** - Why we made these design choices

### 2. Supporting Documentation  
- **[community-insights.md](community-insights.md)** - 15 years of discussions analyzed
- **[use-cases.md](use-cases.md)** - Real-world examples and scenarios
- **[glossary.md](glossary.md)** - Terms and concepts defined
- **[migration-guide.md](migration-guide.md)** - Implementation guidance

### 3. YAML Definitions
Located in `/yaml/` directory:
- `_EVID.yaml` - Evidence container record
- `_RDOC.yaml` - Research documentation record
- `_RACT.yaml` - Research activity record
- `_ID.yaml` - Identifier structure
- `_CONF.yaml` - Confidence level structure
- `_CONC.yaml` - Conclusion structure
- `_FIND.yaml` - Finding structure

## Key Innovation: Floating Evidence with Identifiers

The core innovation addresses the "Which John Smith?" problem:

```gedcom
# Evidence floats with identifiers
0 @E1@ _EVID
1 _ID John Smith
1 _ID son of Mary
1 _ID aged 40
1 SOUR @S1@

# Multiple persons can claim it
0 @I1@ INDI
1 _EVID @E1@
2 _CONF 3  # 60% sure

0 @I2@ INDI
1 _EVID @E1@
2 _CONF 2  # 40% sure
```

## Community Foundation

This extension synthesizes insights from:
- **594 discussions** with 4,970 replies
- **318 wiki pages** from BetterGEDCOM
- **74 contributors** over 15 years
- **Professional genealogists** and software developers

## Key Features

1. **Evidence Independence** - Evidence exists separately from conclusions
2. **Identity Uncertainty** - Same evidence can belong to multiple people
3. **Research Process** - Documents both positive and negative searches
4. **GPS Compliance** - Supports all five elements of the standard
5. **Backward Compatible** - Enhances without breaking GEDCOM

## Design Principles

1. **Simple Over Complex** - Flat identifiers instead of nested structures
2. **Optional Adoption** - Enhance don't replace existing GEDCOM
3. **Evidence-Based** - Separate what sources say from what we conclude
4. **Community-Driven** - Based on real user needs, not theory

## Implementation Strategy

### For Developers
1. Start with basic parsing/preservation
2. Add evidence creation and linking
3. Implement research documentation
4. Build correlation tools

### For Users
1. Begin extracting evidence from key sources
2. Add confidence levels to associations
3. Document research reasoning
4. Track research activities

## Why This Succeeds Where Others Failed

### Previous Attempts
- **GenTech** (1990s) - Too abstract, "assertions" didn't match mental models
- **GEDCOM X** (2012) - Complex, limited to FamilySearch
- **BetterGEDCOM** (2010-2013) - Never implemented, too many visions

### This Extension
- **Concrete** - Evidence and identifiers match how genealogists think
- **Simple** - Easy to understand and implement incrementally  
- **Flexible** - Supports multiple research styles
- **Practical** - Solves real problems users face daily

## Measuring Success

### Technical Metrics
- Zero data loss in round-trip transfers
- Performance impact < 5%
- Validation accuracy > 95%

### Adoption Metrics  
- 3+ major applications in 18 months
- Active developer community
- Positive user feedback

### Impact Metrics
- Reduced citation data loss
- Better research documentation
- GPS compliance enabled

## Next Steps

1. **Community Review** - Gather feedback on specification
2. **Reference Implementation** - Create sample code
3. **Vendor Engagement** - Work with software developers
4. **User Education** - Develop training materials

## Conclusion

After 15 years of discussion, we finally have a practical solution to the evidence/conclusion problem. This extension enables genealogy software to support how research actually works - with uncertain identities, floating evidence, and documented reasoning.

The key insight: Evidence must exist independently with identifiers until we determine through research which person it describes. This simple concept, implemented through three record types, transforms genealogical data exchange.

## Contributing

This specification is open for community input. The genealogy community created this need through years of discussion - the community should shape its solution.

## Acknowledgments

Special recognition to the BetterGEDCOM community whose years of thoughtful discussion made this possible. Your vision of evidence-based genealogy in software is finally becoming reality.