# Confidence Systems: Reality Check Analysis

## Current GRAMPS Implementation

### What GRAMPS Actually Does
```python
# Citation confidence: Simple 0-4 integer scale
class Citation:
    CONF_VERY_LOW = 0
    CONF_LOW = 1  
    CONF_NORMAL = 2  # default
    CONF_HIGH = 3
    CONF_VERY_HIGH = 4
    
    def __init__(self):
        self.confidence = Citation.CONF_NORMAL  # Always defaults to 2
```

### User Experience in GRAMPS
1. **Single dropdown** in citation editor: "Very Low" to "Very High"
2. **Applied to citations only** - not events, facts, or relationships directly
3. **Most users leave it at "Normal"** (default)
4. **Export**: Maps to GEDCOM QUAY (0-3) with adjustment logic
5. **Import**: Reads GEDCOM QUAY and maps back to 0-4 scale

### What This Actually Measures
- **Unclear semantics** - Could be source quality, information reliability, or conclusion confidence
- **One number conflates multiple concepts**
- **No guidance** on when to use different levels
- **Rarely used** in practice (most stay at default)

## Proposed Multidimensional Model

### What We're Proposing
```gedcom
1 BIRT
2 DATE 1850
2 _CONF
3 _TYPE conclusion           # What aspect we're rating
3 _VAL 85                   # Numeric confidence
3 _SCALE percent           # Scale interpretation
3 _BASIS                   # GPS methodology tracking
4 _EXHAUSTIVE Y            # Reasonably exhaustive research
4 _SOURCED Y              # Complete citation
4 _ANALYZED Y             # Analysis and correlation  
4 _CONFLICTS_RESOLVED Y   # Conflicts resolved
4 _WRITTEN Y              # Written conclusion
```

### User Experience Implications
- **Multiple dropdowns/fields** per fact
- **Professional genealogy concepts** (GPS criteria)
- **Complex decisions** about _TYPE values
- **Significantly more data entry**
- **Requires understanding** of evidence theory

## Reality Check: User Behavior

### What Genealogists Actually Do
1. **Most are hobbyists** - not professional researchers
2. **Want simple workflows** - add facts, sources, move on
3. **Rarely use current confidence** - default is "good enough"
4. **Focus on content** not metadata about content
5. **Overwhelmed by complexity** - too many fields = abandoned features

### Evidence from GRAMPS Usage
- **Citation confidence rarely changed** from default
- **Complex features underused** (source attributes, advanced dating)
- **Simple features heavily used** (basic facts, sources, media)

### Professional vs Hobbyist Divide
- **Professionals** might appreciate GPS tracking
- **Hobbyists** would find it overwhelming
- **Current simple systems** already underutilized

## Software Paradigm Analysis

### Current Software Approaches

#### GRAMPS: Citation-Level Confidence
- **Scope**: Citations only
- **Scale**: 0-4 (5 levels)
- **Usage**: Rare, defaults to middle
- **Mapping**: Direct to GEDCOM QUAY

#### RootsMagic: Fact-Level Stars  
- **Scope**: Facts and sources
- **Scale**: 1-5 stars (visual)
- **Usage**: Moderate (visual appeal helps)
- **Additional**: "Proof" flag on facts

#### Family Tree Maker: Multiple Mechanisms
- **Preferred facts**: Simple binary flag
- **Source quality**: Separate rating
- **Conflict resolution**: Manual notes
- **Usage**: Mixed adoption

#### TMG: Surety with Negative
- **Unique feature**: Negative surety (disproven)
- **Scale**: - to 3 (includes disproven)
- **Usage**: Power users only
- **Philosophy**: Evidence-based genealogy

### Pattern: Simpler = More Used
- **Binary flags** (preferred/not) used most
- **Star ratings** used moderately  
- **Complex scales** used rarely
- **Professional features** (like TMG surety) niche adoption

## The Implementation Reality

### What Multidimensional Requires
1. **UI Complexity**
   - Multiple confidence fields per fact
   - Type selection (source/information/evidence/conclusion)
   - GPS criteria checkboxes
   - Scale selection

2. **User Education** 
   - Understanding Evidence Explained concepts
   - GPS methodology training
   - When to use which _TYPE
   - What scales mean

3. **Software Changes**
   - Complete confidence UI redesign
   - Data model extensions
   - Migration logic for existing data
   - Cross-software compatibility issues

### Migration Challenges
- **Existing single values** → How to map to multidimensional?
- **User resistance** to increased complexity
- **Software vendor adoption** unlikely for hobbyist market
- **GEDCOM compatibility** - extensions vs standards

## Alternative Approaches

### 1. Enhanced Simple Model
Keep single scale but add:
- **Clear semantics**: "Overall reliability of this information"
- **Better UI**: Stars instead of dropdown
- **Guidance**: Help text explaining levels
- **Defaults**: Smart defaults based on source type

### 2. Optional Professional Mode
- **Default**: Simple single confidence (current GRAMPS model)
- **Advanced**: Multidimensional model for professionals
- **Migration**: Automatic expansion of simple → complex
- **UI**: Progressive disclosure (simple by default)

### 3. Specialized Extensions
- **Core**: Keep simple confidence in GEDCOM 7
- **Professional**: Separate extension for GPS tracking
- **Evidence**: Separate extension for evidence analysis
- **Modular**: Software can choose which to implement

### 4. Documentation Focus
Instead of complex confidence scales:
- **Better source citations** (already supported)
- **Research notes** (already supported)  
- **Evidence arguments** (could be extension)
- **Process documentation** (research log extension)

## Recommendation Analysis

### The Hard Truth
The multidimensional model is:
- **Theoretically superior** - properly separates concerns
- **Professionally valuable** - supports GPS methodology
- **Practically challenging** - high complexity, low adoption likely
- **Evolutionarily advanced** - beyond current user expectations

### Market Reality
- **99% of users** want simple tools
- **1% of users** want professional features
- **Software vendors** optimize for the 99%
- **Hobby market** drives genealogy software development

### Incremental Path Forward
1. **Start simple**: Enhanced single confidence scale
2. **Add guidance**: Clear semantics and help text
3. **Professional option**: Advanced mode for power users
4. **Evidence extension**: Separate extension for evidence theory
5. **Long-term evolution**: Gradual complexity as users mature

## Conclusion

**The proposed multidimensional model is beyond what most genealogy software users want or need right now.** 

While theoretically sound and professionally valuable, it represents a radical departure from current usage patterns. Most genealogists struggle with basic source citation - adding complex confidence dimensions would likely result in feature abandonment.

**Recommendation**: Design a simpler, more intuitive confidence system first, with a path toward professional features for advanced users.