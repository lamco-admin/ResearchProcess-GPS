# DNA Models Specification for ResearchProcess-GPS

## Overview

This specification defines comprehensive DNA data models supporting multiple test types, analysis methods, and integration with genealogical research. The system accommodates current testing technologies while remaining extensible for future developments.

## Core DNA Test Types

### 1. Autosomal DNA (atDNA)
```yaml
AutosomalDNA:
  test_info:
    test_id: UUID
    person_id: UUID
    testing_company: string
    test_date: date
    chip_version: string
    total_snps_tested: integer
    genome_build: string  # "37", "38"
    raw_data_file: path
    
  matches:
    - match_id: UUID
      match_name: string
      shared_cm: float
      shared_segments: integer
      largest_segment: float
      relationship_range:
        - relationship: string
          probability: percent
      match_test_company: string
      match_person_id: UUID  # If identified
      match_identity_id: UUID  # If unidentified
      maternal_side: boolean
      paternal_side: boolean
      x_match: boolean
      
  segments:
    - segment_id: UUID
      chromosome: integer  # 1-22
      start_position: integer
      end_position: integer
      length_cm: float
      snp_count: integer
      matches_sharing: match_id[]
      triangulated: boolean
      ancestor_attribution: person_id
      confidence: percent
      
  ethnicity:
    company_estimates:
      - company: string
        version: string
        regions:
          - region_name: string
            percentage: float
            confidence_range: {low: float, high: float}
    third_party_analysis:
      - tool_name: string
        analysis_date: date
        results: JSON
```

### 2. Y-DNA
```yaml
YDNA:
  test_info:
    test_id: UUID
    person_id: UUID
    testing_company: string
    test_level: string  # "Y-37", "Y-67", "Y-111", "Big-Y"
    test_date: date
    
  haplogroup:
    confirmed_haplogroup: string
    terminal_snp: string
    haplogroup_origin: string
    migration_path: coordinate[]
    age_estimate: string
    confidence: percent
    
  str_markers:  # Short Tandem Repeats
    - marker_name: string  # "DYS393", etc.
      value: integer
      mutations_from_modal: integer
      multi_copy: boolean
      values: integer[]  # For multi-copy markers
      
  snp_results:  # For Big-Y and SNP packs
    - snp_name: string
      position: integer
      ancestral_value: string
      derived_value: string
      test_result: string  # "+", "-", "no-call"
      
  matches:
    - match_id: UUID
      match_name: string
      genetic_distance:
        at_12: integer
        at_25: integer
        at_37: integer
        at_67: integer
        at_111: integer
      shared_ancestors:
        - ancestor_id: UUID
          generations_back: integer
          confidence: percent
      haplogroup_match: string
      terminal_snp_match: string
      
  surname_project:
    project_name: string
    project_id: string
    subgroup: string
    modal_haplotype_distance: integer
```

### 3. Mitochondrial DNA (mtDNA)
```yaml
MtDNA:
  test_info:
    test_id: UUID
    person_id: UUID
    testing_company: string
    test_level: string  # "HVR1", "HVR2", "Full Sequence"
    test_date: date
    
  haplogroup:
    confirmed_haplogroup: string
    subclade: string
    origin_region: string
    age_estimate: string
    migration_path: coordinate[]
    
  mutations:
    hvr1_mutations: string[]  # e.g., "16519C"
    hvr2_mutations: string[]
    coding_region_mutations: string[]
    heteroplasmy:
      - position: integer
        variation: string
        percentage: float
        
  matches:
    - match_id: UUID
      match_name: string
      genetic_distance: integer
      match_level: string  # "HVR1", "HVR2", "Full"
      common_ancestor_estimate: string
      matching_mutations: string[]
```

### 4. X-DNA
```yaml
XDNA:
  test_info:
    test_id: UUID
    person_id: UUID
    extracted_from_autosomal: boolean
    
  segments:
    - segment_id: UUID
      start_position: integer
      end_position: integer
      length_cm: float
      matches_sharing: match_id[]
      
  inheritance_path:
    possible_paths:
      - path_id: UUID
        ancestors: person_id[]
        gender_sequence: string  # "F-M-F-F"
        probability: percent
```

## Advanced DNA Analysis Models

### 1. Triangulation Groups
```yaml
TriangulationGroup:
  group_id: UUID
  creation_method: string  # "manual", "auto-cluster", "shared-match"
  
  members:
    - person_id: UUID
      match_ids: UUID[]
      role: string  # "target", "known", "unknown"
      
  shared_segments:
    - chromosome: integer
      start_pos: integer
      end_pos: integer
      min_cm: float
      max_cm: float
      sharing_count: integer
      
  mrca_analysis:
    proposed_mrca: person_id
    confidence: percent
    supporting_evidence: text
    alternative_mrcas:
      - ancestor_id: UUID
        probability: percent
```

### 2. Chromosome Mapping
```yaml
ChromosomeMap:
  person_id: UUID
  
  mapped_segments:
    - segment_id: UUID
      chromosome: integer
      start_pos: integer
      end_pos: integer
      maternal_ancestor: person_id
      paternal_ancestor: person_id
      confidence: percent
      supporting_matches: match_id[]
      
  visual_browser:
    display_format: string  # "split", "combined", "maternal", "paternal"
    color_scheme: JSON
    annotation_layers: string[]
```

### 3. DNA Clustering
```yaml
DNACluster:
  cluster_method: string  # "Leeds", "AutoCluster", "Custom"
  parameters:
    min_cm: float
    max_cm: float
    min_shared_matches: integer
    
  clusters:
    - cluster_id: UUID
      cluster_color: string
      members: match_id[]
      shared_matches_matrix: JSON
      
  cluster_analysis:
    - cluster_id: UUID
      proposed_ancestors: person_id[]
      confidence: percent
      notes: text
```

### 4. Ethnicity Analysis
```yaml
EthnicityAnalysis:
  person_id: UUID
  
  chromosome_painting:
    - chromosome: integer
      segments:
        - start_pos: integer
          end_pos: integer
          population: string
          confidence: percent
          
  ancient_dna_matches:
    - ancient_sample_id: string
      shared_dna_pct: float
      time_period: string
      location: string
      culture: string
      
  population_analysis:
    admixture_events:
      - populations: string[]
        estimated_date: string
        confidence: percent
```

## DNA Evidence Integration

### 1. DNA Evidence Items
```yaml
DNAEvidence:
  evidence_id: UUID
  evidence_type: "DNA"
  
  test_references:
    - test_id: UUID
      test_type: string
      relevant_results: string[]
      
  hypothesis_support:
    hypothesis_id: UUID
    support_type: string  # "confirms", "refutes", "neutral"
    explanation: text
    statistical_significance: float
    
  relationship_proof:
    person1_id: UUID
    person2_id: UUID
    proposed_relationship: string
    dna_support:
      - test_type: string
        result: string
        probability: percent
```

### 2. DNA Conflict Resolution
```yaml
DNAConflict:
  conflict_id: UUID
  
  conflicting_evidence:
    - evidence_id: UUID
      claim: text
      
  possible_explanations:
    - explanation_type: string
      # "NPE", "adoption", "misattributed_parentage",
      # "pedigree_collapse", "endogamy", "false_match"
      description: text
      probability: percent
      supporting_evidence: evidence_id[]
```

## Privacy & Ethical Considerations

### 1. Privacy Controls
```yaml
DNAPrivacy:
  test_id: UUID
  
  consent_tracking:
    test_consent_given: boolean
    sharing_consent: boolean
    research_consent: boolean
    consent_date: date
    consent_version: string
    
  data_sharing:
    - platform: string
      shared: boolean
      visibility: string  # "public", "matches_only", "private"
      opt_in_research: boolean
      
  sensitive_findings:
    - finding_type: string
      # "unexpected_parent", "unexpected_ethnicity",
      # "health_related", "criminal_database_match"
      disclosed_to_person: boolean
      handling_notes: text
```

### 2. Living Person Protection
```yaml
DNALivingProtection:
  anonymization_rules:
    - hide_names: boolean
      use_initials: boolean
      hide_dates: boolean
      generalize_locations: boolean
      
  export_restrictions:
    - format: string
      allowed: boolean
      redaction_level: string
```

## Integration Features

### 1. Multi-Company Support
```yaml
CompanyIntegration:
  supported_companies:
    - company_name: string
      api_available: boolean
      import_formats: string[]
      sync_enabled: boolean
      
  data_normalization:
    cm_threshold_differences: JSON
    relationship_calculations: JSON
    ethnicity_region_mapping: JSON
```

### 2. Analysis Tool Integration
```yaml
ThirdPartyTools:
  supported_tools:
    - GEDmatch
    - DNAPainter
    - RootsFinder
    - MyHeritage
    - Jonny_Perl_tools
    - Genetic_Affairs
    
  import_export:
    - tool: string
      import_format: string
      export_format: string
      field_mapping: JSON
```

## Reporting & Visualization

### 1. DNA Reports
```yaml
DNAReports:
  report_types:
    - "Match List"
    - "In Common With"
    - "Chromosome Browser"
    - "Triangulation Report"
    - "Y-DNA Lineage"
    - "mtDNA Lineage"
    - "Ethnicity Comparison"
    - "DNA Coverage Analysis"
    
  visualization_options:
    - charts: ["pie", "bar", "timeline", "map"]
    - chromosome_browser: ["linear", "circular"]
    - relationship_trees: ["fan", "pedigree", "network"]
```

This comprehensive DNA model specification provides the foundation for integrating genetic genealogy into the ResearchProcess-GPS platform while maintaining privacy, supporting multiple test types, and enabling advanced analysis techniques.