#!/usr/bin/env python3
"""
Deep analysis of GRAMPS to ResearchProcess-GPS mapping
"""

import xml.etree.ElementTree as ET
from collections import defaultdict
import json

# GRAMPS to ResearchProcess-GPS mapping
MAPPING = {
    # Direct mappings
    'person': 'IdentityPersona',
    'family': 'Theory + Relationships',
    'event': 'Evidence (not Fact!)',
    'source': 'Source',
    'citation': 'Citation',
    'place': 'Location',
    'repository': 'Source (type=Repository)',
    
    # Embedded/partial mappings
    'note': 'notes field in entities',
    'name': 'part of IdentityPersona',
    'gender': 'part of IdentityPersona (maybe)',
    'address': 'partial in Location/Contact info',
    'url': 'NOT MAPPED - no URL support',
    'tag': 'tags field (partial support)',
    
    # Relationships
    'childof': 'Relationship (parent-child)',
    'parentin': 'Relationship (parent-child)',
    'personref': 'NOT MAPPED - person associations',
    'eventref': 'Evidence reference',
    'citationref': 'Citation reference',
    'noteref': 'Note reference',
    'sourceref': 'Source reference',
    
    # Attributes - THE BIG MISSING PIECE
    'attribute': 'NOT MAPPED - generic attributes',
    
    # Special/religious
    'lds_ord': 'NOT MAPPED - LDS ordinances',
    
    # User interface
    'bookmark': 'NOT MAPPED - UI feature',
    'namemap': 'NOT MAPPED - name standardization',
    
    # Media
    'object': 'NOT MAPPED - media/files',
    'objref': 'NOT MAPPED - media references',
}

def analyze_attributes_in_detail():
    """Analyze how GRAMPS uses attributes throughout"""
    
    print("\n" + "="*60)
    print("GRAMPS ATTRIBUTE SYSTEM ANALYSIS")
    print("="*60)
    
    print("\nGRAMPS uses <attribute> elements for flexible key-value data:")
    print("- Can be attached to: person, family, event, source, media")
    print("- Has 'type' (the key) and 'value' (the value)")
    print("- Can have citations to prove the attribute")
    print("- Can have notes for additional context")
    
    print("\nCommon attribute types in GRAMPS:")
    attributes = [
        ("Person attributes", [
            "Occupation", "Religion", "Education", "Military Service",
            "Physical Description", "Medical", "SSN", "AFN", "Caste",
            "National ID", "Identity", "Nobility Title"
        ]),
        ("Event attributes", [
            "Witness", "Celebrant", "Informant", "Age", "Agency",
            "Cause", "Description"
        ]),
        ("Family attributes", [
            "Number of Children", "Family Nick Name", "Relationship Type"
        ]),
        ("Source attributes", [
            "Call Number", "Microfilm", "Film Number"
        ])
    ]
    
    for category, attrs in attributes:
        print(f"\n{category}:")
        for attr in attrs:
            print(f"  - {attr}")
    
    print("\n" + "-"*60)
    print("ResearchProcess-GPS ATTRIBUTE PROBLEM:")
    print("-"*60)
    print("\nOur Layer 1 entities have specific fields but NO generic")
    print("attribute system. This means we lose a LOT of data:")
    print("- Occupation information")
    print("- Religious affiliations")
    print("- Physical descriptions")
    print("- Custom research attributes")
    print("- Any non-standard data fields")
    
    return attributes

def analyze_fact_model():
    """Analyze how our Fact model compares to GRAMPS events + attributes"""
    
    print("\n" + "="*60)
    print("FACT MODEL ANALYSIS")
    print("="*60)
    
    print("\nGRAMPS Event model:")
    print("- Event has a type (Birth, Death, Marriage, etc.)")
    print("- Event has date, place, description, cause")
    print("- Event can have ATTRIBUTES for additional data")
    print("- Events are referenced by persons/families")
    
    print("\nResearchProcess-GPS Fact model:")
    print("- Fact has fact_type (string - flexible)")
    print("- Fact has FactValue (union type)")
    print("- Facts belong to concluded IdentityPersonas")
    print("- NO generic attribute support")
    
    print("\nThe FactValue type in our model:")
    fact_value_types = [
        "Text(String)",
        "Date { date: String, precision: DatePrecision }",
        "Place(LocationReference)",
        "Number(f64)",
        "Boolean(bool)",
        "Duration(String)",
        "Age(String)",
    ]
    
    for fv_type in fact_value_types:
        print(f"  - {fv_type}")
    
    print("\nWhat we're missing:")
    print("- Multi-value facts (e.g., multiple occupations)")
    print("- Fact attributes (e.g., 'Witness' on a birth)")
    print("- Fact-to-fact relationships")
    print("- Complex fact structures")

def analyze_name_complexity():
    """Analyze name handling differences"""
    
    print("\n" + "="*60)
    print("NAME HANDLING ANALYSIS")
    print("="*60)
    
    print("\nGRAMPS Name structure:")
    print("- first: Given name")
    print("- call: What person is called (nickname)")
    print("- surname: Can be multiple with:")
    print("  - prefix (von, van, de)")
    print("  - surname main part")
    print("  - connector (y, and)")
    print("  - derivation type")
    print("- suffix: Jr., III, etc.")
    print("- title: Dr., Rev., etc.")
    print("- nick: Nickname")
    print("- familynick: Family nickname")
    print("- group: For sorting (e.g., 'S' for Smith)")
    print("- date: When name was used")
    print("- Multiple names with types (Birth, Married, Also Known As)")
    
    print("\nResearchProcess-GPS IdentityPersona:")
    print("- primary_name: Single string")
    print("- alternative_names: Vec<String>")
    print("- NO structured name parts")
    print("- NO name use dates")
    print("- NO name types")
    print("- NO sorting support")

def suggest_solutions():
    """Suggest solutions for the mapping problems"""
    
    print("\n" + "="*60)
    print("SUGGESTED SOLUTIONS")
    print("="*60)
    
    print("\n1. ADD GENERIC ATTRIBUTES TO LAYER 1")
    print("   Every entity could have:")
    print("   attributes: HashMap<String, AttributeValue>")
    print("   ")
    print("   AttributeValue could be:")
    print("   - Text(String)")
    print("   - Number(f64)")
    print("   - Date(DateTime)")
    print("   - Reference(EntityId)")
    print("   - List(Vec<AttributeValue>)")
    
    print("\n2. ENHANCE FACT MODEL")
    print("   Facts need:")
    print("   - attributes: HashMap<String, AttributeValue>")
    print("   - participants: Vec<FactParticipant>")
    print("   - fact_attributes for things like 'Witness'")
    
    print("\n3. STRUCTURED NAME TYPE")
    print("   Create a proper Name type:")
    print("   ```rust")
    print("   struct Name {")
    print("       name_type: NameType, // Birth, Married, etc.")
    print("       given: Option<String>,")
    print("       call_name: Option<String>,")
    print("       surnames: Vec<Surname>,")
    print("       suffix: Option<String>,")
    print("       title: Option<String>,")
    print("       valid_from: Option<DateTime>,")
    print("       valid_to: Option<DateTime>,")
    print("   }")
    print("   ```")
    
    print("\n4. PRESERVE UNMAPPED DATA")
    print("   Add to each entity:")
    print("   import_data: Option<serde_json::Value>")
    print("   This preserves ANYTHING we don't understand")

def create_mapping_examples():
    """Show concrete examples of data loss"""
    
    print("\n" + "="*60)
    print("CONCRETE DATA LOSS EXAMPLES")
    print("="*60)
    
    examples = [
        ("Person with occupation", """
<person>
  <name type="Birth Name">
    <first>John</first>
    <surname>Smith</surname>
  </name>
  <attribute type="Occupation" value="Blacksmith">
    <citationref hlink="_c001"/>
    <dateval val="1850-1860"/>
  </attribute>
  <attribute type="Occupation" value="Farmer">
    <dateval val="1860-1875"/>
  </attribute>
</person>
""", "LOST: Both occupations and their dates"),
        
        ("Event with witness", """
<event handle="_e001" id="E001">
  <type>Marriage</type>
  <dateval val="1850-06-15"/>
  <place hlink="_p001"/>
  <attribute type="Witness" value="John Doe"/>
  <attribute type="Witness" value="Jane Smith"/>
  <attribute type="Celebrant" value="Rev. Johnson"/>
</event>
""", "LOST: All witness and celebrant information"),
        
        ("Complex name", """
<name type="Birth Name">
  <first>Johann</first>
  <call>Hans</call>
  <surname prefix="von" derivation="Patronymic">Neumann</surname>
  <nick>The Computer</nick>
</name>
""", "LOST: Call name, surname prefix, nickname"),
        
        ("Person associations", """
<person>
  <personref hlink="_p002" rel="Godfather"/>
  <personref hlink="_p003" rel="Apprentice of"/>
  <personref hlink="_p004" rel="Witness"/>
</person>
""", "LOST: All person-to-person relationships except family"),
    ]
    
    for title, xml, loss in examples:
        print(f"\n{title}:")
        print("GRAMPS XML:")
        print(xml.strip())
        print(f"DATA LOSS: {loss}")

if __name__ == "__main__":
    print("GRAMPS → ResearchProcess-GPS Deep Mapping Analysis")
    
    # Analyze each aspect
    analyze_attributes_in_detail()
    analyze_fact_model()
    analyze_name_complexity()
    
    # Show concrete examples
    create_mapping_examples()
    
    # Suggest solutions
    suggest_solutions()
    
    print("\n" + "="*60)
    print("SUMMARY")
    print("="*60)
    print("\nThe ResearchProcess-GPS Layer 1 model is more research-oriented")
    print("but LOSES significant genealogical data from GRAMPS:")
    print("1. No generic attribute system (biggest loss)")
    print("2. No structured names (loss of detail)")
    print("3. No person associations beyond family")
    print("4. No media/object support")
    print("5. No witness/participant roles on events")
    print("\nWe need to enhance Layer 1 to handle real genealogical data!")