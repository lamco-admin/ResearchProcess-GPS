# Evidence Extension Deep Analysis: Why It May Already Solve the Confidence Problem

## What the Evidence Extension Actually Does

### Core Problem It Solves
**The "Which John Smith?" problem** - Traditional genealogy software forces premature conclusions by requiring immediate attachment of evidence to specific persons. The Evidence extension allows evidence to "float" until proper identification is made.

### The _CONF Structure's Specific Purpose
**Not general confidence** - The Evidence extension's `_CONF` has a very specific, narrow purpose:

```gedcom
1 _EVID @E1@
2 _CONF 3    # "How confident am I that this evidence applies to this specific person?"
```

**Definition**: "A numerical value (0-5) indicating certainty about an association between evidence and a person."

This is **NOT** about:
- Source quality (original vs derivative)
- Information type (primary vs secondary) 
- Evidence strength (direct vs indirect)
- Genealogical Proof Standard compliance

It's **ONLY** about: "Does this piece of evidence describe this person?"

## Real-World Usage Patterns

### Use Case 1: Multiple Candidate Persons
```gedcom
# Same evidence, three possible Johns
0 @I10@ INDI
1 _EVID @E10@
2 _CONF 2    # 40% chance it's this John

0 @I11@ INDI  
1 _EVID @E10@
2 _CONF 4    # 80% chance it's this John (most likely)

0 @I12@ INDI
1 _EVID @E10@
2 _CONF 1    # 20% chance it's this John
```

### Use Case 2: Conflicting Evidence Resolution
```gedcom
# Person with all evidence, different confidence levels
0 @I1@ INDI
1 _EVID @E30@    # Census age 40 in 1850
2 _CONF 3        # Moderate confidence (ages often rounded)
1 _EVID @E31@    # Death cert age 76 in 1885
2 _CONF 3        # Moderate confidence (informant may estimate)
1 _EVID @E32@    # Baptism born 10 May 1810
2 _CONF 5        # High confidence (primary source)
```

**Key Insight**: This isn't about the quality of the sources themselves, but about how confidently each piece of evidence points to this specific person.

## What This Covers vs What It Doesn't

### What Evidence _CONF Covers ✅
- **Person identification uncertainty**: "Is this the right John Smith?"
- **Evidence applicability**: "Does this census record describe this person?"
- **Competing hypotheses**: Multiple persons can claim same evidence with different confidence
- **Research process**: Shows reasoning behind person-evidence associations
- **Conflict handling**: Same person can have conflicting evidence with different confidence levels

### What Evidence _CONF Doesn't Cover ❌
- **Source quality assessment**: Is this source reliable?
- **Information type evaluation**: Primary vs secondary information?
- **Evidence strength analysis**: Direct vs indirect vs negative evidence?
- **GPS methodology tracking**: Research exhaustiveness, conflict resolution process
- **Conclusion confidence**: Overall certainty about facts/relationships
- **Cross-fact confidence**: Confidence about events, dates, places separate from person identity

## Comparison with "Ideal" Multidimensional Model

### Evidence Extension Approach
- **Single dimension**: Person-evidence association confidence
- **Clear semantics**: "How sure am I this evidence applies to this person?"
- **Practical workflow**: Supports real research uncertainty
- **Implementation friendly**: Simple 0-5 scale, clear UI implications

### Proposed Multidimensional Approach
- **Multiple dimensions**: Source, information, evidence, conclusion confidence
- **Complex semantics**: GPS methodology, multiple scales, professional concepts
- **Theoretical completeness**: Separates all quality concerns
- **Implementation challenging**: Complex UI, user education required

## The Critical Question: What Gap Remains?

### Evidence Extension Handles
1. **"Which person?"** - Multiple candidates for same evidence ✅
2. **"How confident in this association?"** - 0-5 scale for applicability ✅  
3. **"Research documentation"** - _RDOC for analysis ✅
4. **"Floating evidence"** - Evidence exists before person assignment ✅
5. **"GPS compliance"** - Structure supports all 5 GPS elements ✅

### Potential Gaps
1. **Source quality independent of person identification** - "This is definitely about John Smith, but the source itself is questionable"
2. **Fact-level confidence separate from person identification** - "This is John Smith, but I'm unsure about the birth date in this record"
3. **Cross-evidence conclusion confidence** - "Based on all evidence, how confident am I about John's birth year?"
4. **Professional methodology tracking** - GPS criteria documentation beyond _RDOC prose

## Real Usage Reality Check

### Current Software Patterns
- **Most confidence systems underused** - Users stick with defaults
- **Simple binary decisions preferred** - "This applies" vs "This doesn't apply"
- **Evidence Extension _CONF maps to real workflow** - "How sure am I this evidence is about this person?" is a question researchers actually ask

### Evidence Extension Advantage
- **Matches mental model**: Researchers naturally think "Which person does this evidence describe?"
- **Solves real problems**: The "premature conclusion" problem is genuine
- **Progressive enhancement**: Works with existing GEDCOM, adds capabilities
- **Implementation path**: Clear UI implications (confidence slider on evidence links)

## Assessment: Is a Separate Confidence Extension Needed?

### Arguments Against Separate Extension
1. **Evidence _CONF covers the critical use case** - Person identification uncertainty
2. **Additional confidence dimensions rarely used** - Even current simple systems underutilized  
3. **Complexity vs benefit tradeoff** - Multidimensional model too complex for most users
4. **GPS compliance already possible** - _RDOC supports documentation of methodology
5. **Source quality already exists** - GEDCOM 7 QUAY handles source assessment

### Arguments For Separate Extension
1. **Conceptual gap exists** - Evidence _CONF only handles person association, not source/conclusion quality
2. **Professional needs** - Advanced researchers want GPS methodology tracking
3. **Cross-fact confidence missing** - "How confident am I about this birth date?" separate from person identification
4. **Standards completeness** - GEDCOM should support full spectrum of confidence assessment

### The Synthesis
**Evidence Extension _CONF is actually well-designed for its specific purpose** - it solves the "which person" problem that traditional software handles poorly. It's not trying to be a general confidence system.

**The question becomes**: Do we need general confidence assessment beyond person-evidence association?

## Recommendation

**The Evidence Extension's _CONF is good design, not a limitation.** It has:
- Clear, specific purpose (person-evidence association)
- Matches real research workflows
- Solves genuine software problems
- Implementation-friendly semantics

**A separate confidence extension would serve different purposes:**
- General fact/conclusion confidence
- Source quality independent of person identification  
- Professional methodology tracking
- GPS criteria documentation

**However**, given the analysis of user behavior and software adoption patterns, **a separate confidence extension may be solving problems most users don't have**.

**Conclusion**: The Evidence Extension already handles the most important confidence use case (person identification). Additional confidence dimensions should be carefully evaluated against real user needs rather than theoretical completeness.

## Next Steps for Evaluation

1. **Test Evidence Extension in practice** - Does the person-evidence confidence model handle most real research scenarios?
2. **Identify specific gaps** - What confidence questions can't be answered with Evidence _CONF + _RDOC documentation?
3. **Survey professional genealogists** - Do they need confidence assessment beyond person identification?
4. **Consider incremental enhancement** - Could Evidence Extension be extended rather than creating separate system?

The Evidence Extension may already be the right level of confidence support for most genealogical research.