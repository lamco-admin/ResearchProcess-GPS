# ResearchProcess-GPS Conceptual Model Overview

## Core Philosophy

ResearchProcess-GPS treats genealogical research as a **process**, not a conclusion. Everything in the system supports multiple interpretations, temporal awareness, and cultural flexibility.

## Entity Hierarchy

```
Universe (The entire research space)
├── Research Projects
│   ├── Theories (Alternative interpretations)
│   │   ├── Identities (People who might exist)
│   │   │   └── Personas (Identity in specific theory)
│   │   ├── Events (Things that happened)
│   │   ├── Relationships (Connections between identities)
│   │   └── Locations (Places with history)
│   │
│   ├── Evidence (Source material)
│   │   ├── Sources (Original documents)
│   │   ├── Information (What sources contain)
│   │   └── Facts (Extracted atomic data)
│   │
│   └── Analysis (Research work)
│       ├── Correlations (Connecting evidence)
│       ├── Conflicts (Contradictions)
│       ├── Resolutions (Decisions made)
│       └── Conclusions (Defensible positions)
│
└── Knowledge Systems (Cultural abstractions)
    ├── Naming Systems
    ├── Calendar Systems
    ├── Location Systems
    ├── Kinship Systems
    └── Social Systems
```

## First-Class Concepts

### 1. Theory
- Container for a specific interpretation of data
- Can branch, merge, compare
- Has confidence metrics and GPS compliance tracking
- Contains specific versions of all entities

### 2. Identity
- A potentially real person
- Exists across theories with different interpretations (Personas)
- Not locked to any specific facts until theory resolution

### 3. Evidence
- Source material that exists independently
- Can support or contradict multiple theories
- Contains extractable facts that float between theories
- Includes negative evidence (absence of expected records)

### 4. Analysis
- **This should be a first-class concept** (you're right!)
- The work of interpreting evidence
- Creates connections, identifies conflicts, proposes resolutions
- Tracks methodology and reasoning

### 5. Event
- Something that happened (or might have happened)
- Not owned by any person or family
- Multiple participants with roles
- Can be interpreted differently in different theories

### 6. Relationship
- Connections between identities
- Multi-party, not just two-person
- Culturally aware with temporal validity
- Can represent any human connection

### 7. Location
- Places that change over time
- Multiple names, jurisdictions, boundaries
- Supports multiple coordinate systems
- Temporal snapshots at any date

## Abstraction Layers

### Naming System Abstraction
```
NameSystem (Abstract)
├── WesternNaming
│   ├── Given + Middle + Surname
│   ├── Suffixes (Jr, III, etc.)
│   └── Titles (Mr, Dr, etc.)
├── PatronymicNaming
│   ├── Given + Patronymic
│   └── Chain of ancestry
├── IslamicNaming
│   ├── Ism (given)
│   ├── Nasab (lineage)
│   ├── Laqab (epithet)
│   ├── Nisba (origin)
│   └── Kunya (teknonym)
├── ChineseNaming
│   ├── Family + Generation + Given
│   └── Courtesy names
└── [Extensible for other systems]
```

### Calendar System Abstraction
```
CalendarSystem (Abstract)
├── GregorianCalendar
├── JulianCalendar
├── HebrewCalendar
├── IslamicCalendar
├── ChineseCalendar
├── PersianCalendar
├── FrenchRevolutionary
└── [Extensible]

DateExpression (Flexible dates)
├── ExactDate
├── DateRange
├── MultipleDates
├── ApproximateDate
├── CalculatedDate
├── NotDate (explicitly NOT this date)
├── RelativeDate (before X, after Y)
└── UnknownDate
```

### Location System Abstraction
```
LocationSystem (Abstract)
├── CoordinateSystem
│   ├── WGS84
│   ├── LocalGrid
│   └── HistoricalMap
├── AddressSystem
│   ├── StreetAddress
│   ├── PostalSystem
│   └── TraditionalDescription
├── JurisdictionSystem
│   ├── ModernAdministrative
│   ├── HistoricalAdministrative
│   ├── Ecclesiastical
│   └── Traditional/Tribal
└── [Extensible]
```

## Key Relationships

### Evidence → Analysis → Theory
1. Evidence provides raw material
2. Analysis interprets evidence, creating facts
3. Facts support or contradict theories
4. Theories contain coherent interpretations

### Identity ↔ Persona ↔ Theory
1. Identity exists abstractly
2. Personas are theory-specific interpretations
3. Same identity can have different life stories in different theories

### Time Pervades Everything
- Events have complex temporal expressions
- Locations change over time
- Relationships have temporal validity
- Even evidence has temporal context (when found, when created)

## What Makes This Revolutionary

1. **No Forced Conclusions**: Unlike traditional genealogy software
2. **Cultural Flexibility**: Not Western-centric
3. **Temporal Awareness**: Everything can change over time
4. **Theory Testing**: Scientific method for genealogy
5. **Evidence Floating**: Same evidence, multiple interpretations
6. **Process Tracking**: How you reached conclusions matters

## Missing Pieces to Build

1. **Analysis Engine**: Make analysis a first-class entity
2. **Cultural Abstractions**: Naming, calendar, location systems
3. **Confidence Propagation**: How confidence flows through theories
4. **Merge Strategies**: How to reconcile different theories
5. **Export Strategies**: How to flatten for traditional software