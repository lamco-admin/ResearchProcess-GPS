# GEDCOM Extension Categories and Development Roadmap

## Extension Classification

### Phase 1: Data Model Extensions 🏗️
**Purpose**: Enhance GEDCOM data interchange capabilities  
**Target**: Existing genealogy software adoption  
**Implementation**: GEDCOM structure extensions only  

**Completed:**
- ✅ **Occurrences (_OCUR)**: Enhanced event documentation with roles and relationships
- ✅ **Evidence (_EVID)**: Floating evidence containers with identifiers  
- ✅ **Tags (_TAG)**: Universal categorization system with colors and properties

**In Progress:**
- 🔄 **Links (_LINK)**: Enhanced relationship documentation beyond ASSO

**Status**: 4/4 data model extensions (Phase 1 complete with LINK)

### Phase 2: Research/GPS Extensions 🔬  
**Purpose**: Professional genealogy application capabilities  
**Target**: Advanced research workflow support  
**Implementation**: Requires new software features + GEDCOM extensions

**Research/GPS Capabilities Identified:**

1. **GPS-Compliant Research Process Tracking**
   - Research plan documentation
   - Search activity logging  
   - Negative evidence recording
   - Exhaustive search verification
   - **Gap**: Traditional software lacks research planning tools

2. **Evidence Correlation and Analysis Tools**
   - Automated evidence matching
   - Conflict detection and resolution
   - Identity correlation across sources
   - Proof argument construction
   - **Gap**: Manual correlation in current software

3. **Professional Genealogy Workflow Support**
   - Client research documentation
   - Professional report generation
   - GPS compliance verification
   - Peer review and collaboration
   - **Gap**: Hobbyist focus in existing software

4. **Advanced Research Methodology**
   - Source quality assessment frameworks
   - Information type classification tools
   - Evidence strength evaluation
   - Multi-dimensional confidence modeling
   - **Gap**: Basic confidence systems only

5. **Research Documentation Standards**
   - Standardized proof arguments
   - Research log templates
   - Citation quality verification
   - Source analysis frameworks
   - **Gap**: Free-form notes only

## Key Distinction

### Data Model Extensions (Phase 1)
```gedcom
# Can be implemented by existing software
0 @I1@ INDI
1 _TAG @T1@        # New structure, existing software paradigm
2 COLOR blue
2 PRIORITY high
```

### Research/GPS Extensions (Phase 2)  
```
# Requires new application capabilities
- Research planning interface
- Evidence correlation algorithms  
- GPS compliance checking
- Professional report generation
- Collaborative research tools
```

## Current Project Status

**Phase 1 Focus**: Complete fundamental data model extensions
- **Rationale**: Provides immediate value to existing software
- **Adoption Path**: Minimal software changes required
- **Compatibility**: Works with traditional genealogy workflows

**Phase 2 Deferred**: Research/GPS capabilities require significant software development
- **Rationale**: Beyond scope of pure GEDCOM extensions
- **Development Need**: New application features, not just data structures
- **Market Gap**: Traditional software lacks research-focused tools

## Implementation Strategy

### Phase 1: Data Interchange Focus
1. **Registry Submission**: Submit all data model extensions to GEDCOM registry
2. **Software Adoption**: Work with GRAMPS, other software for implementation
3. **Testing**: Verify cross-software compatibility
4. **Documentation**: Provide clear migration guides

### Phase 2: Research Application Development  
1. **Market Analysis**: Survey professional genealogy needs
2. **Prototype Development**: Build research-focused genealogy tools
3. **GPS Integration**: Implement Genealogical Proof Standard compliance
4. **Professional Features**: Advanced research workflow support

## File Organization

### Phase 1 Documentation (Data Model)
- Extension specifications and registries
- Implementation guides and examples
- Software compatibility testing
- Migration documentation

### Phase 2 Documentation (Research/GPS) 🔬
**Tagged as Research/GPS:**
- `CONFIDENCE_CERTAINTY_ANALYSIS.md` 🔬
- `GPS_COMPLIANT_RESEARCH_TRACKING.md` 🔬  
- `PROFESSIONAL_GENEALOGY_WORKFLOWS.md` 🔬
- `EVIDENCE_CORRELATION_METHODS.md` 🔬
- `RESEARCH_METHODOLOGY_EXTENSIONS.md` 🔬

## Next Steps

**Immediate**: Complete Phase 1 with LINK extension
**Medium-term**: Monitor registry PR reviews, ensure software adoption  
**Long-term**: Evaluate Phase 2 research application development

---

**Note**: The 🔬 tag indicates Research/GPS extensions requiring application capabilities beyond traditional genealogy software paradigms.