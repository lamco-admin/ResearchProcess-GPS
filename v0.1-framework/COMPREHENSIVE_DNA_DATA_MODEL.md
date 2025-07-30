# Comprehensive DNA Data Model for ResearchProcess-GPS

## Overview

DNA evidence is now fundamental to genealogical research. This model supports all types of DNA testing while maintaining privacy and enabling advanced analysis.

## Core DNA Entity Types

### 1. DNA Kit (Base Entity)

```yaml
DNAKit:
  inherits: UniversalEntity
  
  # Core Identity
  kit_info:
    kit_id: UUID
    external_kit_id: string  # FamilyTreeDNA, Ancestry, etc.
    test_company: string
    test_date: date
    kit_owner: identity_id
    test_taker: identity_id  # May differ from owner
    
  # Privacy & Consent
  privacy:
    sharing_level: enum ["private", "matches_only", "project_members", "public"]
    real_name_display: boolean
    consent_research: boolean
    consent_law_enforcement: boolean
    gdpr_consent: consent_record
    
  # Test Types Taken
  tests_completed:
    y_dna: boolean
    autosomal: boolean
    mitochondrial: boolean
    x_dna: boolean
    
  # Raw Data Files
  raw_data:
    autosomal_file: encrypted_file_ref
    vcf_file: encrypted_file_ref
    genome_build: string  # "37", "38"
```

### 2. Y-DNA Data (Paternal Line)

```yaml
YDNAData:
  inherits: DNAData
  
  # STR (Short Tandem Repeat) Data
  str_markers:
    - marker_name: string  # "DYS393"
      value: integer  # 13
      multi_copy: [integer]  # For multi-copy markers
      
  str_testing_levels:
    y12: boolean
    y25: boolean
    y37: boolean
    y67: boolean
    y111: boolean
    y700: boolean
    
  # SNP (Single Nucleotide Polymorphism) Data
  snp_data:
    # Terminal SNP
    terminal_snp: string  # "R-FGC11134"
    snp_path: [string]  # Full path from root
    
    # All tested SNPs
    tested_snps:
      - snp_name: string  # "M269"
        position: integer  # GRCh38 position
        ancestral_allele: string  # "T"
        derived_allele: string  # "C"
        result: string  # "+", "-", "no call"
        
  # BigY Results
  bigy_results:
    test_date: date
    named_variants: [string]
    private_variants: [variant]
    novel_variants: [variant]
    coverage_analysis: coverage_data
    
  # Haplogroup Analysis
  haplogroup:
    confirmed: string  # "R-M269"
    predicted: string
    confidence: percent
    subclade_matches: [match]
    phylogenetic_age: integer  # Years
    
  # STR Matching
  str_matches:
    - match_kit: kit_id
      genetic_distance:
        at_12: integer
        at_25: integer
        at_37: integer
        at_67: integer
        at_111: integer
      shared_ancestors: [identity_id]
      mrca_estimate: generation_range
      
  # SNP Matching
  snp_matches:
    - match_kit: kit_id
      shared_snps: [string]
      unique_snps_self: [string]
      unique_snps_match: [string]
      block_years: integer  # Years since common ancestor
```

### 3. Autosomal DNA Data

```yaml
AutosomalDNAData:
  inherits: DNAData
  
  # Summary Statistics
  summary:
    total_cm: float  # Total centiMorgans
    tested_snps: integer
    no_calls: integer
    heterozygosity: float
    
  # Ethnicity/Ancestry Composition
  ethnicity:
    - population: string  # "Scottish"
      percentage: float
      confidence_range: [float, float]
      chromosome_painting: painting_data
      
  # Match Data
  matches:
    - match_kit: kit_id
      total_cm: float
      largest_segment: float
      segment_count: integer
      relationship_prediction:
        - relationship: string  # "2nd cousin"
          probability: float
      shared_matches: [kit_id]  # For clustering
      maternal_side: boolean  # If known
      paternal_side: boolean  # If known
      
  # Segment Data (Detailed)
  segments:
    - chromosome: integer  # 1-22, X
      start_position: integer
      end_position: integer
      genetic_length: float  # cM
      snp_count: integer
      matching_kits: [kit_id]
      phase_set: string  # If phased
      pile_up_region: boolean
      
  # Triangulation Groups
  triangulation_groups:
    - group_id: UUID
      members: [kit_id]  # All kits sharing this segment
      chromosome: integer
      start_pos: integer
      end_pos: integer
      mrca_estimate: identity_id
      confidence: float
      
  # Parent-Child Phasing
  phasing:
    maternal_kit: kit_id
    paternal_kit: kit_id
    phased_segments: [segment]
    crossover_points: [position]
```

### 4. Mitochondrial DNA Data

```yaml
MitochondrialDNAData:
  inherits: DNAData
  
  # Test Levels
  test_level:
    hvr1: boolean
    hvr2: boolean
    coding_region: boolean
    full_sequence: boolean
    
  # Haplogroup
  haplogroup:
    confirmed: string  # "H1a1"
    subclade: string
    defining_mutations: [mutation]
    
  # Mutations from Reference
  mutations:
    - position: integer  # 16519
      reference: string  # "T"
      value: string  # "C"
      region: string  # "HVR1", "HVR2", "CR"
      
  # Matches
  matches:
    - match_kit: kit_id
      genetic_distance: integer
      match_level: string  # "exact", "1-step", "2-step"
      shared_mutations: [mutation]
      extra_mutations_self: [mutation]
      extra_mutations_match: [mutation]
      
  # Ancient DNA Connections
  ancient_connections:
    - ancient_sample: string
      location_found: string
      age_bp: integer  # Years before present
      genetic_distance: integer
      source_study: string
```

### 5. X-DNA Data

```yaml
XDNAData:
  inherits: DNAData
  
  # X-DNA specific matching
  x_matches:
    - match_kit: kit_id
      total_cm: float
      largest_segment: float
      segments: [x_segment]
      possible_paths: [string]  # "maternal", "paternal_grandmother"
      
  # X Inheritance Path Analysis
  inheritance_analysis:
    possible_ancestors: [identity_id]
    excluded_lines: [string]  # "paternal_grandfather"
    probability_maternal: float
    probability_paternal: float
```

### 6. DNA Analysis Results

```yaml
DNAAnalysis:
  inherits: UniversalEntity
  
  # Automated Analysis Results
  analysis_type: enum ["triangulation", "chromosome_mapping", "ethnicity", "haplogroup"]
  
  # Triangulation Analysis
  triangulation_result:
    target_ancestor: identity_id
    supporting_matches: [kit_id]
    confidence_score: float
    conflicting_evidence: [conflict]
    suggested_relationships: [relationship]
    
  # Chromosome Mapping
  chromosome_map:
    - ancestor: identity_id
      segments: [segment]
      total_cm: float
      generation_estimate: integer
      confidence: float
      
  # Network Analysis
  match_network:
    clusters: [cluster]
    isolated_matches: [kit_id]
    suggested_connections: [connection]
    visual_graph: graph_data
```

### 7. DNA Project Integration

```yaml
DNAProject:
  inherits: UniversalEntity
  
  # Project Info
  project_info:
    name: string  # "Henderson Surname Project"
    type: enum ["surname", "geographic", "haplogroup", "ethnic"]
    platform: string  # "FamilyTreeDNA", "custom"
    administrators: [identity_id]
    
  # Member Kits
  members:
    - kit: kit_id
      join_date: date
      consent_level: string
      display_name: string
      grouping: string  # "Caithness Hendersons"
      
  # Project Analysis
  analysis:
    str_modal_haplotype: [marker_value]
    common_ancestors: [identity_id]
    geographic_origins: [location]
    migration_patterns: [path]
    
  # Reporting
  reports:
    - type: "Y-DNA Results Table"
      visibility: enum ["members", "public"]
      last_generated: date
      format: "html|pdf|csv"
```

## Integration with Core Model

### 1. DNA Evidence

```yaml
DNAEvidence:
  extends: Evidence
  
  evidence_type: "DNA"
  
  dna_specific:
    test_type: enum ["Y-DNA", "Autosomal", "Mitochondrial", "X-DNA"]
    supporting_data: dna_data_id
    interpretation: text
    
  # How DNA supports relationships
  relationship_support:
    - claimed_relationship: relationship_id
      dna_evidence_type: string  # "shared_segments", "haplogroup", "triangulation"
      support_strength: enum ["strong", "moderate", "weak", "contradicts"]
      explanation: text
```

### 2. DNA-Enhanced Identity

```yaml
Identity_DNA_Extensions:
  # Added to Identity entity
  
  dna_profiles:
    - kit: kit_id
      test_taker_relationship: string  # "self", "son", "descendant"
      confirmed_by_dna: boolean
      
  genetic_ancestry:
    paternal_line: haplogroup
    maternal_line: haplogroup
    autosomal_ancestry: [population_percentage]
    
  dna_confirmed_relationships:
    - related_identity: identity_id
      relationship_type: string
      dna_evidence: [dna_evidence_id]
      confidence: float
```

### 3. Theory Support via DNA

```yaml
Theory_DNA_Support:
  # How DNA evidence supports/refutes theories
  
  dna_consistency_check:
    - theory: theory_id
      dna_predictions:
        - "If theory true, X and Y should share DNA"
        - "If theory true, haplogroup should be R-M269"
      test_results:
        - prediction_confirmed: boolean
          supporting_evidence: dna_evidence_id
```

## Privacy & Ethical Considerations

```yaml
DNAPrivacy:
  # Special privacy for genetic data
  
  consent_levels:
    - no_sharing: "Data never shared"
    - research_only: "Anonymized for research"
    - match_discovery: "Allow matches to see"
    - full_sharing: "Public profile"
    
  sensitive_discoveries:
    - type: "misattributed_parentage"
      handling: "Private note to kit owner only"
    - type: "cousin_marriage"
      handling: "Highlighted privately"
    - type: "health_implications"
      handling: "Not stored, refer to health testing"
      
  law_enforcement:
    opted_in: boolean
    restrictions: [string]
    audit_log: [access_record]
```

## Import Capabilities

```yaml
DNAImporters:
  ftdna_importer:
    supports:
      - "Y-DNA CSV export"
      - "Family Finder matches"
      - "BigY VCF files"
      - "mtDNA results"
      
  ancestry_importer:
    supports:
      - "DNA matches list"
      - "Shared matches"
      - "Ethnicity estimates"
      
  gedmatch_importer:
    supports:
      - "One-to-many CSV"
      - "Triangulation reports"
      
  generic_importer:
    supports:
      - "23andMe raw data"
      - "VCF files"
      - "Segment data CSV"
```

This comprehensive DNA model:
1. **Supports all DNA test types** (Y, autosomal, mt, X)
2. **Handles both STR and SNP** data for Y-DNA
3. **Enables advanced analysis** (triangulation, chromosome mapping)
4. **Maintains privacy** and consent
5. **Integrates with theories** and evidence
6. **Imports from major testing companies**

The model is extensible for future DNA analysis methods we can't yet imagine!