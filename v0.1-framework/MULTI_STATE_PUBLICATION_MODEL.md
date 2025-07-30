# Multi-State Publication & Branching Model

## Beyond Binary: A Spectrum of States

Instead of just "working" vs "published", we need a rich state model that reflects how research actually flows through communities.

## Enhanced State Model

```yaml
EntityStateModel:
  # Core States (not just two!)
  publication_states:
    private_draft:
      visibility: "author_only"
      mutability: "full"
      branching: "self_only"
      
    team_working:
      visibility: "team_members"
      mutability: "full"
      branching: "team_only"
      collaboration: "real_time"
      
    shared_draft:
      visibility: "invited_reviewers"
      mutability: "suggestions_only"
      branching: "with_permission"
      attribution: "track_all_input"
      
    open_theory:  # The key innovation!
      visibility: "public"
      mutability: "author_can_update"
      branching: "anyone_can_fork"
      versioning: "semantic"
      citation: "version_specific"
      
    peer_review:
      visibility: "reviewers"
      mutability: "locked_for_review"
      branching: "prohibited"
      outcome: ["approved", "revisions", "rejected"]
      
    published_living:  # Published but evolving
      visibility: "public"
      mutability: "author_updates_only"
      branching: "encouraged"
      versioning: "major.minor.patch"
      citation: "includes_version"
      deprecation: "old_versions_accessible"
      
    published_final:
      visibility: "public"
      mutability: "errata_only"
      branching: "allowed"
      preservation: "permanent"
      
    archived_historical:
      visibility: "public"
      mutability: "none"
      branching: "allowed"
      context: "preserved_forever"
      
    retracted:
      visibility: "metadata_only"
      mutability: "none"
      reason: "required"
      replacement: "linked"
```

## Branching & Forking System

```yaml
BranchingModel:
  # Branch Types
  branch_types:
    exploration_branch:
      purpose: "Test alternative theories"
      merge_back: "optional"
      attribution: "original_preserved"
      
    collaborative_fork:
      purpose: "Build upon others' work"
      merge_back: "discouraged"
      attribution: "chain_of_contribution"
      citation: "cites_parent"
      
    translation_branch:
      purpose: "Adapt for different context"
      merge_back: "never"
      attribution: "translator_credit"
      relationship: "derived_work"
      
    correction_branch:
      purpose: "Fix errors in original"
      merge_back: "pull_request"
      attribution: "corrector_noted"
      
    extension_branch:
      purpose: "Add new evidence/analysis"
      merge_back: "author_discretion"
      attribution: "additional_contributor"
      
  # Fork Governance
  fork_permissions:
    open_license:
      fork_allowed: "always"
      attribution_required: "yes"
      modifications_allowed: "yes"
      commercial_use: "allowed"
      
    academic_license:
      fork_allowed: "with_citation"
      attribution_required: "detailed"
      modifications_allowed: "with_notation"
      peer_review: "encouraged"
      
    restricted_license:
      fork_allowed: "with_permission"
      attribution_required: "negotiated"
      modifications_allowed: "case_by_case"
      
  # Relationship Tracking
  genealogy_of_ideas:
    parent_entity: entity_id
    fork_timestamp: timestamp
    fork_reason: text
    
    divergence_metrics:
      content_similarity: percent
      structural_changes: count
      evidence_overlap: percent
      conclusion_alignment: boolean
      
    contribution_flow:
      inherited_from_parent: percent
      new_contributions: percent
      modified_content: percent
```

## Living Document Features

```yaml
LivingDocuments:
  # Version Stream
  version_stream:
    current_version: "3.2.1"
    
    version_history: [
      {
        version: "3.2.1",
        date: "2024-03-15",
        changes: ["Fixed birth date calculation"],
        author: researcher_id,
        impact: "minor"
      },
      {
        version: "3.2.0",
        date: "2024-03-01",
        changes: ["Added new census evidence"],
        author: researcher_id,
        impact: "minor_feature"
      },
      {
        version: "3.0.0",
        date: "2024-01-15",
        changes: ["Revised main hypothesis"],
        author: researcher_id,
        impact: "breaking_change"
      }
    ]
    
  # Subscription Model
  subscriber_notifications:
    subscription_types:
      - "major_changes_only"
      - "all_updates"
      - "evidence_additions"
      - "conclusion_changes"
      
    notification_methods:
      - email
      - in_app
      - rss_feed
      - webhook
      
  # Collaborative Evolution
  evolution_tracking:
    downstream_works: [
      {
        entity_id: UUID,
        relationship: "extends",
        author: researcher_id,
        description: "Added DNA evidence"
      },
      {
        entity_id: UUID,
        relationship: "challenges",
        author: researcher_id,
        description: "Alternative interpretation"
      }
    ]
    
    upstream_dependencies: [
      {
        entity_id: UUID,
        relationship: "builds_upon",
        version_used: "2.1.0",
        still_compatible: boolean
      }
    ]
```

## Theory Evolution Example

```yaml
TheoryEvolutionExample:
  original_theory:
    id: "theory_001"
    title: "John Smith of Ohio Theory"
    version: "1.0.0"
    state: "published_living"
    author: "researcher_123"
    
  community_development:
    fork_1:
      id: "theory_001_fork_1"
      title: "John Smith - DNA Enhanced Analysis"
      forked_from: "theory_001@1.0.0"
      author: "researcher_456"
      changes: "Added Y-DNA evidence"
      state: "open_theory"
      
      # Original author incorporates back
      merged_back:
        into_version: "2.0.0"
        attribution: "DNA analysis by researcher_456"
        
    fork_2:
      id: "theory_001_fork_2"
      title: "Two John Smiths Theory"
      forked_from: "theory_001@1.5.0"
      author: "researcher_789"
      changes: "Proposed split into two people"
      state: "open_theory"
      relationship: "challenges_original"
      
    fork_3:
      id: "theory_001_fork_3"
      title: "John Smith German Origins"
      forked_from: "theory_001@2.0.0"
      author: "researcher_012"
      changes: "Added German church records"
      state: "published_living"
      
  # Network Effect
  theory_network:
    nodes: [theory_001, fork_1, fork_2, fork_3]
    edges: [
      {from: theory_001, to: fork_1, type: "inspired"},
      {from: fork_1, to: theory_001, type: "merged_back"},
      {from: theory_001, to: fork_2, type: "challenged_by"},
      {from: theory_001, to: fork_3, type: "extended_by"}
    ]
```

## Citation in Multi-State World

```yaml
FlexibleCitation:
  # Citing Living Documents
  citation_modes:
    cite_specific_version:
      text: "Smith, J. (2024). 'John of Ohio.' ResearchGPS, v2.1.0. DOI:10.1234/theory_001@2.1.0"
      permanence: "guaranteed"
      
    cite_latest:
      text: "Smith, J. (2024-). 'John of Ohio.' ResearchGPS, living document. DOI:10.1234/theory_001"
      note: "Accessed 2024-03-15, latest version"
      
    cite_range:
      text: "Smith, J. (2024). 'John of Ohio.' ResearchGPS, v2.0.0-2.5.0"
      purpose: "When discussing evolution of idea"
      
    cite_with_forks:
      text: "Smith, J. et al. (2024). 'John of Ohio' theory network. ResearchGPS. See also: Jones fork, Brown challenge."
      
  # Attribution Chains
  attribution_preservation:
    original_author: "always_credited"
    contributors: "listed_by_significance"
    fork_authors: "independent_credit"
    merger_credit: "documented"
```

## Benefits of Multi-State Model

1. **Research as Living Process**
   - Ideas evolve naturally
   - Community can build together
   - Attribution preserves credit
   - Nothing is lost

2. **Encourages Collaboration**
   - Fork and improve others' work
   - Challenge with alternative theories
   - Merge back improvements
   - Build research networks

3. **Maintains Rigor**
   - Every version citable
   - All changes tracked
   - Attribution automatic
   - Quality through transparency

4. **Reflects Reality**
   - Research isn't binary (draft/done)
   - Ideas build on ideas
   - Theories compete and merge
   - Knowledge evolves

This model truly captures how genealogical research works - as a living, breathing, collaborative process where ideas build upon each other while maintaining rigorous attribution and citation capability.