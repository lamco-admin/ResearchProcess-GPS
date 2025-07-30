# ResearchProcess-GPS Conceptual Model Diagram

## Core Flow Diagram

```mermaid
graph TD
    RQ[Research Question] --> EG[Evidence Gathering]
    EG --> A[Analysis]
    A --> WH[Working Hypothesis]
    WH --> TV[Testing & Validation]
    TV --> C[Conclusion]
    
    E[Evidence] --> A
    E --> |supports/contradicts| WH
    
    C --> |maps to| TG[Traditional Genealogy]
    TG --> P[Persons]
    TG --> EV[Events]
    TG --> R[Relationships]
    TG --> L[Locations]
```

## Entity Relationship Diagram

```mermaid
erDiagram
    ResearchQuestion ||--o{ WorkingHypothesis : contains
    ResearchQuestion ||--o{ Analysis : drives
    ResearchQuestion ||--o{ Evidence : examines
    ResearchQuestion ||--|| Conclusion : "evolves to"
    
    Evidence ||--o{ ExtractedFact : contains
    Evidence ||--o{ Analysis : "analyzed in"
    
    Analysis ||--o{ AnalyticalPoint : contains
    Analysis ||--o{ CorrelationSet : identifies
    Analysis ||--o{ ConflictResolution : resolves
    
    WorkingHypothesis ||--o{ Evidence : "supported by"
    WorkingHypothesis ||--o{ Identity : interprets
    
    Identity ||--o{ Persona : "has in hypothesis"
    Identity ||--o{ Event : participates
    Identity ||--o{ Relationship : "involved in"
    
    Event ||--o{ EventParticipation : has
    Event ||--|| Location : "occurs at"
    
    Relationship ||--o{ RelationshipParticipant : includes
    
    Conclusion ||--o{ Identity : "establishes"
    Conclusion ||--o{ Event : confirms
    Conclusion ||--o{ Relationship : validates
```

## Abstraction Layer Architecture

```mermaid
graph TB
    subgraph "Domain Models"
        I[Identity]
        E[Event]
        R[Relationship]
        L[Location]
    end
    
    subgraph "Abstraction Layers"
        NS[Naming Systems]
        TS[Temporal Systems]
        SS[Spatial Systems]
        KS[Kinship Systems]
    end
    
    subgraph "Implementations"
        WN[Western Naming]
        PN[Patronymic]
        IN[Islamic Naming]
        CN[Chinese Naming]
        
        GC[Gregorian Calendar]
        JC[Julian Calendar]
        HC[Hebrew Calendar]
        IC[Islamic Calendar]
        
        GPS[GPS/WGS84]
        ADDR[Street Address]
        PLSS[US Land Survey]
        TRAD[Traditional]
    end
    
    I --> NS
    E --> TS
    L --> SS
    R --> KS
    
    NS --> WN
    NS --> PN
    NS --> IN
    NS --> CN
    
    TS --> GC
    TS --> JC
    TS --> HC
    TS --> IC
    
    SS --> GPS
    SS --> ADDR
    SS --> PLSS
    SS --> TRAD
```

## Data Flow Through System

```mermaid
sequenceDiagram
    participant User
    participant RQ as Research Question
    participant Ev as Evidence
    participant An as Analysis
    participant WH as Working Hypothesis
    participant Val as Validation
    participant Con as Conclusion
    participant Trad as Traditional Format
    
    User->>RQ: Creates question
    User->>Ev: Gathers evidence
    Ev->>An: Provides data
    An->>WH: Generates hypothesis
    WH->>Val: Tests predictions
    Val->>Con: Validates hypothesis
    Con->>Trad: Maps to GEDCOM/etc
    
    Note over WH,Val: Multiple hypotheses can be tested
    Note over Con,Trad: Conclusions map to traditional genealogy
```

## State Transitions

```mermaid
stateDiagram-v2
    [*] --> Planning: Create Research Question
    Planning --> Active: Start Research
    Active --> Analyzing: Evidence Gathered
    Analyzing --> Testing: Hypothesis Formed
    Testing --> Review: Results Available
    Review --> Concluded: Approved
    Review --> Testing: Needs More Work
    Testing --> Blocked: Insufficient Evidence
    Blocked --> Active: New Evidence Found
    Concluded --> [*]
    
    note right of Testing: Multiple hypotheses tested in parallel
    note right of Concluded: Maps to traditional genealogy
```

## Confidence Flow

```mermaid
graph LR
    E1[Evidence 1<br/>Confidence: 0.8] --> A[Analysis]
    E2[Evidence 2<br/>Confidence: 0.9] --> A
    E3[Evidence 3<br/>Confidence: 0.6] --> A
    
    A --> WH[Working Hypothesis<br/>Confidence: 0.75]
    
    WH --> GPS{GPS Compliance?}
    GPS -->|Yes| C1[Conclusion<br/>High Confidence]
    GPS -->|No| C2[Conclusion<br/>Moderate Confidence]
```

## Cultural Abstraction Example

```mermaid
graph TD
    Name[Name: محمد بن أحمد الصنعاني]
    
    Name --> NS[Naming System Analyzer]
    NS --> Islamic[Islamic Naming System]
    
    Islamic --> Ism[Ism: محمد<br/>Muhammad]
    Islamic --> Nasab[Nasab: بن أحمد<br/>ibn Ahmad]
    Islamic --> Nisba[Nisba: الصنعاني<br/>al-Sanani]
    
    Ism --> Given[Given Name]
    Nasab --> Patronym[Father: أحمد/Ahmad]
    Nisba --> Origin[From: صنعاء/Sana'a]
```

## Key Insights from Diagrams

1. **Research Question drives everything** - Not conclusions
2. **Evidence floats** - Can support multiple hypotheses
3. **Analysis is central** - The work of interpretation
4. **Conclusions map backwards** - To traditional genealogy
5. **Abstractions enable flexibility** - Cultural systems plug in
6. **Confidence propagates** - Through the analysis chain
7. **States are clear** - Research process is trackable