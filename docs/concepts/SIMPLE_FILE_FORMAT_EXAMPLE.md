# Simple File Format Example

## What's Actually in the Git Repository

The beauty is that it's just **plain text files** that both humans and programs can read:

## Repository Structure

```
my-smith-family-research/
├── README.md                 # Project overview (human readable)
├── theories/
│   ├── main/                # Main theory
│   │   ├── theory.yaml      # Theory metadata
│   │   └── README.md        # Theory description
│   ├── john-died-1853/      # Alternative theory
│   │   ├── theory.yaml
│   │   └── README.md
│   └── two-john-smiths/     # Another theory
│       ├── theory.yaml
│       └── README.md
├── identities/              # All the people
│   ├── john-smith-ohio.yaml
│   ├── mary-jones.yaml
│   └── unknown-parent-1.yaml
├── evidence/                # All source materials
│   ├── documents/
│   │   ├── 1850-census.yaml
│   │   └── 1850-census.jpg
│   ├── photos/
│   │   └── gravestone-john.jpg
│   └── analysis/
│       └── dna-matches.yaml
├── events/                  # Births, deaths, etc.
│   ├── birth-john-1810.yaml
│   └── death-john-1878.yaml
├── places/                  # Locations
│   └── clark-county-ohio.yaml
└── research-log/           # Automatic diary
    └── 2024-03-15.md
```

## Example Files

### Identity File: `identities/john-smith-ohio.yaml`

```yaml
# This is John Smith of Ohio
# File: identities/john-smith-ohio.yaml

id: "person_001"
type: "Identity"

names:
  - given: "John"
    surname: "Smith"
    source: "1850-census"
    confidence: "high"
    
  - given: "Johann"
    surname: "Schmidt"
    source: "church-record-1812"
    confidence: "possible"
    note: "Might be German origin"

birth:
  date: "about 1810"
  place: "Clark County, Ohio"
  confidence: "calculated from age"
  
death:
  date: "15 Mar 1878"
  place: "Springfield, Ohio"
  source: "death-certificate"
  confidence: "primary source"

research_notes: |
  This John is definitely in the 1850 census.
  Might be the Johann Schmidt in German church records.
  DNA matches suggest German ancestry.

theories_present_in:
  - "main"
  - "john-died-1853"  # But as different person
```

### Evidence File: `evidence/documents/1850-census.yaml`

```yaml
# 1850 Federal Census
# File: evidence/documents/1850-census.yaml

id: "source_001"
type: "Evidence"

citation: "1850 U.S. Federal Census, Clark County, Ohio, Springfield, page 42, dwelling 310, family 298, John Smith household"

repository: "FamilySearch.org"
accessed: "2024-03-15"
image_file: "1850-census.jpg"

extracted_facts:
  - person: "john-smith-ohio"
    fact: "age"
    value: "40"
    confidence: "as recorded"
    
  - person: "john-smith-ohio"
    fact: "occupation"
    value: "Farmer"
    
  - person: "mary-jones"
    fact: "age"
    value: "38"
    relationship_to_head: "wife"

quality:
  readability: "good"
  completeness: "full page"
```

### Theory File: `theories/john-died-1853/theory.yaml`

```yaml
# Alternative Theory: John Died in 1853
# File: theories/john-died-1853/theory.yaml

id: "theory_002"
name: "John Smith died 1853"
status: "exploring"
confidence: 65

hypothesis: |
  The death record from 1853 is actually our John Smith,
  not a different person. This would mean...

key_assumptions:
  - "1853 death record is our John"
  - "1860 census John is different person"
  - "Property transfer in 1854 supports early death"

problems_solved:
  - "Explains property transfer timing"
  - "Resolves conflicting census ages"

problems_created:
  - "Who is 1860 census John?"
  - "Children's ages don't align"

next_research:
  - "Check probate records 1853-1854"
  - "Look for John Smith Jr."
```

## The Beautiful Simplicity

### For Users

1. **It's just files and folders** - Open in any text editor
2. **Self-documenting** - Comments explain everything
3. **No database** - Can't corrupt, always readable
4. **Works offline** - It's just files on your computer

### For Developers

1. **Parse with any language** - YAML/JSON/Markdown
2. **Git handles versioning** - No custom version system
3. **GitHub provides** - Backup, sharing, webhooks, API
4. **Extend easily** - Just add more files/fields

## How the App Makes It Simple

```yaml
WhatUserSees:
  action: "Clicks on John Smith"
  display: "Nice form with all his information"
  
  action: "Edits birth date"
  result: "Updates the YAML file"
  
  action: "Drops in census image"
  result: "Copies to evidence/documents/, creates YAML"
  
  action: "Clicks Save"
  result: "Git commits all changes"
  
  action: "Clicks Share"
  result: "Pushes to GitHub, sends link"
```

## Migration Path

```yaml
ImportFromGEDCOM:
  gedcom_person: "0 @I1@ INDI"
  becomes_file: "identities/person-i1.yaml"
  
  gedcom_family: "0 @F1@ FAM"
  becomes_relationships: "In person files"
  
  gedcom_source: "0 @S1@ SOUR"
  becomes_file: "evidence/sources/source-s1.yaml"
```

## Why This Is Revolutionary

1. **Your data is yours** - Just files you can read
2. **Use any tool** - Text editor, Excel, whatever
3. **Version control built in** - Via Git
4. **Share naturally** - Email files, use GitHub, Dropbox
5. **Future proof** - Plain text lasts forever

No database to corrupt. No proprietary format. No lock-in. Just your research in simple files that Git manages behind the scenes.