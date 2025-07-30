# Core Data Model vs. Application Features Separation
**Date**: July 29, 2025, 16:15 EEST  
**Purpose**: Clarify what belongs in the ResearchProcess-GPS core data model versus application/service offerings

## Architectural Principle: Clean Separation of Concerns

The ResearchProcess-GPS core should be a **pure research methodology and data preservation system**. Business features, while essential for professional genealogists, should exist as **application layer services** that interact with but don't pollute the core data model.

## Core Data Model Scope

### What BELONGS in the Core Data Model

#### 1. Research Methodology Primitives
```yaml
Core_Research_Entities:
  - Research questions and objectives
  - Hypotheses and theories
  - Evidence and analysis
  - Sources and citations
  - Conclusions and reasoning
  - Confidence levels and uncertainty
  
Research_Process_Tracking:
  - Research activities and logs
  - Search strategies and results
  - Decision points and reasoning
  - Negative findings documentation
  - Methodology used (GPS compliance)
  - Time sequences of discoveries
  
Research_Project_Management:
  - Research goals and objectives
  - Research task dependencies
  - Evidence requirements tracking
  - Hypothesis testing workflows
  - Research milestone definitions
  - GPS element completion tracking
  - Research team assignments
  - Peer review workflows
  - Theory development stages
  - Evidence gathering progress
```

#### Important Distinction: Research vs Business Project Management
```yaml
Core_Research_PM_Features:
  Task_Management:
    - "Search for birth certificate" ✓
    - "Test hypothesis about parents" ✓
    - "Resolve conflicting evidence" ✓
    - "Complete GPS analysis" ✓
    
  Progress_Tracking:
    - Evidence gathering completion ✓
    - Hypothesis confidence levels ✓
    - Research coverage metrics ✓
    - GPS compliance status ✓
    
  Workflow_Management:
    - Research methodology steps ✓
    - Evidence analysis stages ✓
    - Peer review processes ✓
    - Theory validation workflows ✓

Business_PM_Features_NOT_in_Core:
  Resource_Management:
    - Staff hour allocation ✗
    - Budget tracking ✗
    - Cost calculations ✗
    
  Business_Operations:
    - Client deliverable deadlines ✗
    - Invoice milestones ✗
    - Profitability metrics ✗
```

#### 2. Identity and Relationship Management
```yaml
Genealogical_Entities:
  - Persons (resolved individuals)
  - Identities (uncertain fragments)
  - Personas (source-specific mentions)
  - Flexible relationships (any type)
  - Places and geographic data
  - Events and temporal data
  - DNA evidence and analysis
```

#### 3. Collaboration and Attribution
```yaml
Research_Collaboration:
  - Researcher identification
  - Contribution tracking
  - Change attribution
  - Review and validation records
  - Comments and annotations
  - Collaborative workflows
  - Consensus tracking
```

#### 4. Data Integrity and Preservation
```yaml
Preservation_Features:
  - Version control for all entities
  - Immutable audit trails
  - Digital signatures
  - Timestamp verification
  - Evidence chains
  - Provenance tracking
  - Long-term format stability
```

#### 5. Standards Compliance Metadata
```yaml
Standards_Tracking:
  - Which standards apply
  - Compliance status
  - Validation results
  - Required fields tracking
  - Quality metrics
  - Completeness scores
```

### What STAYS OUT of the Core Data Model

#### 1. Financial/Business Operations ❌
```yaml
Not_In_Core:
  - Client billing information
  - Time tracking for invoicing
  - Project cost calculations
  - Payment processing
  - Business expenses
  - Tax categories
  
Why: These are business operations, not research data
```

#### 2. Client Relationship Management ❌
```yaml
Not_In_Core:
  - Client contact details (beyond researcher ID)
  - Contract terms
  - Communication preferences
  - Marketing data
  - Sales pipeline
  
Why: CRM data doesn't belong in research archives
```

#### 3. Scheduling and Calendar ❌
```yaml
Not_In_Core:
  - Appointment scheduling
  - Task deadlines (beyond research planning)
  - Meeting calendars
  - Personal schedules
  
Why: Operational data, not research methodology
```

#### 4. Educational/Training Features ❌
```yaml
Not_In_Core:
  - Course management
  - Student grades
  - Curriculum tracking
  - Certification progress
  
Why: Educational operations, not research data
```

## Marketing and Social Media Enablement

### Core Provides Marketing-Ready Outputs
The core data model should **enable** marketing without **becoming** a marketing platform:

```yaml
Marketing_Enablement_in_Core:
  Shareable_Outputs:
    - Research milestone achievements (GPS compliant!)
    - Anonymized success stories
    - Progress visualizations
    - Professional certification badges
    - Research quality metrics
    
  Privacy-Safe_Exports:
    - Redacted case studies
    - Statistical accomplishments
    - Research methodology demonstrations
    - Before/after research transformations
    
  Professional_Credibility:
    - Standards compliance certificates
    - Peer review validations
    - Research thoroughness metrics
    - Time-stamped discoveries

What_Core_Provides:
  - "Solved 150-year-old mystery" ✓
  - "GPS-compliant research completed" ✓
  - "Breakthrough discovery made" ✓
  - "Research peer-reviewed and validated" ✓
  
What_Core_Does_NOT_Do:
  - Post to social media ✗
  - Manage marketing campaigns ✗
  - Track engagement metrics ✗
  - Handle email marketing ✗
```

### Example: Marketing-Ready Outputs
```yaml
Research_Achievement_Export:
  core_generates:
    - Achievement type: "Mystery Solved"
    - Research duration: "6 months"
    - Sources consulted: "47"
    - GPS compliance: "100%"
    - Privacy-safe summary: "Identified unknown ancestor through innovative DNA analysis"
    
  marketing_app_uses:
    - Creates social media posts
    - Generates blog content
    - Updates professional website
    - Sends client newsletters
```

## Application Layer Services

### How Application Features Connect to Core

#### 1. Reference Model
```yaml
Application_Layer_Integration:
  Core_Provides:
    - Research project IDs
    - Researcher identities
    - Activity timestamps
    - Research status/progress
    - Quality metrics
    
  Application_Layer_Adds:
    - Business context (billing, contracts)
    - Operational data (schedules, tasks)
    - External integrations (CRM, accounting)
    - UI/UX preferences
    - Non-research communications
```

#### 2. Example: Professional Billing Service
```yaml
Billing_Service:
  Links_to_Core:
    - References research_project_id
    - Queries activity timestamps
    - Checks project completion status
    
  Maintains_Separately:
    - Client billing information
    - Hourly rates and contracts
    - Invoice generation
    - Payment tracking
    - Financial reports
    
  Never_Pollutes_Core:
    - No billing data in .rgps archives
    - No financial fields in research entities
    - Clean separation maintained
```

#### 3. Example: Project Management Service
```yaml
Project_Management:
  Links_to_Core:
    - Monitors research progress
    - Tracks milestone completion
    - Aggregates quality metrics
    
  Maintains_Separately:
    - Gantt charts and timelines
    - Resource allocation
    - Budget tracking
    - Risk management
    - Stakeholder communications
```

## Benefits of This Separation

### 1. Clean, Focused Data Model
- Research data remains pure and uncluttered
- Easier to maintain and evolve
- Clear purpose and boundaries
- Better long-term preservation

### 2. Flexible Business Features
- Can swap billing systems without touching research data
- Different service providers can offer competing solutions
- Business features can evolve independently
- Regional variations (tax, legal) handled outside core

### 3. Privacy and Security
- Research data separate from business data
- Different access controls possible
- Client financial data not mixed with research
- Cleaner compliance boundaries

### 4. Interoperability
- Other systems can use the research data model
- Not forced to adopt business model
- Clean API boundaries
- Standard research format, custom business layer

## Implementation Strategy

### Core Platform Responsibilities
1. **Provide clean APIs** for applications to query research data
2. **Generate events** that applications can subscribe to
3. **Maintain referential integrity** for research data only
4. **Export research metrics** that applications can use

### Application Layer Responsibilities
1. **Never write business data** to research archives
2. **Maintain own databases** for operational data
3. **Link via references** not embedded data
4. **Handle own compliance** requirements

### Example API Boundaries
```yaml
Core_Research_API:
  # What core provides to applications
  GET /projects/{id}/status
  GET /projects/{id}/activities
  GET /projects/{id}/researchers
  GET /projects/{id}/completion-metrics
  
Application_Services:
  # Business services maintain separately
  Billing_Service:
    POST /invoices
    GET /clients/{id}/billing
    
  Project_Management:
    POST /tasks
    GET /projects/{id}/timeline
    
  CRM_Service:
    POST /clients
    GET /communications/{id}
```

## Key Architectural Decision

**The ResearchProcess-GPS core data model is a research methodology and preservation system, NOT a business operations platform.**

Features like billing, CRM, project management, and scheduling are **essential for professional genealogists** but should be implemented as **application services** that reference and integrate with the core research data without polluting it.

This separation ensures the research data model remains:
- **Focused** on its core purpose
- **Clean** and maintainable
- **Portable** across different business contexts
- **Timeless** without business logic dependencies

---
*Architectural guidance for maintaining clean separation between research data and business operations*