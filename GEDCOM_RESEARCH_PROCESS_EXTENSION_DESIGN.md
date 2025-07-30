# GEDCOM Research Process Extension Design

## Overview

The gedcom-research-process extension enables tracking of genealogical research activities, plans, and logs within GEDCOM files. This addresses the need to document the research process itself, not just the conclusions.

## Motivation

Professional genealogists maintain research logs, task lists, and correspondence records that are crucial for:
- Documenting negative searches (what was looked for but not found)
- Tracking research progress and next steps
- Recording correspondence with repositories, researchers, and relatives
- Managing research tasks and priorities
- Preserving the research narrative for future researchers

Current GEDCOM has no standard way to preserve this meta-research data.

## Core Concepts

### 1. Research Log (_RLOG)
Documenting research activities and findings.

```gedcom
0 @R1@ _RLOG
1 DATE 15 JAN 2025
1 _REPO National Archives
1 _PURP Looking for John Smith in 1850 census
1 _FIND Nothing found in expected location
1 NOTE Checked all Smiths in Boston ward 3
2 CONT No John aged 35-45 found
1 SOUR @S1@
2 PAGE Microfilm 1234, frames 100-150
1 _NEXT Check neighboring wards
```

### 2. Research Task (_TASK)
Tracking planned research activities.

```gedcom
0 @T1@ _TASK
1 _STAT pending
1 _PRIO high
1 TITL Find marriage record for John Smith
1 NOTE Check Massachusetts vital records 1845-1855
1 _ASSG Greg Lamberson
1 _DUE 1 FEB 2025
1 _LINK @I1@
```

### 3. Correspondence (_CORR)
Recording research-related communications.

```gedcom
0 @C1@ _CORR
1 DATE 10 JAN 2025
1 _TYPE email
1 _FROM librarian@archives.org
1 _TO researcher@example.com
1 _SUBJ RE: Smith family records
1 NOTE They have tax records 1840-1860
2 CONT Will send copies next week
1 _LINK @I1@
1 _LINK @R1@
```

### 4. Research Plan (_PLAN)
Documenting research strategies and objectives.

```gedcom
0 @P1@ _PLAN
1 TITL Smith Family Origins Research
1 _GOAL Identify parents of John Smith b.1815
1 NOTE Strategy:
2 CONT 1. Check birth records 1810-1820
2 CONT 2. Search census for family groups
2 CONT 3. Look for probate of potential father
1 _TASK @T1@
1 _TASK @T2@
```

## Integration Points

### Links to Standard Records
Research records can link to:
- Individuals (INDI)
- Families (FAM)
- Sources (SOUR)
- Repositories (REPO)

### Links to Extensions
Works with:
- gedcom-evidence: Research logs can reference evidence
- gedcom-occurrences: Tasks can relate to event research
- gedcom-citations: Logs reference sources with templates

## Use Cases

### 1. Professional Research Documentation
```gedcom
0 @R1@ _RLOG
1 DATE 20 JAN 2025
1 _REPO FamilySearch Library
1 _PURP Verify death date of Mary Jones
1 _FIND Found death certificate
1 _EVID @E1@              # Links to evidence record
1 SOUR @S1@
2 PAGE Certificate #1234
1 _TIME 2.5 hours
1 _COST $15 copy fee
```

### 2. Collaborative Research
```gedcom
0 @T1@ _TASK
1 _STAT assigned
1 _ASSG John Researcher
1 _PRIO medium
1 TITL Review German church records
1 NOTE Need German translation
1 _LANG de
1 _SKIL Language skills required
```

### 3. Research Timeline
```gedcom
0 @I1@ INDI
1 NAME John /Smith/
1 _RLOG @R1@              # Research activities
1 _RLOG @R2@              # Listed chronologically
1 _TASK @T1@              # Outstanding tasks
```

## Data Model Details

### Research Log (_RLOG)
- `DATE`: When research was conducted
- `_REPO`: Repository visited/contacted
- `_PURP`: Purpose/objective of research
- `_FIND`: Summary of findings
- `_TIME`: Time spent (optional)
- `_COST`: Expenses incurred (optional)
- `NOTE`: Detailed notes
- `SOUR`: Sources consulted
- `_NEXT`: Next steps identified
- `_LINK`: Links to related records

### Task (_TASK)
- `_STAT`: Status (pending|active|completed|cancelled)
- `_PRIO`: Priority (low|medium|high)
- `TITL`: Task title
- `NOTE`: Task details
- `_ASSG`: Assigned to
- `_DUE`: Due date
- `_COMP`: Completion date
- `_LINK`: Related records

### Correspondence (_CORR)
- `DATE`: Date of correspondence
- `_TYPE`: Type (email|letter|phone|meeting)
- `_FROM`: Sender
- `_TO`: Recipient
- `_SUBJ`: Subject
- `NOTE`: Content/summary
- `_ATTA`: Attachments (filenames)
- `_LINK`: Related records

### Research Plan (_PLAN)
- `TITL`: Plan title
- `_GOAL`: Research objectives
- `NOTE`: Strategy details
- `_TASK`: Associated tasks
- `_STAT`: Status
- `DATE`: Creation/update date

## Benefits

1. **Preserves Research Investment**: Documents hours of work even when nothing found
2. **Enables Collaboration**: Share tasks and coordinate research
3. **Prevents Duplication**: Shows what's already been searched
4. **Professional Standards**: Meets GPS documentation requirements
5. **Future Researchers**: Provides roadmap for continuing research

## Implementation Considerations

### Privacy
- Research logs may contain sensitive information
- Consider `_PRIV` flags for restricted access
- Correspondence may need redaction

### Storage
- Can generate significant data volume
- Consider summary vs. detailed modes
- Link to external research logs

### User Interface
- Chronological view of research activities
- Task management features
- Research timeline visualization

## Example: Complete Research Record

```gedcom
0 HEAD
1 GEDC
2 VERS 7.0
1 SCHMA
2 TAG _RLOG https://github.com/glamberson/gedcom-research-process
2 TAG _TASK https://github.com/glamberson/gedcom-research-process
2 TAG _CORR https://github.com/glamberson/gedcom-research-process
2 TAG _PLAN https://github.com/glamberson/gedcom-research-process

0 @I1@ INDI
1 NAME John /Smith/
1 _PLAN @P1@              # Research plan for this person

0 @P1@ _PLAN
1 TITL Find parents of John Smith
1 _GOAL Identify Smith family origins
1 _TASK @T1@
1 _TASK @T2@

0 @T1@ _TASK
1 _STAT completed
1 TITL Search 1850 census
1 _COMP 15 JAN 2025
1 _RLOG @R1@              # Task resulted in this log

0 @R1@ _RLOG
1 DATE 15 JAN 2025
1 _REPO National Archives
1 _PURP Find John Smith family in 1850 census
1 _FIND Located in Boston Ward 5
1 _EVID @E1@              # Created evidence record
1 _NEXT Check earlier census records

0 @T2@ _TASK
1 _STAT pending
1 TITL Check 1840 census
1 _PRIO high
1 NOTE Based on 1850 findings
```

## Compatibility Strategy

### Backward Compatibility
- All research elements are extensions
- Standard GEDCOM readers ignore them
- Core genealogical data unaffected

### Export Options
1. **Full Export**: Include all research data
2. **Summary Export**: Include plans and key findings only
3. **Clean Export**: Exclude research process data

## Future Extensions

1. **Research Groups**: Coordinate multiple researchers
2. **Research Budgets**: Track expenses and funding
3. **Research Metrics**: Time spent, success rates
4. **AI Integration**: Suggested next steps, pattern analysis

## Conclusion

The gedcom-research-process extension fills a critical gap in GEDCOM by preserving the research process alongside research results. This enables professional-quality documentation and collaborative research while maintaining full backward compatibility.