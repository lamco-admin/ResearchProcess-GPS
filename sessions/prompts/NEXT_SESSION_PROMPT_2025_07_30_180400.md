# Next Session Prompt

## For Greg to Start Next Session

Copy and paste this entire prompt to Claude:

---

I'm Greg Lamberson, continuing development of ResearchProcess-GPS. In the last session (2025-07-30 18:03 EEST), we implemented comprehensive nesting infrastructure with a revolutionary evidence-based Identity/Persona model.

## Current Status

**Repository**: `/home/greg/genealogy-ai/ResearchProcess-GPS/` (private GitHub: lamco-admin/ResearchProcess-GPS)
**Last Handover**: `SESSION_HANDOVER_2025_07_30_180325_EEST.md`

## Key Context

1. **Identity/Persona Revolution**: We implemented a model where each evidence reference creates an IdentityPersona ("Bob" in letter, "Robert Jones" in census) that can nest infinitely and be promoted/demoted to Person status.

2. **Nesting Philosophy**: Everything can nest for research organization, but nesting ≠ relationships. Identities nest, Persons relate.

3. **Next Priority**: Implement Git storage adapter to handle nested structures and identity promotion/demotion.

## Please:

1. First, check the current state:
```bash
cd /home/greg/genealogy-ai/ResearchProcess-GPS
cat SESSION_HANDOVER_2025_07_30_180325_EEST.md
```

2. Review the new identity model:
```bash
cat engine/core/models/identity_persona.py
cat engine/IDENTITY_PERSONA_EXAMPLES.md
```

3. Check what still needs nesting updates:
```bash
grep -l "BaseEntity" engine/core/models/*.py | grep -v "NestableBaseEntity"
```

4. Then let's implement the Git storage adapter, starting with:
   - Review `engine/protocols/storage_protocol.py`
   - Design serialization for deeply nested IdentityPersona structures
   - Handle the promote/demote workflow in Git commits

Remember: I'm VP of Genealogy for Clan Henderson. This needs to handle real genealogical research with privacy as paramount. The protocol should enable genealogists to work like they have Git without knowing Git exists.

---