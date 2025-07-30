"""
Example: Theory Branching in ResearchProcess-GPS

This demonstrates how a genealogist might use theory branching to explore
different interpretations of evidence about a person's death date.
"""

from datetime import datetime
from uuid import uuid4

# This would normally import from the engine
from engine.core.models import (
    Identity, Persona, Theory, NameForm, 
    ConfidenceContainer, TheoryType, ExistenceStatus,
    NameType, ResearchGap
)


def theory_branching_example():
    """
    Scenario: You have John Smith in your research. The 1850 census shows
    him as age 30 (born ~1820). But you find a death certificate for 
    "John Smith" in 1853 and another in 1875. Which is your John?
    
    Traditional genealogy software: You have to pick one.
    ResearchProcess-GPS: Create theories and test them!
    """
    
    print("=== ResearchProcess-GPS Theory Branching Example ===\n")
    
    # Step 1: Create the main theory (what we think we know)
    main_theory = Theory(
        hypothesis="John Smith born ~1820, married Mary Jones",
        theory_type=TheoryType.MAIN,
        owner="researcher@example.com"
    )
    print(f"Created main theory: {main_theory.hypothesis}")
    
    # Step 2: Create John Smith identity
    john_identity = Identity(
        existence_status=ExistenceStatus.EVIDENCED
    )
    
    # Add his name
    john_identity.add_name(NameForm(
        given_names=["John"],
        surnames=["Smith"],
        name_type=NameType.BIRTH,
        cultural_context="American"
    ))
    
    # Step 3: Create initial persona in main theory
    main_persona = Persona(
        identity_id=john_identity.id,
        theory_id=main_theory.id,
        interpreted_birth_date=datetime(1820, 1, 1),  # Approximate from census
        theory_notes="Age 30 in 1850 census"
    )
    john_identity.add_persona(main_theory.id, main_persona)
    main_theory.add_entity(john_identity.id, "Identity", john_identity.version.version_id)
    
    print(f"\nCreated identity: {john_identity.get_primary_name().full_name()}")
    print(f"Birth year in main theory: ~{main_persona.interpreted_birth_date.year}")
    
    # Step 4: Find conflicting death records - need to branch!
    print("\n--- Found two death certificates for 'John Smith' ---")
    print("Certificate A: John Smith died 1853, age 33")
    print("Certificate B: John Smith died 1875, age 55")
    print("\nWhich is our John? Let's create theories to test!\n")
    
    # Step 5: Create Theory A - John died in 1853
    theory_a = main_theory.branch(
        new_hypothesis="John Smith died in 1853 (Certificate A is our John)"
    )
    print(f"Branched Theory A: {theory_a.hypothesis}")
    
    # Create persona for Theory A
    persona_a = Persona(
        identity_id=john_identity.id,
        theory_id=theory_a.id,
        interpreted_birth_date=datetime(1820, 1, 1),
        interpreted_death_date=datetime(1853, 6, 15),
        theory_notes="If Certificate A is correct, John died young at 33"
    )
    john_identity.add_persona(theory_a.id, persona_a)
    
    # Add research gap
    theory_a.add_research_gap(ResearchGap(
        description="Need to find what happened to Mary after 1853",
        gap_type="missing_evidence",
        suggested_actions=["Check 1860 census for Mary as widow", 
                          "Look for Mary's remarriage"]
    ))
    
    # Step 6: Create Theory B - John died in 1875
    theory_b = main_theory.branch(
        new_hypothesis="John Smith died in 1875 (Certificate B is our John)"
    )
    print(f"Branched Theory B: {theory_b.hypothesis}")
    
    # Create persona for Theory B
    persona_b = Persona(
        identity_id=john_identity.id,
        theory_id=theory_b.id,
        interpreted_birth_date=datetime(1820, 1, 1),
        interpreted_death_date=datetime(1875, 3, 22),
        theory_notes="If Certificate B is correct, John lived to 55"
    )
    john_identity.add_persona(theory_b.id, persona_b)
    
    # Add research gap
    theory_b.add_research_gap(ResearchGap(
        description="Need to find John in 1860 and 1870 census",
        gap_type="missing_evidence",
        suggested_actions=["Search 1860 census", "Search 1870 census",
                          "Look for John and Mary together post-1853"]
    ))
    
    # Step 7: Show how the same identity exists differently in each theory
    print("\n--- How John Smith exists in each theory ---")
    
    for theory_id, theory_name in [
        (main_theory.id, "Main Theory"),
        (theory_a.id, "Theory A (died 1853)"),
        (theory_b.id, "Theory B (died 1875)")
    ]:
        persona = john_identity.get_persona_for_theory(theory_id)
        if persona:
            death_info = f"died {persona.interpreted_death_date.year}" if persona.interpreted_death_date else "death unknown"
            print(f"\n{theory_name}:")
            print(f"  Born: ~{persona.interpreted_birth_date.year}")
            print(f"  Status: {death_info}")
            print(f"  Notes: {persona.theory_notes}")
    
    # Step 8: Show research gaps identified
    print("\n--- Research needed to test theories ---")
    
    print("\nTheory A research gaps:")
    for gap in theory_a.research_gaps:
        print(f"  • {gap.description}")
        for action in gap.suggested_actions:
            print(f"    - {action}")
    
    print("\nTheory B research gaps:")
    for gap in theory_b.research_gaps:
        print(f"  • {gap.description}")
        for action in gap.suggested_actions:
            print(f"    - {action}")
    
    # Step 9: Simulate finding evidence that supports Theory B
    print("\n--- Found John and Mary in 1860 census! ---")
    
    # Evidence supports Theory B
    theory_b.add_supporting_evidence(uuid4(), 0.9)  # Strong support
    theory_a.add_contradicting_evidence(uuid4(), "1860 census shows John alive")
    
    # Calculate confidence
    print(f"\nTheory confidence after evidence:")
    print(f"  Theory A: {theory_a.calculate_confidence_score():.2%}")
    print(f"  Theory B: {theory_b.calculate_confidence_score():.2%}")
    
    print("\n=== End Example ===")
    print("\nThis demonstrates how ResearchProcess-GPS enables:")
    print("1. Testing multiple theories simultaneously")
    print("2. Same person existing differently in each theory")
    print("3. Systematic tracking of research needs")
    print("4. Evidence-based theory evaluation")
    print("5. No forced conclusions until evidence supports them")


if __name__ == "__main__":
    theory_branching_example()