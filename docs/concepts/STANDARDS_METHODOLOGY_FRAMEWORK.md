# Standards-Based Methodology Framework

## Overview

ResearchProcess-GPS implements genealogical standards as **configurable, versioned, and auditable** components rather than hard-coded rules. This allows the platform to adapt to evolving standards while maintaining rigorous compliance tracking.

## Core Design Principle: Standards as Configuration

```yaml
StandardsAsConfiguration:
  principle: "Standards are data, not code"
  benefits:
    - Update standards without platform changes
    - Support multiple standards simultaneously  
    - Track compliance automatically
    - Version standard requirements
    - Audit against specific standard versions
```

## 1. Genealogical Proof Standard (GPS) Implementation

### GPS Configuration Schema

```yaml
GPS_Standard_v2025:
  standard_id: "GPS-2025"
  authority: "Board for Certification of Genealogists"
  version: "2025.1"
  effective_date: "2025-01-01"
  
  elements:
    reasonably_exhaustive_research:
      element_id: "GPS-1"
      description: "Reasonably exhaustive research"
      
      requirements:
        - req_id: "GPS-1.1"
          description: "Search all reasonably available sources"
          measurement_criteria:
            repository_coverage:
              - repository_type: "vital_records"
                time_period_relevance: boolean
                geographic_relevance: boolean
                completion_threshold: 0.90
              - repository_type: "census_records"
                decades_coverage: 0.80
              - repository_type: "probate_records"
                relevant_jurisdiction: boolean
                
        - req_id: "GPS-1.2"
          description: "Document negative searches"
          verification_method: "negative_search_log_exists"
          
        - req_id: "GPS-1.3"
          description: "Identify all possible record locations"
          checklist_items:
            - "Original jurisdiction records"
            - "Repository catalogs consulted"
            - "Finding aids reviewed"
            - "Related collections examined"
            
    complete_accurate_citations:
      element_id: "GPS-2"
      description: "Complete and accurate source citations"
      
      requirements:
        - req_id: "GPS-2.1"
          description: "Include all elements needed to locate source"
          citation_elements:
            mandatory: ["author", "title", "publication", "date", "location"]
            conditional: ["page", "url", "accessed_date", "repository"]
            
        - req_id: "GPS-2.2"
          description: "Distinguish original vs derivative sources"
          verification: "source_type_classification"
          
    thorough_analysis:
      element_id: "GPS-3"
      description: "Thorough analysis and correlation"
      
      requirements:
        - req_id: "GPS-3.1"
          description: "Analyze each source's relevance"
          evidence_quality_matrix:
            source_type: ["original", "derivative", "authored"]
            information_type: ["primary", "secondary", "indeterminate"]
            evidence_type: ["direct", "indirect", "negative"]
            
        - req_id: "GPS-3.2"
          description: "Correlate information from all sources"
          correlation_tracking:
            - "Identity correlation across sources"
            - "Timeline consistency verification"
            - "Geographic plausibility checks"
            - "Relationship logic validation"
            
    conflict_resolution:
      element_id: "GPS-4"
      description: "Resolution of conflicting evidence"
      
      requirements:
        - req_id: "GPS-4.1"
          description: "Identify all conflicts"
          conflict_types: ["date", "place", "identity", "relationship"]
          
        - req_id: "GPS-4.2"
          description: "Explain conflict resolution"
          resolution_components:
            - "Evidence weight assessment"
            - "Source reliability comparison"
            - "Information credibility analysis"
            - "Logical explanation of conclusion"
            
    sound_written_conclusion:
      element_id: "GPS-5"
      description: "Sound written conclusion"
      
      requirements:
        - req_id: "GPS-5.1"
          description: "Written proof argument or summary"
          formats: ["proof_argument", "proof_summary", "research_report"]
          
        - req_id: "GPS-5.2"
          description: "Conclusion supported by evidence"
          verification: "evidence_citation_coverage"
```

### GPS Compliance Engine

```yaml
ComplianceEngine:
  # Automatic Compliance Checking
  compliance_check:
    check_id: UUID
    standard: "GPS-2025"
    entity_checked: entity_id
    check_timestamp: timestamp
    
    element_results:
      GPS-1_exhaustive_research:
        status: enum ["compliant", "partial", "non_compliant"]
        score: percent
        missing_requirements:
          - req_id: "GPS-1.1"
            description: "Census decades 1870-1880 not searched"
            severity: "major"
            remediation: "Search census records for missing decades"
            
      GPS-2_citations:
        status: "compliant"
        score: 98
        issues:
          - citation_id: UUID
            issue: "Missing page number"
            severity: "minor"
            
      GPS-3_analysis:
        status: "partial"
        score: 75
        missing_analysis:
          - "Identity correlation incomplete"
          - "Timeline gaps not explained"
          
      GPS-4_conflicts:
        status: "compliant"
        score: 100
        conflicts_resolved: 3
        resolution_quality: "well_documented"
        
      GPS-5_conclusion:
        status: "partial"
        score: 80
        issues:
          - "Proof argument lacks evidence summary"
          - "Some conclusions unsupported by citations"
          
  # Remediation Tracking
  remediation_plan:
    plan_id: UUID
    compliance_check_id: UUID
    
    action_items:
      - action: "Search 1870-1880 census"
        requirement: "GPS-1.1"
        assigned_to: researcher_id
        due_date: date
        status: enum ["pending", "in_progress", "complete"]
        
      - action: "Add page numbers to 5 citations"
        requirement: "GPS-2.1"
        priority: "low"
        estimated_time: "30 minutes"
```

## 2. BCG Standards Implementation

### BCG Standards Configuration

```yaml
BCG_Standards_v3_4:
  standard_id: "BCG-3.4"
  authority: "Board for Certification of Genealogists"
  version: "3.4"
  
  competency_areas:
    research_planning:
      competencies:
        - comp_id: "BCG-RP-1"
          description: "Develops effective research plans"
          evaluation_criteria:
            - "Identifies research objectives"
            - "Prioritizes research tasks"
            - "Estimates time and resources"
            - "Documents research strategy"
            
    source_analysis:
      competencies:
        - comp_id: "BCG-SA-1"
          description: "Evaluates source reliability"
          measurement_method: "source_evaluation_rubric"
          
        - comp_id: "BCG-SA-2"
          description: "Identifies source limitations"
          checklist:
            - "Temporal coverage gaps"
            - "Geographic limitations"
            - "Record completeness"
            - "Preservation issues"
            
    evidence_correlation:
      competencies:
        - comp_id: "BCG-EC-1"
          description: "Correlates evidence accurately"
          verification_methods:
            - "Identity resolution accuracy"
            - "Timeline consistency"
            - "Relationship logic"
            
    writing_skills:
      competencies:
        - comp_id: "BCG-WS-1"
          description: "Writes clear proof arguments"
          quality_metrics:
            - clarity_score: algorithm
            - citation_completeness: percent
            - logical_flow: rubric
            - evidence_coverage: percent
```

### BCG Certification Support

```yaml
CertificationSupport:
  # Portfolio Building
  portfolio_assistant:
    portfolio_id: UUID
    candidate: researcher_id
    
    required_elements:
      research_report:
        status: "in_progress"
        gps_compliance: 85
        bcg_rubric_score: "pending"
        peer_reviews: 2
        
      case_study:
        status: "complete"
        problem_type: "complex_identity"
        sources_used: 47
        conflicts_resolved: 5
        
      document_transcription:
        status: "complete"
        document_type: "will"
        accuracy_score: 98
        paleography_demonstrated: true
        
      kinship_determination:
        status: "planning"
        relationship_type: "3rd_cousin"
        proof_standard: "GPS"
        
  # Skill Assessment
  skill_tracking:
    researcher_id: UUID
    
    competency_scores:
      BCG-RP-1: 85  # Research planning
      BCG-SA-1: 90  # Source analysis
      BCG-EC-1: 75  # Evidence correlation
      BCG-WS-1: 80  # Writing skills
      
    improvement_recommendations:
      - competency: "BCG-EC-1"
        suggestion: "Practice complex identity resolution"
        resources: ["webinar_link", "exercise_set"]
        
    certification_readiness:
      overall_score: 82
      ready_for_submission: false
      areas_needing_work: ["evidence_correlation", "conflict_resolution"]
```

## 3. Research Log Standards

### Configurable Research Log Schema

```yaml
ResearchLogStandard:
  standard_id: "NGS-RL-2024"
  authority: "National Genealogical Society"
  
  required_fields:
    session_info:
      - date: date
      - researcher: string
      - objective: text
      - client_matter: string  # Optional
      
    search_activity:
      - repository: string
      - collection: string
      - search_parameters: text
      - time_period_searched: date_range
      - names_searched: [string]
      
    results:
      - found: boolean
      - items_found: [item]
      - relevance: enum ["high", "medium", "low", "none"]
      - notes: text
      
    next_steps:
      - planned_searches: [search]
      - hypotheses_to_test: [hypothesis]
      - follow_up_required: boolean
      
  # Automatic Log Generation
  auto_logging:
    capture_events:
      - "search_performed"
      - "source_viewed"
      - "evidence_extracted"
      - "analysis_completed"
      
    enrichment:
      - add_timestamp: automatic
      - add_researcher: from_session
      - add_repository: from_source
      - add_objectives: from_project
      
    privacy_settings:
      - client_info: "redactable"
      - living_persons: "auto_redact"
      - sensitive_searches: "flag_for_review"
```

## 4. Academic Standards Integration

### Scholarly Citation Standards

```yaml
AcademicStandards:
  chicago_manual_17th:
    standard_id: "CMOS-17"
    
    citation_formats:
      archival_source:
        template: "${author}, ${title}, ${date}, ${series}, ${box}:${folder}, ${repository}, ${location}."
        
      published_work:
        template: "${author}. ${title}. ${place}: ${publisher}, ${year}."
        
      online_source:
        template: "${author}. \"${title}.\" ${website}. ${url} (accessed ${access_date})."
        
  digital_object_identifiers:
    doi_support:
      - generate_for_publications: true
      - link_to_evidence: true
      - version_tracking: true
      
    orcid_integration:
      - researcher_identification: true
      - contribution_tracking: true
      - publication_attribution: true
```

## 5. Compliance Dashboard

### Real-Time Compliance Monitoring

```yaml
ComplianceDashboard:
  # Project-Level Compliance
  project_compliance:
    project_id: UUID
    
    standards_tracked:
      - standard: "GPS-2025"
        overall_compliance: 87
        trend: "improving"
        
      - standard: "BCG-3.4"
        competency_coverage: 92
        certification_ready: false
        
    compliance_timeline:
      - date: "2024-01-15"
        gps_score: 72
        issues_count: 15
        
      - date: "2024-06-15"
        gps_score: 87
        issues_count: 5
        
  # Researcher Development
  researcher_dashboard:
    researcher_id: UUID
    
    skill_progression:
      source_analysis:
        current_level: "proficient"
        next_level: "expert"
        requirements_remaining: ["analyze_10_foreign_sources"]
        
      proof_writing:
        current_level: "intermediate"
        improvement_areas: ["conflict_explanation", "evidence_summary"]
        
    achievements:
      - "First GPS-compliant proof"
      - "100 sources analyzed"
      - "Complex conflict resolved"
      
    learning_recommendations:
      - course: "Advanced Evidence Analysis"
        reason: "Improve correlation skills"
        provider: "BCG Education"
```

## 6. Standards Evolution Management

### Version Control for Standards

```yaml
StandardsVersioning:
  # Track Standard Changes
  standard_evolution:
    standard: "GPS"
    
    version_history:
      - version: "2020.1"
        changes: ["Added digital source requirements"]
        migration_notes: "Update citation formats"
        
      - version: "2025.1"
        changes: ["Enhanced DNA evidence requirements"]
        backward_compatible: true
        
  # Migration Support
  standard_migration:
    from_version: "GPS-2020"
    to_version: "GPS-2025"
    
    migration_tasks:
      - task: "Review DNA evidence citations"
        affected_entities: 145
        automation_available: true
        
      - task: "Update digital source citations"
        affected_entities: 892
        manual_review_required: true
        
  # Multi-Standard Support
  concurrent_standards:
    entity_id: UUID
    
    compliance_tracking:
      - standard: "GPS-2025"
        score: 92
        
      - standard: "NGS-Guidelines-2024"
        score: 88
        
      - standard: "ICAPGen-2023"
        score: 95
```

## Implementation Benefits

1. **Adaptability**: Standards evolve; the platform evolves with them
2. **Transparency**: Researchers see exactly what's required
3. **Automation**: Compliance checking happens automatically
4. **Education**: Platform teaches standards through use
5. **Certification**: Direct support for professional certification
6. **Multi-Standard**: Support different standards for different purposes

This framework ensures ResearchProcess-GPS maintains the highest professional standards while remaining flexible enough to adapt to the evolving field of genealogy.