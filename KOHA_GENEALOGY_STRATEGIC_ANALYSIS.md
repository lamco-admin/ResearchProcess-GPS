# Koha for Genealogical Research: Strategic Analysis

**Document Created**: 2025-11-09
**Purpose**: Evaluate Koha library management system as potential platform/solution for genealogical research management

---

## Executive Summary

**Koha** is the world's first open-source Integrated Library System (ILS), used by thousands of libraries worldwide. While not designed for genealogy, it possesses several architectural elements that align **surprisingly well** with genealogical research management needs.

### Key Finding

Koha's **authority control system** is architecturally similar to what ResearchProcess-GPS needs for managing canonical person/place records, and its **source-centric design** (bibliographic records representing source documents) maps naturally to genealogical research methodology.

### Strategic Recommendation

**Explore Koha as an integration target and architectural inspiration, not as a replacement platform.**

Create a new branch (`claude/koha-integration-exploration`) to:
1. Develop Koha adapter/plugin for ResearchProcess-GPS
2. Test authority control concepts
3. Evaluate bidirectional sync possibilities
4. Learn from its mature plugin architecture

---

## Part 1: What is Koha?

### Core Identity

- **World's first open-source library management system** (since 1999)
- **25+ years of development** with active community
- **Used by thousands of libraries** worldwide (public, academic, special)
- **Full-featured ILS**: cataloging, circulation, acquisitions, authorities, OPAC
- **Technology**: Perl/Mojolicious backend, MariaDB/MySQL, Template Toolkit frontend
- **License**: GNU GPL v3

### Architecture Highlights

**Database**: 300+ tables with full referential integrity
**API**: REST API with OpenAPI/Swagger specification
**Search**: Dual engine support (Zebra/Elasticsearch)
**Plugins**: Robust plugin system with lifecycle management
**Standards**: MARC21, UNIMARC, Dublin Core, Z39.50, OAI-PMH

---

## Part 2: Architectural Alignments with Genealogical Research

### 1. Authority Control System ⭐⭐⭐

**This is the killer feature for genealogy!**

#### How Koha's Authority System Works

```
Authority Record (Canonical Entity)
├── authid: Unique identifier
├── heading: "Smith, John, 1820-1891"
├── marcxml: Full MARC with variants, notes, dates
├── authtrees: Hierarchical relationships (broader/narrower)
├── linkid: Links to related authorities
└── Linked bibliographic records: All sources mentioning this person
```

**Authority Types**:
- Personal names (PERSO_NAME)
- Corporate names (CORPO_NAME)
- Geographic names (GEOGR_NAME)
- Subject terms (TOPIC_TERM)
- Uniform titles

#### Genealogical Mapping

| Koha Authority | Genealogical Equivalent |
|----------------|------------------------|
| Personal Name Authority | **Canonical Person Record** |
| Geographic Name Authority | **Place Name Master** |
| Subject Authority | **Event Type** |
| See From References | **Name Variants/Aliases** |
| See Also References | **Related Persons** |
| Authority Hierarchy (`authtrees`) | **Family Structure** |

**Example: Person Authority**
```
Authority: "Smith, John, 1820-1891"
├── Heading: "Smith, John, 1820-1891"
├── Birth: 1820 (from MARC 100$d)
├── Death: 1891 (from MARC 100$d)
├── Variant forms:
│   ├── "John Smith"
│   ├── "J. Smith"
│   └── "Johann Schmidt" (immigration name)
├── Related to:
│   ├── "Smith, Mary, 1822-1895" (spouse)
│   └── "Smith, William, 1790-1870" (father)
└── Appears in sources:
    ├── 1850 Census, Ohio
    ├── Marriage Record, 1845
    └── Probate File, 1892
```

#### Why This Matters

**Solves the "Which John Smith?" problem!**

- One **canonical authority record** per person
- All **source documents** (bibliographic records) link to it
- **Automatic aggregation** of information from multiple sources
- **Variant name handling** built-in
- **Relationship modeling** via authority hierarchies
- **Usage tracking** shows which sources mention each person

### 2. Source-Centric Design ⭐⭐⭐

#### Three-Tier Model

```
1. BIBLIO (Bibliographic Record)
   └── Represents: Source document
       Examples: Census record, vital record, will, deed

2. BIBLIO_METADATA (MARC Storage)
   └── Full MARC record with all fields
       Complete source citation data

3. ITEMS (Physical/Digital Copies)
   └── Represents: Specific source citations
       Location, repository, call number
```

**Genealogical Mapping**:
- **Biblio** = Source Document (death certificate, census page, will)
- **Biblio_metadata** = Full source details in MARC
- **Items** = Citations to that source (where to find it)

**Example**:
```
BIBLIO: "1850 U.S. Census, Ohio, Summit County"
├── MARC 260: Publication (Washington: National Archives, 1850)
├── MARC 500: Notes (Enumeration date: Aug 1850)
├── MARC 651: Geographic (Ohio--Summit County)
└── ITEMS:
    ├── Item 1: Microfilm at National Archives (M432, Roll 712)
    ├── Item 2: Digital image at Ancestry.com
    └── Item 3: FamilySearch image (Film 1234567)
```

#### Authority Linking

```
Biblio (Source) → MARC 600 field → Authority (Person)
                  MARC 651 field → Authority (Place)
                  MARC 650 field → Authority (Event Type)
```

**Result**: Every source automatically linked to the people/places it mentions!

### 3. Relationship Modeling ⭐⭐

#### Borrower Relationships Table

**THIS IS GENEALOGICALLY PERFECT!**

```sql
CREATE TABLE borrower_relationships (
  id int(11) NOT NULL AUTO_INCREMENT,
  guarantor_id int(11) NOT NULL,        -- Parent/guardian
  guarantee_id int(11) NOT NULL,        -- Child/dependent
  relationship varchar(100) NOT NULL,   -- Type of relationship
  PRIMARY KEY (id)
)
```

**Direct Genealogical Application**:
- `guarantor_id` → Parent
- `guarantee_id` → Child
- `relationship` → "Parent-Child", "Spouse", "Sibling"

**Authority Hierarchies**:
- `auth_header.authtrees` → Family tree structure
- Broader/narrower term relationships → Parent/child
- See also references → Spouse, sibling, cousin

### 4. Provenance & Research Trail ⭐⭐⭐

#### Action Logs Table

```sql
CREATE TABLE action_logs (
  action_id int(11) NOT NULL AUTO_INCREMENT,
  timestamp timestamp NOT NULL,
  user int(11) NOT NULL,
  module mediumtext DEFAULT NULL,
  action mediumtext DEFAULT NULL,      -- ADDED, DELETED, MODIFIED
  object int(11) DEFAULT NULL,
  info mediumtext DEFAULT NULL,        -- Details
  diff longtext DEFAULT NULL,          -- Changed data
  ...
)
```

**Complete audit trail**:
- Who added information
- When it was added
- What changed
- Full diff of modifications

**Genealogical Application**: Perfect research log!

#### Item Provenance

```sql
items:
  dateaccessioned    -- When source acquired
  booksellerid       -- Where it came from (archive, repository)
  itemnotes          -- Research notes
  itemnotes_nonpublic -- Private research notes
```

**Research Documentation**: Where you got each source, when, from whom

### 5. Metadata Flexibility ⭐⭐

#### MARC Extensibility

- **MARC21**: Fully extensible bibliographic format
- **Local fields**: 9XX fields for custom data
- **Custom frameworks**: Different MARC templates per record type
- **Subfields**: Infinite granularity

#### Additional Fields System

```sql
additional_fields:
  name, authorised_value_category, marcfield, searchable

additional_field_values:
  field_id, record_id, value
```

**Custom genealogical fields**:
- DNA test IDs
- Probabilities
- Research hypotheses
- Custom event types
- Non-standard dates

### 6. Search & Discovery ⭐⭐⭐

#### Dual Search Engine

**Elasticsearch Features**:
- Full-text search across all fields
- Faceted search (filter by type, date, place, etc.)
- Field-specific queries
- Boolean operators
- Date range searching
- Geographic searching

**Example Genealogical Queries**:
1. `"Smith, John" AND Ohio AND date:[1850 TO 1860]`
2. `subject:"Marriage" AND place:"Kent County, Michigan"`
3. `author:"Jones, Sarah" AND type:"Birth Certificate"`

#### Authority Search

- Search across all name variants
- Find all sources mentioning a person
- Geographic hierarchy searching
- Subject/event type filtering

### 7. Plugin System ⭐⭐⭐

#### Plugin Capabilities

**Lifecycle Management**:
```perl
sub install { }   # Run on first install
sub upgrade { }   # Run on version change
sub uninstall { } # Cleanup
```

**Data Persistence**:
```perl
$self->store_data({ key => 'value' });
$self->retrieve_data('key');
```

**Hook Points**:
- `after_hold_create`, `after_circ_action`
- `intranet_js`, `opac_js` (JavaScript injection)
- `tool()` - Custom tools in interface
- `report()` - Custom reports
- `api_routes()` - Custom REST endpoints

**Template Integration**:
- Inject HTML/CSS/JavaScript
- Custom pages
- Modify existing pages

#### Potential Genealogy Plugins

**GEDCOM Bridge Plugin**:
```perl
package Koha::Plugin::GEDCOMBridge;

sub tool {
    # GEDCOM import wizard
    # Convert GEDCOM → Koha authorities/biblios
}

sub api_routes {
    return [
        { path => '/gedcom/import', method => 'POST', ... },
        { path => '/gedcom/export/:person_id', method => 'GET', ... },
    ];
}
```

**ResearchProcess-GPS Integration Plugin**:
```perl
package Koha::Plugin::ResearchProcessGPS;

sub api_routes {
    return [
        { path => '/rp-gps/sync', ... },
        { path => '/rp-gps/person/:id', ... },
        { path => '/rp-gps/theory/:id', ... },
    ];
}
```

**Family Tree Visualizer**:
```perl
sub tool {
    # D3.js tree visualization
    # Use authtrees for hierarchy
}
```

**Timeline Generator**:
```perl
sub report {
    # Extract dates from authorities
    # Generate chronological view
}
```

### 8. Multi-User Collaboration ⭐⭐

#### Patron System

```
Patron (Borrower) → Researcher
├── Categories → Research roles (admin, contributor, viewer)
├── Permissions → Granular access control
├── Messages → Researcher communication
└── Patron attributes → Custom researcher metadata
```

**Built-in collaboration**:
- Multiple researchers per project
- Attribution (who added what)
- Message system
- Permission levels
- Virtual shelves (shared collections)

### 9. REST API ⭐⭐⭐

#### OpenAPI Specification

**199 API endpoint definitions** in `api/v1/swagger/`

**Key Endpoints**:
- `/api/v1/biblios` - Source documents
- `/api/v1/authorities` - Person/place records
- `/api/v1/items` - Source citations
- Custom plugin endpoints

**Authentication**:
- OAuth2
- API keys
- Session-based

**Integration Possibilities**:
- FamilySearch API ↔ Koha API
- Ancestry.com ↔ Koha
- ResearchProcess-GPS ↔ Koha
- FindAGrave ↔ Koha authorities

---

## Part 3: Mapping Koha to Genealogical Concepts

### Conceptual Translation Table

| Library Concept | Genealogical Equivalent | Koha Implementation |
|----------------|------------------------|---------------------|
| **Authority Record** | Canonical Person/Place | `auth_header` table |
| **Bibliographic Record** | Source Document | `biblio` table |
| **Item** | Source Citation | `items` table |
| **Patron** | Researcher | `borrowers` table |
| **Virtual Shelf** | Research Collection | `virtualshelves` |
| **Acquisition Order** | Research Task | `aqorders` |
| **Subject Heading** | Event Type | Subject authorities |
| **Author** | Person mentioned in source | MARC 100/700 → authority |
| **Publisher/Place** | Event Location | MARC 260/264 → geographic authority |
| **Series** | Family Group | Biblio relationships |
| **Collection** | Family Line | `items.ccode` |
| **Branch** | Research Project | `branches` table |

### MARC → Genealogical Event Mapping

**Example: Birth Record**

```
MARC Record for "Birth Certificate: John Smith, 1850"
100 1_ |a Smith, John |d 1850-1891
245 10 |a Birth certificate
260 __ |a [Ohio] : |b Summit County Recorder, |c 1850
300 __ |a 1 certificate
500 __ |a Birth date: March 15, 1850
500 __ |a Birth place: Akron, Summit County, Ohio
500 __ |a Parents: William Smith and Mary Jones
600 10 |a Smith, John, |d 1850-1891
600 10 |a Smith, William |d 1790-1870
600 10 |a Jones, Mary |d 1795-1875
651 _0 |a Akron (Ohio)
655 _7 |a Birth certificates
```

**Authority Links**:
- MARC 600 fields automatically link to person authorities
- MARC 651 field links to geographic authority for Akron, Ohio
- MARC 655 field links to genre/form authority for "Birth certificates"

**Result**:
- Each person's authority record shows this source
- Geographic search for Akron returns this record
- Event type search for "Birth" returns this record

### Workflow Example: Research Session

**Scenario**: Researching John Smith in Ohio

1. **Search Authority**: "Smith, John, 1850-1891"
   - Returns canonical authority record
   - Shows all linked sources (census, birth, marriage, death, etc.)

2. **Add New Source**:
   - Catalog 1860 census page as bibliographic record
   - Link to John Smith authority via MARC 600 field
   - Koha auto-updates John's authority "usage count"
   - John's authority record now shows 1860 census

3. **Track Research**:
   - Acquisition order for death certificate from archives
   - Virtual shelf for "John Smith research"
   - Research notes in item/biblio notes fields

4. **Collaborate**:
   - Share virtual shelf with co-researcher
   - Co-researcher adds sources
   - Action logs track who added what

5. **Generate Reports**:
   - Custom report: All sources for John Smith
   - Timeline of John's life events (from source dates)
   - Family group sheet (from authority relationships)

---

## Part 4: Comparison with ResearchProcess-GPS Architectures

### Architecture Comparison Matrix

| Feature | Koha | RP-GPS Original | RP-GPS Meta-Model |
|---------|------|-----------------|-------------------|
| **Primary Design** | Library cataloging | GPS research process | Universal data model |
| **Entity Model** | MARC bibliographic | 18 concrete entities | 4 universal primitives |
| **Metadata** | MARC21/UNIMARC | PropertyGraph | PropertyGraph |
| **Authority Control** | ✅ Built-in, mature | Planned (Person entity) | Universal Entity |
| **Source Management** | ✅ Core feature | Citation entity | Universal Entity |
| **Relationships** | ✅ Via authorities | Relationship entity | Universal Relationship |
| **State Machines** | ❌ None | ✅ Complete | ✅ Complete |
| **Theory Versioning** | ❌ None | Planned | Quantum states |
| **Confidence Framework** | ❌ None | ✅ Designed | Certainty types |
| **Plugin System** | ✅ Mature | ✅ WASM/native | ✅ WASM/native |
| **REST API** | ✅ OpenAPI | ✅ 98% complete | ✅ Complete |
| **Multi-User** | ✅ Built-in | Designed | Designed |
| **Search Engine** | ✅ Elasticsearch | PostgreSQL FTS | PropertyGraph query |
| **Audit Trail** | ✅ action_logs | Event sourcing | Event sourcing |
| **Production Ready** | ✅ Yes (25 years) | ❌ 40% done | ✅ Web UI working |

### Strengths & Weaknesses

**Koha Strengths**:
- ✅ **Mature, battle-tested** (25+ years, thousands of installations)
- ✅ **Authority control is exactly what genealogy needs**
- ✅ **Source-centric design** aligns with research methodology
- ✅ **Robust search** (Elasticsearch)
- ✅ **Multi-user collaboration** built-in
- ✅ **Active development community**
- ✅ **Comprehensive API**
- ✅ **Proven scalability**
- ✅ **Plugin system** for extensions

**Koha Weaknesses**:
- ❌ **Not designed for genealogy** (requires adaptation)
- ❌ **MARC learning curve** (complex for non-librarians)
- ❌ **Library-centric features** add unnecessary complexity
- ❌ **No GPS compliance** features
- ❌ **No theory versioning**
- ❌ **No confidence framework**
- ❌ **No GEDCOM native support**
- ❌ **No built-in visualizations** (pedigree charts, timelines)

**ResearchProcess-GPS Strengths**:
- ✅ **Designed specifically for genealogical research**
- ✅ **GPS compliance built-in**
- ✅ **Theory versioning** (designed)
- ✅ **Comprehensive confidence framework**
- ✅ **Research process focus** (not just sources)
- ✅ **Meta-model flexibility** (universal data model)
- ✅ **Modern architecture** (Rust, WASM)

**ResearchProcess-GPS Weaknesses**:
- ❌ **Only 40% implemented** (original architecture)
- ❌ **No mature authority control yet**
- ❌ **Smaller community** (new project)
- ❌ **Less battle-tested**
- ❌ **No production deployments yet**

---

## Part 5: Strategic Options

### Option 1: Use Koha Directly (Adapt Koha for Genealogy)

**Approach**: Install Koha, customize via plugins and configuration

**Pros**:
- ✅ Working system **today**
- ✅ Proven stability
- ✅ Authority control ready to use
- ✅ Multi-user collaboration included
- ✅ Powerful search out-of-box

**Cons**:
- ❌ Significant adaptation needed
- ❌ MARC expertise required
- ❌ Library features add complexity
- ❌ Missing genealogy-specific features
- ❌ Would abandon ResearchProcess-GPS work

**Effort**: Medium (configuration + plugins)
**Timeline**: 2-3 months for basic genealogical adaptation
**Risk**: Medium (learning curve, user acceptance)

**Verdict**: ⚠️ **Not recommended** - Too much excellent ResearchProcess-GPS work would be wasted

---

### Option 2: Koha Plugin for ResearchProcess-GPS Sync

**Approach**: Develop Koha plugin that syncs with ResearchProcess-GPS via APIs

**Architecture**:
```
ResearchProcess-GPS (Research Platform)
         ↕ REST API
Koha Plugin (Authority & Source Management)
         ↓
    Koha Database
```

**Workflow**:
1. Research in ResearchProcess-GPS (theories, hypotheses, analysis)
2. Concluded identities → Sync to Koha authorities
3. Sources cataloged in Koha → Reference from RP-GPS
4. Koha provides authority control, Koha search, source cataloging
5. RP-GPS provides research process, GPS compliance, theory versioning

**Pros**:
- ✅ Leverage Koha's mature authority system
- ✅ Use Koha's powerful search
- ✅ Preserve ResearchProcess-GPS research features
- ✅ Best of both worlds
- ✅ Can use existing Koha installations

**Cons**:
- ⚠️ Two systems to maintain
- ⚠️ Sync complexity
- ⚠️ Requires Koha installation

**Effort**: High (plugin development + API integration)
**Timeline**: 3-4 months
**Risk**: Medium (sync reliability, two systems)

**Verdict**: ✅ **Viable option** - Good for organizations already using Koha

---

### Option 3: ResearchProcess-GPS Adapter for Koha (Inspired Architecture)

**Approach**: Build ResearchProcess-GPS adapters/exporters for Koha interoperability

**Architecture**:
```
ResearchProcess-GPS
    ├── Core research platform
    ├── Internal data model (18 entities or meta-model)
    └── Adapters:
        ├── Koha MARC exporter (concluded research → MARC)
        ├── Koha authority importer (authorities → RP-GPS)
        └── Koha API client (query Koha for sources)
```

**Workflow**:
1. Research in ResearchProcess-GPS
2. Export concluded identities as Koha MARC authorities
3. Import Koha authorities as starting points for research
4. Query Koha catalog for sources during research

**Pros**:
- ✅ ResearchProcess-GPS remains independent
- ✅ Optional Koha integration
- ✅ Can work with or without Koha
- ✅ Learn from Koha's authority architecture

**Cons**:
- ⚠️ One-way sync (RP-GPS → Koha easier than bidirectional)
- ⚠️ Limited benefit without Koha installation

**Effort**: Medium (adapter development)
**Timeline**: 2-3 months
**Risk**: Low (doesn't break existing RP-GPS)

**Verdict**: ✅ **Good option** - Especially for export to existing library systems

---

### Option 4: Architectural Inspiration (Learn from Koha)

**Approach**: Study Koha's authority control and plugin architecture, implement similar patterns in ResearchProcess-GPS

**What to Learn**:

1. **Authority Control Architecture**:
   - How Koha manages canonical entities
   - How it handles variant names
   - How it links authorities to records
   - How it builds hierarchies

2. **Plugin System Patterns**:
   - Lifecycle management (install/upgrade/uninstall)
   - Hook system design
   - Data persistence for plugins
   - API route registration

3. **Search Architecture**:
   - Elasticsearch integration patterns
   - Faceted search implementation
   - Authority embedding in search

4. **MARC Flexibility**:
   - How MARC achieves extensibility
   - Framework system for different record types
   - Local field usage patterns

**Implementation**:
```
ResearchProcess-GPS
├── Enhanced Authority System
│   ├── Canonical person/place records
│   ├── Variant handling (like Koha authorities)
│   ├── Hierarchical relationships
│   └── Auto-linking from sources
├── Improved Plugin System
│   ├── Lifecycle hooks (learned from Koha)
│   └── Plugin data persistence
└── Enhanced Search
    ├── Elasticsearch option (like Koha)
    └── Authority-embedded search
```

**Pros**:
- ✅ Keep ResearchProcess-GPS independent
- ✅ Learn from 25 years of proven patterns
- ✅ No external dependencies
- ✅ Strengthen RP-GPS architecture

**Cons**:
- ⚠️ Development time for implementation
- ⚠️ Doesn't leverage existing Koha installations

**Effort**: Medium-High (study + implement)
**Timeline**: 3-6 months
**Risk**: Low (improves RP-GPS)

**Verdict**: ✅ **RECOMMENDED** - Best learning opportunity

---

### Option 5: Hybrid Approach (Plugin + Inspiration)

**Approach**:
1. Implement Koha-inspired authority system in ResearchProcess-GPS
2. Develop Koha plugin for optional integration
3. Create MARC export adapter

**Best of All Worlds**:
- RP-GPS gains mature authority control patterns
- Optional Koha integration for those who want it
- MARC export for library systems
- Independent operation

**Pros**:
- ✅ Maximum flexibility
- ✅ Learn from Koha
- ✅ Optional interoperability
- ✅ Serves multiple user types

**Cons**:
- ⚠️ Most complex option
- ⚠️ Longest timeline
- ⚠️ Highest development effort

**Effort**: High (multiple components)
**Timeline**: 6-9 months
**Risk**: Medium (scope creep)

**Verdict**: ✅ **BEST LONG-TERM** - Most comprehensive solution

---

## Part 6: Recommendation & Next Steps

### Primary Recommendation: **Option 4 + Elements of Option 3**

**Strategic Approach**:

1. **Phase 1: Study & Learn** (Current - Week 4)
   - Create new branch: `claude/koha-integration-exploration`
   - Deep study of Koha's authority control architecture
   - Analyze plugin system patterns
   - Document learnings

2. **Phase 2: Enhance RP-GPS Authority System** (Weeks 5-12)
   - Implement Person entity (canonical individuals)
   - Add variant name handling (inspired by Koha)
   - Build authority linking system
   - Implement hierarchical relationships

3. **Phase 3: Build MARC Adapter** (Weeks 13-16)
   - MARC21 exporter for concluded research
   - MARC authority importer
   - Test with real Koha installation

4. **Phase 4: Optional Plugin** (Future)
   - If demand exists, build Koha plugin
   - Bidirectional sync
   - Deploy to library genealogy departments

### Why This Approach?

**Strengths**:
- ✅ Preserves ResearchProcess-GPS independence
- ✅ Learns from 25 years of proven patterns
- ✅ Creates interoperability option
- ✅ Serves both individual researchers AND institutions
- ✅ Doesn't abandon existing RP-GPS work
- ✅ Low risk, high value

**Value Delivered**:
1. **Better authority control** in RP-GPS (learned from Koha)
2. **MARC export capability** (for libraries and archivists)
3. **Optional Koha integration** (for institutions)
4. **Proven architectural patterns** (plugin system, search, etc.)

### Specific Technical Learnings to Apply

**1. Authority Control Architecture**:

Study: `/tmp/koha-exploration/Koha/Authority.pm`
```perl
sub linked_biblionumbers {
    # Get all sources mentioning this authority
}
```

**Apply to RP-GPS**:
```rust
impl Person {
    fn linked_sources(&self) -> Vec<SourceId> {
        // All sources mentioning this person
    }
    fn variant_names(&self) -> Vec<String> {
        // All name forms
    }
    fn hierarchical_relationships(&self) -> FamilyTree {
        // Parent/child structure
    }
}
```

**2. Plugin Lifecycle Management**:

Study: `/tmp/koha-exploration/Koha/Plugins/Base.pm`
```perl
sub new {
    if ( $self->can('install') && !$self->retrieve_data('__INSTALLED__') ) {
        $self->install();
    }
    elsif ( $self->can('upgrade') ) {
        $self->upgrade();
    }
}
```

**Apply to RP-GPS**:
```rust
// Enhance existing module system with lifecycle hooks
trait ModuleLifecycle {
    fn on_install(&mut self) -> Result<()>;
    fn on_upgrade(&mut self, old_version: &str) -> Result<()>;
    fn on_uninstall(&mut self) -> Result<()>;
}
```

**3. Search with Embedded Authorities**:

Study: `/tmp/koha-exploration/Koha/SearchEngine/Elasticsearch.pm`
```perl
use Koha::Filter::MARC::EmbedSeeFromHeadings;
# Embeds authority variants into search index
```

**Apply to RP-GPS**:
```rust
// When indexing sources, embed person variants
struct SourceIndex {
    source_id: Uuid,
    person_canonical_names: Vec<String>,
    person_variant_names: Vec<String>,  // For broader matching
    // ...
}
```

**4. MARC Flexibility Pattern**:

Study: Framework system allowing multiple cataloging templates

**Apply to RP-GPS**:
```rust
// Schema system for different source types
struct SourceSchema {
    source_type: String,  // "Census", "Vital Record", "Will", etc.
    required_fields: Vec<FieldDefinition>,
    optional_fields: Vec<FieldDefinition>,
    validation_rules: Vec<Rule>,
}
```

---

## Part 7: Branch Strategy

### Recommended Branch Structure

```
master (or main development branch)
├── Current work continues
│
└── claude/koha-integration-exploration (NEW BRANCH)
    ├── Phase 1: Research & Documentation
    │   ├── Koha architecture analysis
    │   ├── Authority control study
    │   └── Plugin pattern documentation
    │
    ├── Phase 2: Authority System Enhancement
    │   ├── Person entity implementation
    │   ├── Variant name handling
    │   └── Authority linking
    │
    ├── Phase 3: MARC Adapter
    │   ├── MARC21 exporter
    │   ├── Authority importer
    │   └── Testing with Koha
    │
    └── Phase 4: Optional Plugin (if needed)
        └── Koha plugin for RP-GPS sync
```

### Branch Naming Rationale

**`claude/koha-integration-exploration`** because:
- Clear purpose: Exploring Koha integration
- Exploratory nature: Not committing to full adoption
- Integration focus: Interoperability, not replacement
- Follows existing naming convention

### Initial Branch Commits

1. **Initial Analysis**:
   - This document (KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md)
   - Koha repository analysis notes
   - Strategic decision documentation

2. **Authority Control Study**:
   - Koha authority architecture analysis
   - RP-GPS authority design document
   - Comparison and adaptation plan

3. **Implementation Plan**:
   - Person entity specification
   - MARC adapter architecture
   - Development roadmap

---

## Part 8: Deliverables & Timeline

### Phase 1: Research (Weeks 1-4) - **CURRENT PHASE**

**Week 1**: ✅ **COMPLETE**
- Clone Koha repository
- Analyze architecture
- Strategic analysis document

**Week 2**:
- Deep dive into authority control code
- Document authority linking patterns
- Design RP-GPS Person entity

**Week 3**:
- Study plugin lifecycle management
- Analyze search architecture
- Document learnings

**Week 4**:
- Create Person entity specification
- Design MARC adapter architecture
- Plan implementation phases

**Deliverables**:
- [x] KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md
- [ ] KOHA_AUTHORITY_ARCHITECTURE_STUDY.md
- [ ] RP_GPS_PERSON_ENTITY_DESIGN.md
- [ ] MARC_ADAPTER_SPECIFICATION.md

### Phase 2: Authority System (Weeks 5-12)

**Implementation**:
- Person entity (canonical individuals)
- Variant name system
- Authority linking from sources
- Hierarchical relationships
- Person-Person relationships

**Testing**:
- Unit tests for Person entity
- Integration tests for authority linking
- Relationship graph tests

**Deliverables**:
- Person entity implementation
- Authority linking system
- Test suite

### Phase 3: MARC Adapter (Weeks 13-16)

**Implementation**:
- MARC21 record builder
- Person → MARC authority converter
- Source → MARC biblio converter
- MARC importer for authorities

**Testing**:
- Test with real Koha installation
- Round-trip testing (export/import)
- Validation against MARC standards

**Deliverables**:
- MARC exporter module
- MARC importer module
- Integration tests
- Documentation

### Phase 4: Optional Future

**If Demand Exists**:
- Koha plugin development
- Bidirectional sync
- Deployment to library environments

---

## Part 9: Risk Assessment

### Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| **Koha complexity overwhelming** | Medium | Medium | Focus on specific features (authorities), ignore rest |
| **MARC learning curve** | High | Low | Use existing MARC libraries, focus on export only |
| **Sync complexity (if bidirectional)** | High | High | Start with one-way export, add import carefully |
| **RP-GPS architecture incompatibility** | Low | Medium | Design adapters, don't force fit |

### Strategic Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| **Scope creep** | Medium | High | Stick to authority control + MARC export only |
| **Wasted effort if Koha not adopted** | Low | Low | Learnings valuable regardless |
| **User confusion (two systems)** | Low | Medium | Clear documentation, optional integration |
| **Community fragmentation** | Low | Low | Koha integration is optional add-on |

### Mitigation Strategies

1. **Limit Scope**: Focus ONLY on authority control and MARC export
2. **Keep RP-GPS Independent**: Koha integration is optional
3. **Document Learnings**: Even if not directly adopted, architectural patterns valuable
4. **Prototype Early**: Test MARC export with real Koha in Week 4
5. **User Validation**: Get feedback from genealogists early

---

## Part 10: Success Criteria

### Phase 1 Success (Research)

- [ ] Comprehensive understanding of Koha authority control
- [ ] Clear design for RP-GPS Person entity
- [ ] Documented architectural learnings
- [ ] Decision on whether to proceed with MARC adapter

### Phase 2 Success (Authority System)

- [ ] Person entity implemented and tested
- [ ] Variant name handling working
- [ ] Authority linking from sources functional
- [ ] Hierarchical relationships supported
- [ ] Test coverage >80%

### Phase 3 Success (MARC Adapter)

- [ ] Can export Person → MARC authority
- [ ] Can export Source → MARC biblio
- [ ] Can import MARC authority → Person
- [ ] Round-trip test passes
- [ ] Works with real Koha installation

### Overall Success

**RP-GPS gains**:
- ✅ Mature authority control patterns
- ✅ MARC interoperability
- ✅ Optional Koha integration
- ✅ Better plugin system
- ✅ Enhanced search capabilities

**Without**:
- ❌ Dependency on Koha
- ❌ Abandoning existing work
- ❌ Forcing library-centric workflow

---

## Conclusion

Koha is a **surprisingly relevant** discovery for genealogical research management. While not designed for genealogy, its **authority control system**, **source-centric design**, and **mature architecture** offer valuable lessons and integration opportunities.

### Final Recommendation

**Create `claude/koha-integration-exploration` branch** to:

1. ✅ **Study** Koha's authority control architecture
2. ✅ **Enhance** RP-GPS with authority patterns
3. ✅ **Build** MARC adapter for interoperability
4. ⏸️ **Optionally** develop Koha plugin if demand exists

**This approach**:
- Preserves ResearchProcess-GPS independence
- Learns from 25 years of proven patterns
- Creates institutional integration path
- Serves both individual researchers AND libraries
- Low risk, high value
- Builds on existing RP-GPS work

### Next Immediate Step

**Create the new branch and begin Phase 1 research**:
```bash
git checkout -b claude/koha-integration-exploration
git add KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md
git commit -m "docs: Add comprehensive Koha strategic analysis for genealogy integration"
git push -u origin claude/koha-integration-exploration
```

Then proceed with Week 2 tasks: Deep dive into Koha authority control code.

---

**Document Status**: Strategic analysis complete, ready for decision
**Recommendation Confidence**: High - solid architectural alignment with manageable scope
**Next Action**: Create branch and begin authority control study

---

**Key Insight**: Koha isn't a replacement for ResearchProcess-GPS, but its authority control architecture is **exactly** what genealogical research needs for managing canonical person/place records. Learning from 25 years of library science best practices will make RP-GPS stronger.
