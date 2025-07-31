"""
Example: GRAMPS Data Liberation

This demonstrates how GRAMPS's rigid person/family structure gets
liberated into ResearchProcess-GPS's flexible identity/relationship model.
"""

from datetime import datetime
from uuid import uuid4

# Simulated GRAMPS data structures
class GRAMPSPerson:
    def __init__(self, handle, name, birth_date=None, death_date=None):
        self.handle = handle
        self.name = name
        self.birth_date = birth_date
        self.death_date = death_date
        self.event_refs = []
        self.family_list = []  # Families where person is parent
        self.parent_family_list = []  # Families where person is child

class GRAMPSFamily:
    def __init__(self, handle, father_handle=None, mother_handle=None):
        self.handle = handle
        self.father_handle = father_handle
        self.mother_handle = mother_handle
        self.child_handles = []
        self.relationship_type = "Married"

class GRAMPSEvent:
    def __init__(self, handle, event_type, date=None, place=None):
        self.handle = handle
        self.type = event_type
        self.date = date
        self.place = place


def gramps_liberation_example():
    """
    Show how GRAMPS's fixed structure becomes flexible in RGPS
    """
    
    print("=== GRAMPS to ResearchProcess-GPS Liberation Example ===\n")
    
    # Step 1: Simulate GRAMPS data
    print("--- Original GRAMPS Data (Fixed Structure) ---\n")
    
    # Create GRAMPS persons
    john = GRAMPSPerson("P001", "John Smith", 
                       birth_date="1820", death_date="1875")
    mary = GRAMPSPerson("P002", "Mary Jones", 
                       birth_date="1825", death_date="1880")
    william = GRAMPSPerson("P003", "William Smith", birth_date="1845")
    sarah = GRAMPSPerson("P004", "Sarah Smith", birth_date="1847")
    
    # Create GRAMPS family (rigid structure)
    family = GRAMPSFamily("F001", father_handle="P001", mother_handle="P002")
    family.child_handles = ["P003", "P004"]
    
    # Create GRAMPS events (owned by person/family)
    marriage = GRAMPSEvent("E001", "Marriage", date="1844-06-15", 
                          place="Springfield Church")
    census1850 = GRAMPSEvent("E002", "Census", date="1850", 
                            place="Springfield")
    
    print("GRAMPS Person: John Smith")
    print("  - Fixed birth: 1820")
    print("  - Fixed death: 1875")
    print("  - Locked into Family F001 as father")
    print("  - Events 'owned' by person\n")
    
    print("GRAMPS Family F001:")
    print("  - Rigid structure: Father + Mother + Children")
    print("  - Cannot represent complex relationships")
    print("  - Events 'owned' by family\n")
    
    # Step 2: Liberation to RGPS
    print("\n--- After Liberation to ResearchProcess-GPS ---\n")
    
    print("IDENTITY: John Smith")
    print("  ├─ Not locked to any birth/death date")
    print("  ├─ Can exist in multiple theories")
    print("  ├─ Can have multiple personas:")
    print("  │   ├─ Persona A: Born 1820 (based on census)")
    print("  │   ├─ Persona B: Born 1823 (based on death cert)")
    print("  │   └─ Persona C: Two different Johns theory")
    print("  └─ Events float independently\n")
    
    print("RELATIONSHIPS: Multi-party and Flexible")
    print("  ├─ Spousal Relationship")
    print("  │   ├─ Participants: John (spouse), Mary (spouse)")
    print("  │   ├─ Type: Legal marriage")
    print("  │   ├─ Period: 1844-1875")
    print("  │   └─ Can have nuanced types (common law, betrothal, etc.)")
    print("  │")
    print("  ├─ Parent-Child Relationship 1")
    print("  │   ├─ Participants: John (parent), William (child)")
    print("  │   └─ Type: Biological (but could be adoption, step, etc.)")
    print("  │")
    print("  └─ Parent-Child Relationship 2")
    print("      ├─ Participants: John (parent), Sarah (child)")
    print("      └─ Each relationship tracked separately\n")
    
    print("EVENTS: Floating and Reusable")
    print("  ├─ 1850 Census Event")
    print("  │   ├─ Not 'owned' by any person")
    print("  │   ├─ Participants:")
    print("  │   │   ├─ John (head of household)")
    print("  │   │   ├─ Mary (spouse)")
    print("  │   │   ├─ William (child)")
    print("  │   │   └─ Sarah (child)")
    print("  │   └─ Same event can support multiple theories")
    print("  │")
    print("  └─ Marriage Event")
    print("      ├─ Participants: John (groom), Mary (bride)")
    print("      ├─ Not locked to 'family' concept")
    print("      └─ Can be referenced by multiple theories\n")
    
    # Step 3: Show theory flexibility
    print("--- Theory Flexibility Example ---\n")
    
    print("Theory 1: 'Traditional Family'")
    print("  - John and Mary married, had William and Sarah")
    print("  - Standard interpretation from GRAMPS\n")
    
    print("Theory 2: 'William was adopted'")
    print("  - Same identities")
    print("  - William's parent-child relationship type = 'adoption'")
    print("  - Birth parents unknown (new identities)")
    print("  - GRAMPS cannot represent this nuance\n")
    
    print("Theory 3: 'Sarah was Mary's from previous marriage'")
    print("  - Sarah only linked to Mary biologically")
    print("  - John is step-parent")
    print("  - Previous husband identity (hypothetical)")
    print("  - GRAMPS forces her into the family unit\n")
    
    # Step 4: Show evidence floating
    print("--- Evidence Liberation ---\n")
    
    print("GRAMPS Source/Citation (rigid hierarchy):")
    print("  Source → Citations → Attached to Person/Family")
    print("  - Citation locked to specific conclusion")
    print("  - Cannot support multiple interpretations\n")
    
    print("RGPS Evidence (floating):")
    print("  Evidence: 1850 Census Page")
    print("  ├─ Extracted Facts:")
    print("  │   ├─ 'John Smith, age 30' → supports Theory 1 & 2")
    print("  │   ├─ 'Mary Smith, age 25' → supports all theories")
    print("  │   ├─ 'William, age 5' → neutral on biological relationship")
    print("  │   └─ 'Sarah, age 3' → supports Theory 3 (age gap)")
    print("  └─ Same evidence, multiple interpretations\n")
    
    print("=== Liberation Benefits ===")
    print("1. No forced conclusions about relationships")
    print("2. Events exist independently, not 'owned'")
    print("3. Evidence supports multiple theories simultaneously")
    print("4. Complex relationships properly represented")
    print("5. Research process tracked, not just conclusions")


if __name__ == "__main__":
    gramps_liberation_example()