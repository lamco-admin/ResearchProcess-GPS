# Research Certification and Negative Findings Specification

## Overview

This specification defines how ResearchProcess-GPS handles research certification, immutable attribution, and the critical capability of publishing negative research findings. These features ensure research integrity, prevent duplicate efforts, and give proper credit for all types of genealogical work.

## Research Certification System

### 1. Certified Research Packages
```yaml
CertifiedPackage:
  # Immutable Core
  package_header:
    package_id: UUID
    package_type: enum
      # "positive_finding", "negative_finding", "analysis", 
      # "methodology", "collection", "synthesis"
    created_timestamp: ISO-8601
    certification_timestamp: ISO-8601
    
  # Researcher Identity
  researcher:
    researcher_id: UUID
    name: string
    email: string  # Hashed if private
    orcid: string  # Optional
    credentials: string[]  # "CG", "AG", etc.
    affiliation: string
    
  # Certification Details
  certification:
    method: enum
      # "self_certified", "peer_reviewed", "institution_verified",
      # "blockchain_anchored", "timestamp_authority"
    signature:
      algorithm: string  # "RSA-4096", "Ed25519"
      public_key: string
      signature: base64
    verification:
      hash: SHA3-512
      merkle_root: string  # If using blockchain
      timestamp_proof: URL
      
  # Package Metadata
  metadata:
    title: string
    abstract: text
    keywords: string[]
    gps_compliant: boolean
    ethics_review: boolean
    privacy_certified: boolean
```

### 2. Package Contents Structure
```yaml
PackageContents:
  # Research Documentation
  research_core:
    research_question: text
    hypothesis: text
    methodology: text
    
  # Evidence Chain
  evidence:
    - evidence_id: UUID
      evidence_type: string
      source_citation: text
      extracted_information: text
      quality_assessment: JSON
      images: file_reference[]
      
  # Analysis
  analysis:
    correlation_method: text
    conflicts_identified: conflict[]
    resolution_reasoning: text
    statistical_analysis: JSON
    
  # Conclusions
  conclusions:
    primary_conclusion: text
    confidence_level: percent
    alternative_explanations: text[]
    limitations: text[]
    future_research: text[]
    
  # Supporting Files
  attachments:
    - file_id: UUID
      file_type: string
      file_hash: SHA-256
      description: string
      access_level: enum  # "public", "restricted", "private"
```

### 3. Version Control for Certified Research
```yaml
ResearchVersioning:
  # Original Package (Immutable)
  original:
    version: "1.0"
    immutable: true
    withdrawn: false  # Can be marked but not deleted
    
  # Amendments
  amendments:
    - amendment_id: UUID
      original_package: package_id
      version: string  # "1.1", "1.2"
      change_type: enum
        # "correction", "addition", "clarification", "retraction"
      changes:
        - field_path: string
          old_value: any
          new_value: any
          rationale: text
      certification: Certification
      
  # Relationships
  relationships:
    supersedes: package_id[]
    superseded_by: package_id
    references: package_id[]
    disputes: package_id[]
    supports: package_id[]
```

## Negative Research Findings

### 1. Negative Finding Types
```yaml
NegativeFinding:
  finding_types:
    exhaustive_search_negative:
      description: "Searched all available records, person not found"
      components:
        - repositories_searched: repository[]
        - record_types_checked: string[]
        - date_ranges: period[]
        - geographic_scope: place[]
        - search_strategies: text[]
        
    disproven_identity:
      description: "Two people thought to be same are different"
      components:
        - identity_1: identity_reference
        - identity_2: identity_reference
        - distinguishing_evidence: evidence[]
        - impossibility_proof: text
        
    disproven_relationship:
      description: "Claimed relationship is impossible/incorrect"
      components:
        - person_1: person_reference
        - person_2: person_reference
        - claimed_relationship: string
        - disproving_evidence: evidence[]
        - actual_relationship: string  # If known
        
    impossible_theory:
      description: "Theory/hypothesis proven impossible"
      components:
        - theory_statement: text
        - impossibility_type: enum
          # "temporal", "geographic", "biological", "documentary"
        - proof_elements: evidence[]
        - mathematical_proof: text  # If applicable
```

### 2. Negative Finding Documentation
```yaml
NegativeDocumentation:
  required_elements:
    - Clear problem statement
    - Comprehensive search strategy
    - All repositories/sources consulted
    - Exact search parameters used
    - Why expected to find something
    - What would have been found
    - Significance of absence
    
  quality_standards:
    search_thoroughness:
      - All reasonable sources checked
      - Multiple search strategies used
      - Variant spellings attempted
      - Extended date ranges tried
      - Adjacent locations searched
      
    documentation_completeness:
      - Every search documented
      - Negative results explicit
      - Time periods covered
      - Access limitations noted
      
  peer_review:
    - Methodology sound
    - Search comprehensive
    - Conclusions justified
    - Alternative explanations considered
```

### 3. Negative Finding Publication
```yaml
NegativePublication:
  # Publication Record
  publication:
    finding_id: UUID
    publication_date: timestamp
    visibility: enum  # "public", "community", "private"
    
  # Searchability
  indexing:
    - Person names involved
    - Places searched
    - Time periods
    - Repository coverage
    - Theory keywords
    
  # Academic Credit
  citation:
    format: "Researcher Name, 'Negative Finding: [Title]', 
            ResearchProcess-GPS, Package ID, Date"
    doi_option: boolean
    academic_credit: boolean
    
  # Alerts System
  notifications:
    - Alert researchers working on same problem
    - Prevent duplicate negative searches
    - Update related positive findings
    - Notify theory subscribers
```

## Attribution and Citation System

### 1. Attribution Requirements
```yaml
AttributionSystem:
  # Mandatory Attribution
  required_elements:
    - Original researcher name
    - Package ID (with version)
    - Creation date
    - Certification status
    - Access date
    
  # Citation Formats
  formats:
    inline: "Smith (2025, PKG-12345)"
    
    footnote: "Jane Smith, 'Research on John Doe Identity', 
               ResearchProcess-GPS Package PKG-12345, 
               certified 29 Jul 2025, accessed 30 Jul 2025."
               
    bibliography: "Smith, Jane. 2025. 'Research on John Doe Identity.'
                   ResearchProcess-GPS Certified Package PKG-12345.
                   Certified July 29, 2025. DOI: 10.12345/pkg-12345."
                   
  # Machine-Readable
  structured_citation:
    - RIS format
    - BibTeX format
    - JSON-LD
    - Schema.org markup
```

### 2. Usage Rights Management
```yaml
UsageRights:
  # License Types
  licenses:
    read_only:
      - Can view
      - Can cite
      - Cannot modify
      - Cannot redistribute
      
    attribution_share:
      - Can build upon
      - Must attribute
      - Must share alike
      - Can redistribute
      
    commercial_allowed:
      - All above rights
      - Commercial use OK
      - Derivative works OK
      
    custom:
      - Define specific rights
      - Time-limited options
      - Geographic restrictions
      - Purpose restrictions
      
  # Enforcement
  enforcement:
    technical:
      - Watermarking
      - Access controls
      - Usage tracking
      - Expiration dates
      
    legal:
      - Terms of service
      - DMCA provisions
      - License agreement
      - Dispute process
```

### 3. Modification Tracking
```yaml
ModificationTracking:
  # By Original Researcher
  self_modifications:
    - Create amendments
    - Issue corrections
    - Add clarifications
    - Publish retractions
    - All preserve original
    
  # By Others
  derivative_works:
    - Must fork package
    - Clear attribution
    - Link to original
    - Cannot alter original
    - Track divergence
    
  # Notification System
  change_notifications:
    subscribers:
      - Get alerts on changes
      - See amendment details
      - Review retractions
      - Track derivatives
      
    impact_analysis:
      - Which conclusions affected
      - Which researchers impacted
      - Cascade notifications
      - Update recommendations
```

## Integration with Main Platform

### 1. Research Workflow Integration
```yaml
WorkflowIntegration:
  # During Research
  pre_certification:
    - Draft packages
    - Peer review option
    - Quality checking
    - GPS compliance verification
    
  # Certification Process
  certification_workflow:
    - Completeness check
    - Privacy review
    - Sign package
    - Generate certificates
    - Publish/archive
    
  # Post-Certification
  maintenance:
    - Monitor citations
    - Track usage
    - Handle disputes
    - Process amendments
```

### 2. Discovery and Search
```yaml
DiscoverySystem:
  # Finding Negative Research
  negative_search:
    - "Has anyone disproven X?"
    - "Failed searches for Y in Z location"
    - "Impossible theories about A"
    
  # Finding Certified Research
  certified_search:
    - By researcher
    - By topic/person
    - By date range
    - By GPS compliance
    - By peer review status
    
  # Impact Metrics
  metrics:
    - Citation count
    - Prevented duplications
    - Theory invalidations
    - Research time saved
```

### 3. GEDCOM 7 Export
```yaml
GEDCOM7Export:
  # Certified Person Record
  0 @I1@ INDI
  1 NAME John /Smith/
  1 _CERTIFIED_RESEARCH @CR1@
  2 _PACKAGE 550e8400-e29b-41d4-a716-446655440000
  2 _VERSION 1.0
  2 _RESEARCHER "Jane Doe"
  2 _DATE 29 JAN 2025
  2 _TYPE positive_finding
  2 _SIGNATURE base64signature==
  2 _RIGHTS read_only
  
  # Negative Finding Record
  0 @NF1@ _NEGATIVE_FINDING
  1 _TYPE exhaustive_search
  1 _PERSON "John Smith b. 1850"
  1 _SEARCHED "Ohio births 1845-1855"
  1 _RESULT "Not found"
  1 _PACKAGE 660e9500-f39c-51e5-b827-557766551111
  1 _RESEARCHER "Jane Doe"
  1 _CERTIFIED 29 JAN 2025
  
  # Disproven Theory
  0 @DT1@ _DISPROVEN
  1 _THEORY "John Smith son of William Smith"
  1 _DISPROOF temporal_impossibility
  1 _EVIDENCE @E1@ @E2@
  1 _PACKAGE 770fa611-g4ad-62f6-c938-668877662222
  1 _CERTIFIED 29 JAN 2025
```

## Benefits and Impact

### 1. For Individual Researchers
- Protect intellectual property
- Get credit for all work (including negative)
- Prevent others from duplicating effort
- Build professional reputation

### 2. For the Community
- Reduce duplicate research
- Share negative findings
- Build on certified work
- Maintain quality standards

### 3. For Professional Genealogy
- Meet client confidentiality needs
- Provide certified deliverables
- Document due diligence
- Support peer review

This certification and negative findings system transforms genealogical research by giving equal weight to what we don't find as to what we do find, while ensuring proper attribution and preventing wasteful duplication of effort.