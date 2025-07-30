# Next Session Prompt - ResearchProcess-GPS Implementation

I'm Greg Lamberson, continuing development of ResearchProcess-GPS - a revolutionary genealogical research platform that works like "GitHub for Genealogy."

## Current Status

I've completed the v0.1 framework design with comprehensive architecture documents in `/home/greg/genealogy-ai/ResearchProcess-GPS/v0.1-framework/`. The key insight: ResearchProcess-GPS is a **protocol** (like Git) not just another genealogy platform.

## Key Design Elements

1. **Core Vision**: Version-controlled genealogical research with theory branching, evidence floating between theories, and full collaboration support.

2. **Technical Stack Decided**:
   - Django backend (modular architecture)
   - Git for version control (hidden from users)
   - PostgreSQL + Apache AGE for graph database
   - Private encrypted repositories (not public GitHub)

3. **Real Use Case**: I'm VP of Genealogy for Clan Henderson Society with immediate needs:
   - Organize multiple GEDCOM files from Ancestry
   - Integrate Geni profiles and FTDNA project data
   - Manage Google Docs repository
   - Provide secure access to clan members

4. **Privacy Critical**: Unified privacy framework treating ALL data with DNA-level security. GDPR compliant, provider requirements met.

## Key Documents to Review

In `/home/greg/genealogy-ai/ResearchProcess-GPS/v0.1-framework/`:
- `COMPREHENSIVE_HANDOVER_2025_01_30.md` - Complete session summary
- `UNLEASHED_CORE_DATA_MODEL.md` - Revolutionary entity design
- `GITHUB_FOR_GENEALOGY_VISION.md` - Protocol approach
- `SECURE_MODULAR_FOUNDATION.md` - Django implementation plan
- `UNIFIED_PRIVACY_FRAMEWORK.md` - Privacy architecture

## Where We're Starting

Ready to begin implementation. Need to:

1. **Set up Django project** with modular architecture outlined in framework
2. **Implement Git storage service** with encryption for private repos
3. **Create base models** starting with Identity (dual-mode: working/published)
4. **Build GRAMPS interoperability** as proof of concept
5. **Create simple UI** for Clan Henderson pilot

## Key Principles to Maintain

- **Protocol over platform** - Enable many tools, don't lock in users
- **Privacy by design** - Everything encrypted, consented, audited
- **Hide complexity** - Users shouldn't know Git is involved
- **Modular/extensible** - Expect unexpected uses
- **Real-world focus** - Clan Henderson validates everything

## Questions for This Session

1. Should we start with Django project setup or refine any architecture?
2. Best approach for Git integration that's invisible to users?
3. How to structure the modular plugin system in Django?
4. What's the minimal viable feature set for Clan Henderson pilot?

The revolution is: genealogists get version control, branching, merging, and collaboration without knowing anything about Git. Their research becomes truly portable, shareable, and preservable while maintaining highest privacy standards.

Let's build the foundation that makes this possible.