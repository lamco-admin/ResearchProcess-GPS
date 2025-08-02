#!/usr/bin/env python3
"""
Quick test to see how GRAMPS data looks when transformed to ResearchProcess-GPS format
"""

import xml.etree.ElementTree as ET
import json
from datetime import datetime
from uuid import uuid4

class QuickGrampsImporter:
    def __init__(self):
        self.researcher_id = str(uuid4())
        self.gramps_to_rp = {}  # handle -> entity_id mapping

    def import_xml(self, xml_content):
        """Import GRAMPS XML and transform to RP-GPS format"""
        root = ET.fromstring(xml_content)

        # Remove namespace for easier parsing
        for elem in root.iter():
            if '}' in elem.tag:
                elem.tag = elem.tag.split('}')[1]

        result = {
            'identities': [],
            'evidence': [],
            'theories': [],
            'relationships': [],
            'sources': [],
            'citations': [],
            'locations': [],
            'research_questions': [],
            'lost_data': []
        }

        # Process people
        people = root.find('people')
        if people is not None:
            for person in people.findall('person'):
                self.process_person(person, result)

        # Process events
        events = root.find('events')
        if events is not None:
            for event in events.findall('event'):
                self.process_event(event, result)

        # Process families
        families = root.find('families')
        if families is not None:
            for family in families.findall('family'):
                self.process_family(family, result)

        return result

    def process_person(self, person, result):
        """Transform GRAMPS person to IdentityPersona"""
        handle = person.get('handle')
        person_id = person.get('id')

        # Extract primary name
        names = person.findall('name')
        primary_name = "Unknown"
        alt_names = []
        lost_name_data = []

        for i, name in enumerate(names):
            name_type = name.get('type', 'Unknown')
            first = name.findtext('first', '')
            surname = name.findtext('surname', '')
            full_name = f"{first} {surname}".strip()

            if i == 0 or name_type == "Birth Name":
                primary_name = full_name
            else:
                alt_names.append(full_name)

            # Track lost data
            call = name.findtext('call')
            if call:
                lost_name_data.append(f"Call name: {call}")
            nick = name.findtext('nick')
            if nick:
                lost_name_data.append(f"Nickname: {nick}")
            suffix = name.findtext('suffix')
            if suffix:
                lost_name_data.append(f"Suffix: {suffix}")

        # Create IdentityPersona
        identity_id = str(uuid4())
        self.gramps_to_rp[handle] = identity_id

        identity = {
            'id': identity_id,
            'entity_type': 'IdentityPersona',
            'state': 'Hypothesis',  # Not Concluded!
            'primary_name': primary_name,
            'alternative_names': alt_names,
            'identity_type': 'Named',
            'notes': f"Imported from GRAMPS {person_id}. Requires verification.",
            'evidence_references': [],
            'created_by': self.researcher_id,
            'gramps_handle': handle
        }

        result['identities'].append(identity)

        # Process attributes - ALL WILL BE LOST!
        attributes = person.findall('attribute')
        for attr in attributes:
            attr_type = attr.get('type')
            attr_value = attr.get('value')
            result['lost_data'].append({
                'entity': f"Person {primary_name}",
                'data_type': 'attribute',
                'lost': f"{attr_type}: {attr_value}"
            })

        # Process addresses - WILL BE LOST!
        addresses = person.findall('address')
        for addr in addresses:
            street = addr.findtext('street', '')
            city = addr.findtext('city', '')
            result['lost_data'].append({
                'entity': f"Person {primary_name}",
                'data_type': 'address',
                'lost': f"{street}, {city}"
            })

        # Process person references - WILL BE LOST!
        personrefs = person.findall('personref')
        for pref in personrefs:
            rel = pref.get('rel')
            result['lost_data'].append({
                'entity': f"Person {primary_name}",
                'data_type': 'person_association',
                'lost': f"Relationship: {rel}"
            })

        # Generate research question
        result['research_questions'].append({
            'id': str(uuid4()),
            'entity_type': 'Theory',
            'question': f"Verify identity of {primary_name}",
            'hypothesis': f"Confirm {primary_name} from GRAMPS import is correctly identified",
            'state': 'Proposed',
            'supporting_entities': [identity_id]
        })

        if lost_name_data:
            result['lost_data'].append({
                'entity': f"Person {primary_name}",
                'data_type': 'name_details',
                'lost': '; '.join(lost_name_data)
            })

    def process_event(self, event, result):
        """Transform GRAMPS event to Evidence"""
        handle = event.get('handle')
        event_id = event.get('id')
        event_type = event.findtext('type', 'Unknown')

        evidence_id = str(uuid4())
        self.gramps_to_rp[handle] = evidence_id

        # Extract date
        date_str = "Unknown date"
        dateval = event.find('dateval')
        if dateval is not None:
            date_str = dateval.get('val', 'Unknown date')
            date_type = dateval.get('type', 'exact')
            if date_type != 'exact':
                date_str = f"{date_type} {date_str}"

        evidence = {
            'id': evidence_id,
            'entity_type': 'Evidence',
            'evidence_type': 'Extracted',
            'description': f"GRAMPS Event: {event_type} on {date_str}",
            'extracted_facts': [{
                'fact_type': event_type,
                'value': {'Date': {'date': date_str, 'precision': 'Day'}},
                'confidence': 0.0  # Unverified!
            }],
            'notes': f"Imported from GRAMPS event {event_id}. Requires source verification.",
            'gramps_handle': handle
        }

        result['evidence'].append(evidence)

        # Process event attributes - WILL BE LOST!
        attributes = event.findall('attribute')
        for attr in attributes:
            attr_type = attr.get('type')
            attr_value = attr.get('value')
            result['lost_data'].append({
                'entity': f"Event {event_type}",
                'data_type': 'event_attribute',
                'lost': f"{attr_type}: {attr_value}"
            })

    def process_family(self, family, result):
        """Transform GRAMPS family to Theory + Relationships"""
        handle = family.get('handle')
        family_id = family.get('id')

        # Create family theory
        theory_id = str(uuid4())
        theory = {
            'id': theory_id,
            'entity_type': 'Theory',
            'question': f"Verify family unit {family_id}",
            'hypothesis': "These individuals formed a family unit as documented in GRAMPS",
            'state': 'Proposed',
            'supporting_entities': [],
            'notes': f"Imported from GRAMPS family {family_id}"
        }

        # Process relationships
        father_handle = family.find('father')
        mother_handle = family.find('mother')

        if father_handle is not None and mother_handle is not None:
            father_id = self.gramps_to_rp.get(father_handle.get('hlink'))
            mother_id = self.gramps_to_rp.get(mother_handle.get('hlink'))

            if father_id and mother_id:
                rel_type = family.findtext('rel', {'type': 'Unknown'})
                relationship = {
                    'id': str(uuid4()),
                    'entity_type': 'Relationship',
                    'participant_a': father_id,
                    'participant_b': mother_id,
                    'relationship_type': f"Partnership-{rel_type}",
                    'state': 'Proposed',  # Not Verified!
                    'notes': "Imported from GRAMPS. Requires documentary evidence."
                }
                result['relationships'].append(relationship)

                # Add to theory
                theory['supporting_entities'].extend([father_id, mother_id])

        # Process children
        for childref in family.findall('childref'):
            child_handle = childref.get('hlink')
            child_id = self.gramps_to_rp.get(child_handle)
            if child_id:
                theory['supporting_entities'].append(child_id)

        result['theories'].append(theory)

        # Family attributes - WILL BE LOST!
        attributes = family.findall('attribute')
        for attr in attributes:
            attr_type = attr.get('type')
            attr_value = attr.get('value')
            result['lost_data'].append({
                'entity': f"Family {family_id}",
                'data_type': 'family_attribute',
                'lost': f"{attr_type}: {attr_value}"
            })


# Test with our example
if __name__ == "__main__":
    xml_content = '''<?xml version="1.0" encoding="UTF-8"?>
<database xmlns="http://gramps-project.org/xml/1.7.2/">
  <header>
    <created date="2024-01-15" version="5.1.5"/>
  </header>

  <people>
    <person handle="_p001" id="I0001">
      <gender>M</gender>
      <name type="Birth Name">
        <first>John</first>
        <call>Johnny</call>
        <surname prefix="van">Smith</surname>
        <suffix>Jr.</suffix>
      </name>
      <attribute type="Occupation" value="Blacksmith"/>
      <attribute type="Religion" value="Lutheran"/>
      <personref hlink="_p002" rel="Godfather"/>
      <address>
        <street>123 Main St</street>
        <city>Boston</city>
      </address>
    </person>
    <person handle="_p002" id="I0002">
      <gender>F</gender>
      <name type="Birth Name">
        <first>Mary</first>
        <surname>Jones</surname>
      </name>
    </person>
  </people>

  <events>
    <event handle="_e001" id="E0001">
      <type>Marriage</type>
      <dateval val="1850-06-15"/>
      <attribute type="Witness" value="William Johnson"/>
      <attribute type="Witness" value="Sarah Williams"/>
    </event>
  </events>

  <families>
    <family handle="_f001" id="F0001">
      <rel type="Married"/>
      <father hlink="_p001"/>
      <mother hlink="_p002"/>
      <eventref hlink="_e001"/>
      <attribute type="Number of Children" value="5"/>
    </family>
  </families>
</database>'''

    importer = QuickGrampsImporter()
    result = importer.import_xml(xml_content)

    print("GRAMPS → ResearchProcess-GPS Import Results")
    print("="*60)

    print(f"\nEntities created:")
    print(f"- Identities: {len(result['identities'])}")
    print(f"- Evidence: {len(result['evidence'])}")
    print(f"- Theories: {len(result['theories'])}")
    print(f"- Relationships: {len(result['relationships'])}")
    print(f"- Research Questions: {len(result['research_questions'])}")

    print("\nSample Identity:")
    if result['identities']:
        identity = result['identities'][0]
        print(json.dumps(identity, indent=2))

    print("\nSample Evidence:")
    if result['evidence']:
        evidence = result['evidence'][0]
        print(json.dumps(evidence, indent=2))

    print("\nLOST DATA:")
    print("-"*60)
    for lost in result['lost_data']:
        print(f"{lost['entity']} - {lost['data_type']}: {lost['lost']}")

    print(f"\nTotal data loss items: {len(result['lost_data'])}")