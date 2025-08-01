#!/usr/bin/env python3
"""
Analyze GRAMPS XML structure to understand what attributes and data 
we might be missing in the ResearchProcess-GPS data model.
"""

import xml.etree.ElementTree as ET
from collections import defaultdict
import json

class GrampsAnalyzer:
    def __init__(self):
        self.elements = defaultdict(lambda: {
            'count': 0,
            'attributes': defaultdict(set),
            'children': defaultdict(int),
            'text_content': False,
            'examples': []
        })
        
    def analyze_element(self, elem, parent_path=""):
        """Recursively analyze XML elements"""
        path = f"{parent_path}/{elem.tag}" if parent_path else elem.tag
        
        # Track element
        elem_info = self.elements[elem.tag]
        elem_info['count'] += 1
        
        # Track attributes
        for attr, value in elem.attrib.items():
            elem_info['attributes'][attr].add(value[:50] if len(value) > 50 else value)
            
        # Track text content
        if elem.text and elem.text.strip():
            elem_info['text_content'] = True
            if len(elem_info['examples']) < 3:
                elem_info['examples'].append(elem.text.strip()[:100])
                
        # Track children
        for child in elem:
            elem_info['children'][child.tag] += 1
            self.analyze_element(child, path)
            
    def analyze_file(self, filepath):
        """Analyze a GRAMPS XML file"""
        tree = ET.parse(filepath)
        root = tree.getroot()
        self.analyze_element(root)
        
    def print_report(self):
        """Print analysis report"""
        print("GRAMPS XML Structure Analysis")
        print("=" * 50)
        
        # Sort elements by frequency
        sorted_elements = sorted(self.elements.items(), key=lambda x: x[1]['count'], reverse=True)
        
        for tag, info in sorted_elements:
            print(f"\n{tag} (count: {info['count']})")
            print("-" * 30)
            
            # Attributes
            if info['attributes']:
                print("  Attributes:")
                for attr, values in info['attributes'].items():
                    sample_values = list(values)[:3]
                    print(f"    - {attr}: {sample_values}")
                    
            # Children
            if info['children']:
                print("  Children:")
                for child, count in sorted(info['children'].items(), key=lambda x: x[1], reverse=True)[:5]:
                    print(f"    - {child}: {count}")
                    
            # Text examples
            if info['text_content'] and info['examples']:
                print("  Text examples:")
                for ex in info['examples']:
                    print(f"    - {ex}")
                    
    def compare_with_rp_model(self):
        """Compare GRAMPS elements with ResearchProcess-GPS model"""
        print("\n\nComparison with ResearchProcess-GPS Model")
        print("=" * 50)
        
        # GRAMPS to RP mapping
        mapping = {
            'person': 'IdentityPersona',
            'family': 'Theory + Relationships',
            'event': 'Evidence (not Fact)',
            'source': 'Source',
            'citation': 'Citation',
            'place': 'Location',
            'note': '(embedded in entities)',
            'repository': 'Source (type=Repository)',
            'object': '(Media - not implemented)',
            'tag': '(Tags - partial support)',
            'attribute': '??? - NOT MAPPED',
            'address': '??? - PARTIAL in Location',
            'url': '??? - NOT MAPPED',
            'lds_ord': '??? - NOT MAPPED',
            'childof': 'Relationship',
            'parentin': 'Relationship',
            'personref': '??? - NOT MAPPED',
            'name': '(part of IdentityPersona)',
            'surname': '(part of name)',
            'placeref': 'Location reference',
            'eventref': 'Evidence reference'
        }
        
        print("\nGRAMPS Elements → ResearchProcess-GPS Mapping:")
        for gramps_elem in sorted(self.elements.keys()):
            rp_mapping = mapping.get(gramps_elem, '??? - UNKNOWN')
            count = self.elements[gramps_elem]['count']
            print(f"  {gramps_elem:20} → {rp_mapping:30} (count: {count})")
            
        # Identify potentially missing concepts
        print("\n\nPotentially Missing in ResearchProcess-GPS:")
        print("-" * 50)
        
        missing_concepts = [
            ('attribute', 'Generic key-value attributes attached to any entity'),
            ('url', 'Web links associated with entities'),
            ('address', 'Physical addresses (only partial Location support)'),
            ('lds_ord', 'LDS ordinances (religious ceremonies)'),
            ('personref', 'Person-to-person associations (godparent, witness, etc.)'),
            ('object/media', 'Media files and references'),
            ('bookmark', 'User bookmarks/favorites'),
            ('namemap', 'Name mapping/standardization'),
            ('format', 'Name format preferences'),
            ('style', 'Text styling in notes'),
            ('surname prefix/connector', 'Complex surname parts'),
            ('call name', 'Preferred name to be called'),
            ('group', 'Name grouping for sorting'),
            ('date modifiers', 'before/after/about date qualifiers'),
            ('date quality', 'estimated/calculated indicators'),
            ('confidence', 'Numerical confidence on citations'),
            ('privacy', 'Privacy flags on any element'),
            ('change', 'Last change timestamp on elements')
        ]
        
        for concept, description in missing_concepts:
            print(f"  - {concept}: {description}")

# Create example GRAMPS XML for testing
def create_test_gramps_xml():
    """Create a minimal GRAMPS XML example for testing"""
    xml_content = '''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE database PUBLIC "-//GRAMPS//DTD GRAMPS XML V1.7.2//EN"
"http://gramps-project.org/xml/1.7.2/grampsxml.dtd">
<database xmlns="http://gramps-project.org/xml/1.7.2/">
  <header>
    <created date="2024-01-15" version="5.1.5"/>
    <researcher>
      <resname>Test Researcher</resname>
    </researcher>
  </header>
  
  <people>
    <person handle="_abc123" id="I0001" change="1234567890" priv="0">
      <gender>M</gender>
      <name type="Birth Name">
        <first>John</first>
        <surname>Smith</surname>
        <suffix>Jr.</suffix>
        <call>Johnny</call>
      </name>
      <name type="Also Known As" alt="1">
        <first>Johannes</first>
        <surname>Schmidt</surname>
      </name>
      <eventref hlink="_e001" role="Primary"/>
      <eventref hlink="_e002" role="Primary"/>
      <attribute type="Occupation" value="Blacksmith">
        <citationref hlink="_c001"/>
      </attribute>
      <attribute type="Religion" value="Lutheran"/>
      <url type="Email" href="john@example.com" description="Personal email"/>
      <address>
        <street>123 Main St</street>
        <city>Boston</city>
        <state>MA</state>
        <country>USA</country>
        <postal>02101</postal>
        <dateval val="1850-01-01" type="about"/>
      </address>
      <noteref hlink="_n001"/>
      <citationref hlink="_c001"/>
      <childof hlink="_f001"/>
      <personref hlink="_p002" rel="Godfather"/>
      <tagref hlink="_t001"/>
    </person>
    
    <person handle="_p002" id="I0002" change="1234567890">
      <gender>M</gender>
      <name type="Birth Name">
        <first>William</first>
        <surname>Johnson</surname>
      </name>
    </person>
  </people>
  
  <families>
    <family handle="_f001" id="F0001" change="1234567890">
      <rel type="Married"/>
      <father hlink="_p003"/>
      <mother hlink="_p004"/>
      <eventref hlink="_e003" role="Family"/>
      <childref hlink="_abc123"/>
      <attribute type="Number of Children" value="5"/>
      <noteref hlink="_n002"/>
    </family>
  </families>
  
  <events>
    <event handle="_e001" id="E0001" change="1234567890">
      <type>Birth</type>
      <dateval val="1825-03-15" type="exact" quality="primary"/>
      <place hlink="_pl001"/>
      <description>Born at family farm</description>
      <noteref hlink="_n003"/>
      <citationref hlink="_c001"/>
      <attribute type="Witness" value="Mary Johnson"/>
    </event>
    
    <event handle="_e002" id="E0002" change="1234567890">
      <type>Death</type>
      <daterange start="1875-01-01" stop="1875-12-31" quality="estimated"/>
      <place hlink="_pl002"/>
      <cause>Pneumonia</cause>
    </event>
  </events>
  
  <places>
    <place handle="_pl001" id="P0001" change="1234567890">
      <ptitle>Boston, Suffolk, Massachusetts, USA</ptitle>
      <pname value="Boston"/>
      <type>City</type>
      <coord long="-71.0589" lat="42.3601"/>
      <placeref hlink="_pl003"/>
      <noteref hlink="_n004"/>
    </place>
  </places>
  
  <sources>
    <source handle="_s001" id="S0001" change="1234567890">
      <stitle>1850 United States Federal Census</stitle>
      <sauthor>U.S. Census Bureau</sauthor>
      <spubinfo>National Archives</spubinfo>
      <noteref hlink="_n005"/>
      <reporef hlink="_r001" medium="Microfilm"/>
    </source>
  </sources>
  
  <citations>
    <citation handle="_c001" id="C0001" change="1234567890">
      <dateval val="2024-01-15"/>
      <page>Page 42, Line 15, Family 312</page>
      <confidence>4</confidence>
      <noteref hlink="_n006"/>
      <sourceref hlink="_s001"/>
    </citation>
  </citations>
  
  <repositories>
    <repository handle="_r001" id="R0001" change="1234567890">
      <rname>National Archives</rname>
      <type>Archive</type>
      <address>
        <city>Washington</city>
        <state>DC</state>
        <country>USA</country>
      </address>
      <url type="Web Home" href="https://archives.gov"/>
    </repository>
  </repositories>
  
  <notes>
    <note handle="_n001" id="N0001" change="1234567890" type="Person Note" priv="0">
      <text>John Smith was known for his exceptional blacksmith work.</text>
      <style name="bold" value="John Smith">
        <range start="0" end="10"/>
      </style>
      <tagref hlink="_t002"/>
    </note>
  </notes>
  
  <tags>
    <tag handle="_t001" name="Verified" color="#00ff00" priority="1" change="1234567890"/>
    <tag handle="_t002" name="Research Needed" color="#ff0000" priority="2" change="1234567890"/>
  </tags>
  
  <bookmarks>
    <bookmark target="person" hlink="_abc123"/>
    <bookmark target="source" hlink="_s001"/>
  </bookmarks>
  
  <namemaps>
    <map type="surname" key="Smith" value="Smith"/>
    <map type="surname" key="Smithe" value="Smith"/>
    <map type="surname" key="Smythe" value="Smith"/>
  </namemaps>
  
</database>'''
    
    with open('/tmp/test_gramps.xml', 'w') as f:
        f.write(xml_content)
    
    return '/tmp/test_gramps.xml'

if __name__ == "__main__":
    # Create test file
    test_file = create_test_gramps_xml()
    
    # Analyze
    analyzer = GrampsAnalyzer()
    analyzer.analyze_file(test_file)
    analyzer.print_report()
    analyzer.compare_with_rp_model()