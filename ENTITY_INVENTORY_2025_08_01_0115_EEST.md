# ResearchProcess-GPS Complete Entity Inventory
### Timestamp: 2025-08-01 01:15:00 EEST
### Total Count: 19 Entity implementations

---

## 📊 ENTITY BREAKDOWN

### Entities with NestableEntity trait (15 total):
1. **Analysis** - Flexible analytical reasoning container
2. **Citation** - Source references that can be grouped
3. **Evidence** - Extracted information that can be hierarchical
4. **Fact** - Atomic claims that can be grouped
5. **IdentityPersona** - Evidence-based person references
6. **Location** - Places with hierarchical structure
7. **ResearchSession** - Work sessions that can nest
8. **Source** - Hierarchical source management
9. **Theory** - Research questions with sub-theories
10. **WorkProduct** - Outputs that can contain sections
11. **Confidence** - Assessment narratives (uses impl_entity! but no explicit NestableEntity shown)
12. **Relationship** - Entity connections (uses impl_entity! but no explicit NestableEntity shown)
13. **ResearchActivity** - Atomic research tasks (uses impl_entity! but no explicit NestableEntity shown)
14. **Researcher** - Agents/actors (uses impl_entity! but no explicit NestableEntity shown)

**Note**: Some entities use impl_entity! macro but don't explicitly show NestableEntity implementation in the grep results. Need to verify actual trait implementations.

### Entities with direct Entity trait (4 total):
15. **EvidenceAnalysis** - Analysis work products (to be renamed AnalysisReport)
16. **ProofStatement** - GPS proof argument documents
17. **ResearchLog** - Linear process documentation
18. **Workspace** - User research environments

**Missing 1 entity to reach 19 total** - Need to investigate further

---

## 🔍 ENTITIES MISSING FROM EntityType ENUM

The current EntityType enum (13 entries) is missing these 7 entities:
1. Analysis
2. Confidence  
3. ResearchSession
4. ResearchActivity
5. Location
6. Researcher
7. Workspace

And incorrectly includes:
- Repository (which is a SourceType variant, not an entity)

---

## 📁 LAYER ORGANIZATION

### Layer 1 - Core Data Model (9 entities):
- Source, Citation, Evidence
- IdentityPersona, Relationship, Location  
- Fact, Confidence, Analysis

### Layer 2 - Research Process & Products (9 entities):
- Theory, ResearchSession, ResearchActivity, ResearchLog, Researcher
- WorkProduct, ProofStatement, EvidenceAnalysis

### Layer 3 - Workflow & Configuration (1 entity):
- Workspace

---

## ⚠️ INVESTIGATION NEEDED

1. **Missing 19th entity** - The grep results show 18 entities, but documentation claims 19
2. **NestableEntity verification** - Some entities use impl_entity! but don't show explicit NestableEntity implementation
3. **ConfigEntity implementations** - These are separate and not part of the 19 entity count

---

## 🚀 NEXT STEPS

1. Verify the exact trait implementations for each entity
2. Find the missing 19th entity
3. Update EntityType enum with all 19 entities
4. Remove Repository from EntityType enum
5. Fix entity_type_mapper to handle all entities correctly