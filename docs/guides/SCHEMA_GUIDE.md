# Schema Definition Guide

## Introduction

Schemas define the structure, validation rules, and state transitions for your data model. ResearchProcess-GPS uses YAML-based schemas that compile to the universal meta-model.

## Basic Schema Structure

```yaml
schema:
  id: unique-schema-id          # Required: Unique identifier
  version: "1.0.0"               # Required: Semantic version
  name: Human Readable Name      # Required: Display name
  description: Longer description of this schema

entity_types:
  # Entity type definitions

relationship_types:
  # Relationship type definitions

property_definitions:
  # Reusable property definitions (optional)

state_machines:
  # State transition rules (optional)
```

## Entity Types

### Minimal Entity

```yaml
entity_types:
  SimplePerson:
    description: A minimal person entity
    required_properties:
      - name
    properties:
      name:
        type: Text
```

### Complete Entity

```yaml
entity_types:
  Person:
    description: An individual person with comprehensive details

    # Required properties (validation will fail if missing)
    required_properties:
      - name
      - birth_date

    # Optional properties (validation passes if missing)
    optional_properties:
      - death_date
      - occupation
      - notes

    # Property definitions
    properties:
      name:
        type: Text
        description: Full name of the person
        min_length: 1
        max_length: 200
        validation:
          - rule: Required

      birth_date:
        type: Temporal
        description: Date of birth
        validation:
          - rule: Required

      death_date:
        type: Temporal
        description: Date of death (if deceased)

      occupation:
        type: Text
        description: Primary occupation
        max_length: 100

      notes:
        type: Text
        description: Additional notes

    # Valid states for this entity
    valid_states:
      - Draft
      - Research
      - Verified
      - Published

    # Initial state when created
    default_state: Draft

    # Optional constraints
    constraints:
      - type: DateRange
        before: death_date
        after: birth_date
        message: "Death must be after birth"
```

## Property Types

### Text

```yaml
properties:
  name:
    type: Text
    description: Simple text value
    min_length: 1          # Optional: minimum length
    max_length: 100        # Optional: maximum length
    pattern: "^[A-Za-z ]+$"  # Optional: regex pattern
    default: "Unknown"     # Optional: default value

  email:
    type: Text
    pattern: "^[^@]+@[^@]+\\.[^@]+$"
    description: Email address
```

### Integer

```yaml
properties:
  age:
    type: Integer
    description: Age in years
    min: 0              # Optional: minimum value
    max: 150            # Optional: maximum value
    default: 0

  count:
    type: Integer
    description: Number of children
```

### Float

```yaml
properties:
  latitude:
    type: Float
    description: Geographic latitude
    min: -90.0
    max: 90.0

  confidence_score:
    type: Float
    min: 0.0
    max: 1.0
```

### Boolean

```yaml
properties:
  verified:
    type: Boolean
    description: Has this been verified?
    default: false

  is_private:
    type: Boolean
    default: true
```

### Enum

```yaml
properties:
  sex:
    type: Enum
    values: [M, F, X, U]
    description: Sex assigned at birth
    default: U

  status:
    type: Enum
    values:
      - Draft
      - In Progress
      - Complete
      - Archived
    default: Draft
```

### Temporal

Temporal values support multiple calendar systems and date formats:

```yaml
properties:
  birth_date:
    type: Temporal
    description: Date of birth
    # Supports:
    # - Exact dates: 1850-03-15
    # - Approximate: "about 1850"
    # - Ranges: "between 1850 and 1855"
    # - Before/After: "before 1860"
    # - Multiple calendars: Julian, Gregorian, Hebrew, etc.

  event_date:
    type: Temporal
    description: When event occurred
    required_precision: Year  # Year, Month, Day, Time
```

### Spatial

Spatial values represent locations:

```yaml
properties:
  birth_place:
    type: Spatial
    description: Place of birth
    # Supports:
    # - Text: "New York, NY, USA"
    # - Coordinates: (40.7128, -74.0060)
    # - Hierarchical: Country > State > City
    # - Multiple coordinate systems

  residence:
    type: Spatial
    description: Current residence
    required_components: [City, State]
```

### Reference

References link to other entities:

```yaml
properties:
  father:
    type: Reference
    description: Reference to father entity
    target_types: [Person]  # Optional: restrict to types

  sources:
    type: Collection
    item_type: Reference
    description: Source citations
    target_types: [Source]
```

### Collection

Collections hold multiple values:

```yaml
properties:
  children:
    type: Collection
    item_type: Reference
    description: Child entities
    min_items: 0
    max_items: 20

  alternate_names:
    type: Collection
    item_type: Text
    description: Other names used

  attributes:
    type: Collection
    item_type: Map
    description: Custom attributes
```

### Map

Maps represent structured data:

```yaml
properties:
  address:
    type: Map
    description: Mailing address
    properties:
      street:
        type: Text
      city:
        type: Text
        validation:
          - rule: Required
      state:
        type: Text
      postal_code:
        type: Text
        pattern: "^\\d{5}(-\\d{4})?$"

  name_parts:
    type: Map
    properties:
      given:
        type: Text
      surname:
        type: Text
      prefix:
        type: Text
      suffix:
        type: Text
```

### Any

Accepts any value type (use sparingly):

```yaml
properties:
  custom_data:
    type: Any
    description: Flexible custom data
```

## Relationship Types

### Basic Relationship

```yaml
relationship_types:
  Spouse:
    description: Marriage or partnership
    participants:
      - role: spouse
        entity_types: [Person]
        required: true
        multiple: true  # Allows multiple with this role
    min_participants: 2
    max_participants: 2
```

### Complex Relationship

```yaml
relationship_types:
  Parent-Child:
    description: Biological or adoptive parent-child relationship

    participants:
      - role: parent
        entity_types: [Person]
        required: true
        multiple: true      # Can have multiple parents
      - role: child
        entity_types: [Person]
        required: true
        multiple: false     # Only one child per relationship

    # Properties on the relationship itself
    optional_properties:
      - relationship_type
      - adoption_date
      - notes

    properties:
      relationship_type:
        type: Enum
        values: [Birth, Adopted, Foster, Step]
        default: Birth
        description: Type of parent-child relationship

      adoption_date:
        type: Temporal
        description: Date of adoption (if applicable)

      notes:
        type: Text
        description: Additional notes about relationship

    min_participants: 2     # At least parent + child
    max_participants: 3     # Two parents + one child

    constraints:
      - type: RequiredRole
        role: parent
        message: "Must have at least one parent"
      - type: RequiredRole
        role: child
        message: "Must have exactly one child"
```

### N-ary Relationship

```yaml
relationship_types:
  Research-Team:
    description: A research collaboration
    participants:
      - role: lead_researcher
        entity_types: [Person]
        required: true
        multiple: false
      - role: researcher
        entity_types: [Person]
        required: false
        multiple: true
      - role: consultant
        entity_types: [Person]
        required: false
        multiple: true
    min_participants: 1   # Just lead
    max_participants: 20  # Maximum team size
```

## State Machines

State machines define valid state transitions:

```yaml
state_machines:
  person_lifecycle:
    description: Lifecycle of a person entity

    initial_state: Draft

    states:
      - Draft           # Initial research
      - Research        # Active research
      - Verified        # Facts verified
      - Published       # Published in tree
      - Living          # Currently living
      - Deceased        # Confirmed deceased

    transitions:
      Draft:
        - Research
        - Verified
        - Living
      Research:
        - Draft
        - Verified
        - Living
      Verified:
        - Research
        - Published
        - Deceased
      Published:
        - Deceased
      Living:
        - Deceased
      Deceased: []      # Terminal state

    terminal_states:
      - Deceased
```

### State Machine with Actions

```yaml
state_machines:
  document_workflow:
    initial_state: Received

    states:
      - Received
      - Transcribing
      - Review
      - Published
      - Archived

    transitions:
      Received:
        - Transcribing
      Transcribing:
        - Received      # Can go back
        - Review
      Review:
        - Transcribing  # Request changes
        - Published
      Published:
        - Archived
      Archived: []

    # Optional: Actions on transition
    on_transition:
      Received_to_Transcribing:
        - action: AssignTranscriber
        - action: SetStartDate
      Review_to_Published:
        - action: NotifySubmitter
        - action: UpdateIndex

    terminal_states:
      - Archived
```

## Validation Rules

### Built-in Rules

```yaml
properties:
  name:
    type: Text
    validation:
      - rule: Required
      - rule: MinLength
        value: 2
      - rule: MaxLength
        value: 100
      - rule: Pattern
        value: "^[A-Za-z ]+$"

  age:
    type: Integer
    validation:
      - rule: Required
      - rule: Min
        value: 0
      - rule: Max
        value: 150

  email:
    type: Text
    validation:
      - rule: Email

  url:
    type: Text
    validation:
      - rule: URL
```

### Custom Constraints

```yaml
entity_types:
  Person:
    properties:
      birth_date:
        type: Temporal
      death_date:
        type: Temporal

    constraints:
      # Death must be after birth
      - type: DateRange
        before: death_date
        after: birth_date
        message: "Death date must be after birth date"

      # Age at death must be reasonable
      - type: Expression
        expression: "death_date - birth_date < 150 years"
        message: "Age at death seems unreasonable"

      # Mutual exclusion
      - type: MutuallyExclusive
        properties: [death_date, is_living]
        message: "Cannot have death date if marked as living"
```

## Reusable Definitions

### Property Definitions

```yaml
property_definitions:
  # Define once, use many times
  PersonName:
    type: Map
    properties:
      given:
        type: Text
        validation:
          - rule: Required
      middle:
        type: Text
      surname:
        type: Text
        validation:
          - rule: Required
      prefix:
        type: Text
      suffix:
        type: Text

  DatePlace:
    type: Map
    properties:
      date:
        type: Temporal
      place:
        type: Spatial

entity_types:
  Person:
    properties:
      primary_name:
        $ref: "#/property_definitions/PersonName"

      birth:
        $ref: "#/property_definitions/DatePlace"

      death:
        $ref: "#/property_definitions/DatePlace"
```

## Schema Composition

### Extending Schemas

```yaml
schema:
  id: extended-gedcom
  extends: gedcom-7    # Extends another schema
  version: "1.0.0"
  name: Extended GEDCOM

entity_types:
  # Add new entity type
  DNA-Result:
    description: DNA test results
    properties:
      test_type:
        type: Enum
        values: [Autosomal, Y-DNA, mtDNA]
      matches:
        type: Collection
        item_type: Reference

  # Extend existing entity type
  Individual:
    extends: gedcom-7/Individual
    optional_properties:
      - dna_results
    properties:
      dna_results:
        type: Collection
        item_type: Reference
        target_types: [DNA-Result]
```

### Mixing Schemas

```yaml
# You can use multiple schemas in one workspace
schemas:
  - gedcom-7
  - gramps
  - my-custom-schema

# Entities validate against their schema
entity:
  schema: gedcom-7
  type: Individual

# Can create cross-schema relationships
relationship:
  type: SameAs
  participants:
    - entity: gedcom-individual
      role: gedcom_version
    - entity: gramps-person
      role: gramps_version
```

## Complete Example

Here's a complete schema for a research project:

```yaml
schema:
  id: historical-research
  version: "1.0.0"
  name: Historical Research Model
  description: Schema for historical document research

property_definitions:
  Citation:
    type: Map
    properties:
      source_id:
        type: Reference
        target_types: [Source]
      page:
        type: Text
      confidence:
        type: Enum
        values: [Very High, High, Medium, Low, Very Low]
      transcription:
        type: Text

entity_types:
  Document:
    description: Historical document
    required_properties:
      - title
      - date
      - repository
    optional_properties:
      - author
      - document_type
      - language
      - transcription
      - translations
      - images
      - citations
    properties:
      title:
        type: Text
        min_length: 1
        max_length: 500
        validation:
          - rule: Required
      date:
        type: Temporal
        description: Document creation date
      repository:
        type: Reference
        target_types: [Repository]
        description: Where document is held
      author:
        type: Text
        description: Author or creator
      document_type:
        type: Enum
        values: [Letter, Deed, Will, Census, Birth Certificate, Death Certificate, Other]
        default: Other
      language:
        type: Text
        pattern: "^[a-z]{2,3}$"  # ISO 639 code
        default: "en"
      transcription:
        type: Text
        description: Full text transcription
      translations:
        type: Collection
        item_type: Map
        description: Translations to other languages
      images:
        type: Collection
        item_type: Reference
        target_types: [Image]
      citations:
        type: Collection
        item_type:
          $ref: "#/property_definitions/Citation"
    valid_states:
      - Located
      - Acquired
      - Transcribing
      - Transcribed
      - Reviewed
      - Published
    default_state: Located

  Source:
    description: Source of information
    required_properties:
      - title
      - source_type
    properties:
      title:
        type: Text
        validation:
          - rule: Required
      source_type:
        type: Enum
        values: [Primary, Secondary, Tertiary]
      repository:
        type: Reference
        target_types: [Repository]
    valid_states:
      - Unverified
      - Verified
    default_state: Unverified

  Repository:
    description: Archive or library
    required_properties:
      - name
    properties:
      name:
        type: Text
      address:
        type: Spatial
      contact:
        type: Map
        properties:
          email:
            type: Text
            pattern: "^[^@]+@[^@]+\\.[^@]+$"
          phone:
            type: Text
          website:
            type: Text
    valid_states:
      - Active
      - Inactive
    default_state: Active

  Image:
    description: Image of document
    required_properties:
      - file_path
    properties:
      file_path:
        type: Text
      resolution:
        type: Integer
        min: 72
        max: 9600
      file_size:
        type: Integer
        description: Size in bytes
    valid_states:
      - Draft
      - Processed
    default_state: Draft

relationship_types:
  Cites:
    description: Document cites another document
    participants:
      - role: citing
        entity_types: [Document]
        required: true
      - role: cited
        entity_types: [Document]
        required: true
    properties:
      page:
        type: Text
      quote:
        type: Text
    min_participants: 2
    max_participants: 2

  Held-By:
    description: Document held by repository
    participants:
      - role: document
        entity_types: [Document]
        required: true
      - role: repository
        entity_types: [Repository]
        required: true
    properties:
      call_number:
        type: Text
      access_restrictions:
        type: Enum
        values: [Public, Restricted, Private]
        default: Public
    min_participants: 2
    max_participants: 2

state_machines:
  document_lifecycle:
    initial_state: Located
    states:
      - Located
      - Acquired
      - Transcribing
      - Transcribed
      - Reviewed
      - Published
    transitions:
      Located: [Acquired]
      Acquired: [Transcribing]
      Transcribing: [Acquired, Transcribed]
      Transcribed: [Transcribing, Reviewed]
      Reviewed: [Transcribed, Published]
      Published: []
    terminal_states:
      - Published
```

## Best Practices

1. **Start Simple**: Begin with minimal required properties, add complexity as needed
2. **Use References**: Link entities rather than duplicating data
3. **Validate Strictly**: Use validation rules to ensure data quality
4. **Document Everything**: Add descriptions to all types and properties
5. **Version Carefully**: Use semantic versioning and document breaking changes
6. **Test Thoroughly**: Validate your schema with real data
7. **Consider Migration**: Plan for schema evolution and data migration

## Tools

### Schema Validation

```rust
use rp_schema::Schema;

let yaml = std::fs::read_to_string("my-schema.yaml")?;
let schema = Schema::from_yaml(&yaml)?;

// Schema is valid!
```

### Entity Validation

```rust
use rp_schema::Validator;

let validator = Validator::new(&schema);
let result = validator.validate_entity(&entity)?;

if result.has_errors() {
    for error in result.errors {
        eprintln!("Validation error: {}", error);
    }
}
```

## Resources

- [Getting Started](GETTING_STARTED.md)
- [Developer Guide](DEVELOPER_GUIDE.md)
- [Example Schemas](../../schemas/)
- [API Documentation](../api/)

## Next Steps

1. Create your schema YAML file
2. Test with sample data
3. Build an adapter for import/export
4. Share with the community!
