# User-Friendly Research Platform (Powered by Git)

## The Problem

Git is powerful but genealogists shouldn't need to know:
- What a "commit" is
- How to resolve merge conflicts
- What "push" and "pull" mean
- Command line anything

## The Solution: Genealogy-First Interface

### What Users See vs What Actually Happens

```yaml
UserExperience:
  what_user_does: "Save Research Progress"
  what_happens: "git commit"
  
  what_user_does: "Create Alternative Theory"
  what_happens: "git checkout -b"
  
  what_user_does: "Share with Team"
  what_happens: "git push"
  
  what_user_does: "Get Latest Updates"
  what_happens: "git pull"
  
  what_user_does: "See What Changed"
  what_happens: "git diff"
  
  what_user_does: "Go Back to Earlier Version"
  what_happens: "git checkout"
```

## Genealogy-Friendly Terminology

```yaml
TerminologyMapping:
  git_term: "repository"
  genealogy_term: "Research Project"
  
  git_term: "commit"
  genealogy_term: "Research Checkpoint"
  
  git_term: "branch"
  genealogy_term: "Theory"
  
  git_term: "merge"
  genealogy_term: "Combine Theories"
  
  git_term: "fork"
  genealogy_term: "Build on Research"
  
  git_term: "pull request"
  genealogy_term: "Suggest Improvement"
  
  git_term: "diff"
  genealogy_term: "Compare Versions"
  
  git_term: "conflict"
  genealogy_term: "Different Conclusions"
  
  git_term: "tag"
  genealogy_term: "Publication Milestone"
```

## Simple Desktop Application

```yaml
DesktopInterface:
  main_window:
    - research_tree:  # Visual family tree
        current_theory: "highlighted"
        alternative_theories: "grayed branches"
        
    - evidence_panel:  # Drag and drop
        add_evidence: "Drop files here"
        link_to_person: "Drag to connect"
        
    - theory_switcher:  # Simple dropdown
        options: ["Main Theory", "John died 1853", "Two John Smiths"]
        action: "Just selecting switches theories"
        
    - save_button:  # Big friendly button
        label: "Save Progress"
        keyboard: "Ctrl+S"
        auto_save: "every 5 minutes"
        
  # No terminal, no commands, no complexity
```

## Visual Theory Management

```yaml
TheoryVisualizer:
  # Like a family tree for theories
  theory_tree_view:
    main_trunk:
      name: "Original Research"
      color: "green"
      
    branches:
      - name: "What if John died earlier?"
        color: "blue"
        diverged: "March 2024"
        
      - name: "DNA Evidence Added"
        color: "purple"
        merged_back: true
        
      - name: "German Origin Theory"
        color: "orange"
        status: "exploring"
        
  # Click to switch, drag to merge
  interactions:
    click_branch: "Switch to that theory"
    drag_branch_to_trunk: "Propose combining theories"
    right_click: "See what's different"
```

## One-Click Operations

```yaml
SimpleActions:
  share_research:
    button: "Share with Team"
    what_happens:
      - Creates GitHub/GitLab account if needed
      - Uploads research
      - Sends invite link
      - "No Git knowledge required!"
      
  backup_research:
    button: "Backup to Cloud"
    options:
      - "GitHub (Free)"
      - "My Google Drive"
      - "My Dropbox"
      - "USB Drive"
      
  publish_theory:
    button: "Publish Theory"
    what_happens:
      - Creates permanent version
      - Generates citation
      - Makes shareable link
      - "Like publishing a paper"
      
  import_from_others:
    button: "Import Shared Research"
    what_happens:
      - Paste link from email
      - Downloads their research
      - Opens in new theory branch
      - "Safe to explore!"
```

## Conflict Resolution for Humans

```yaml
ConflictResolution:
  # When theories disagree
  conflict_wizard:
    step1:
      title: "Different Conclusions Found"
      message: "You and Mary both edited John's birth date"
      
    step2:
      title: "Compare Evidence"
      display:
        your_conclusion: "1 Jan 1850"
        your_evidence: "[List of sources]"
        
        their_conclusion: "15 Jan 1850"
        their_evidence: "[List of sources]"
        
    step3:
      title: "Choose Resolution"
      options:
        - "Keep mine"
        - "Use theirs"
        - "Keep both as possibilities"
        - "Create new theory to explore"
        - "Ask for help"
```

## Auto-Git Features

```yaml
AutomatedGitOps:
  # Things that just happen
  auto_save:
    trigger: "Any change"
    frequency: "Batched every 5 minutes"
    message: "Auto-generated from edits"
    
  auto_backup:
    trigger: "Daily at close"
    destination: "User's chosen cloud"
    retention: "All versions kept"
    
  auto_organize:
    images: "Goes to evidence/images/"
    documents: "Goes to evidence/documents/"
    gedcom_imports: "Goes to imports/"
    
  auto_merge:
    non_conflicts: "Automatically combined"
    conflicts: "Friendly wizard appears"
```

## Web Version (Even Simpler)

```yaml
WebInterface:
  # GitHub.com but for genealogy
  homepage:
    my_research_projects: "List with thumbnails"
    shared_with_me: "Projects others shared"
    explore: "Public research to build on"
    
  project_page:
    family_tree_view: "Interactive tree"
    theory_selector: "Dropdown menu"
    evidence_gallery: "Pinterest-like board"
    activity_feed: "Who did what when"
    
  collaboration:
    comments: "On any person/event"
    suggestions: "Propose changes"
    tasks: "Research to-do lists"
```

## Mobile App (Field Research)

```yaml
MobileApp:
  quick_actions:
    - "Photo of gravestone"
    - "Voice note"
    - "GPS location"
    - "Quick transcript"
    
  offline_mode:
    work_anywhere: true
    syncs_when_connected: true
    conflict_resolution: "Same friendly wizard"
```

## The Magic: Hidden Git

```yaml
BehindTheScenes:
  user_action: "Drops photo onto person"
  
  git_operations:
    - "git add evidence/photos/IMG_123.jpg"
    - "git add data/identities/john-smith.yaml"
    - "git commit -m 'Added photo to John Smith'"
    - "If online: git push"
    
  user_sees: "Photo attached! ✓"
```

## Progressive Disclosure

```yaml
SkillLevels:
  beginner:
    sees: "Save, Share, Backup buttons"
    hidden: "All Git concepts"
    
  intermediate:
    sees: "Theory branches, Compare versions"
    hidden: "Command line, merge strategies"
    
  advanced:
    sees: "History timeline, Advanced merge"
    optional: "View Git commands (learning mode)"
    
  developer:
    access: "Full Git repository"
    bonus: "API access, scripting"
```

## Why This Works

1. **Genealogists get**:
   - Simple save/share/backup
   - Visual theory management
   - Automatic organization
   - No technical knowledge needed

2. **Git provides**:
   - Version control
   - Distributed backup
   - Merge capabilities
   - Proven infrastructure

3. **Everyone wins**:
   - Beginners: Just works
   - Advanced: More features available
   - Developers: Full Git access
   - Everyone: Data portability

The key insight: **Git is the engine, not the interface**. Users never need to know Git exists - they just get all its benefits through genealogy-appropriate interactions.