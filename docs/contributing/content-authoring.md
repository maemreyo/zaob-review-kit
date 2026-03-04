# Content Authoring

How to edit existing content files or add new ones.

---

## The content model

Content files are pure markdown — no agent-specific syntax. All frontmatter is added at install time by agent transform functions. This means one source file works for all four agents.

```
content/global/review-roles.md   ← pure markdown, no frontmatter
       │
       ├─ Kiro install     →  YAML frontmatter added
       ├─ Claude Code      →  YAML frontmatter added
       ├─ Cursor           →  MDC frontmatter + .mdc extension
       └─ Windsurf         →  HTML comment header added
```

---

## Editing existing files

Edit the files in `content/`:

```
content/
  global/
    review-roles.md
    review-prompting.md
    review-ignore.md
    review-memory.md
  workspace/
    prep-review.md
    pack-materials.md
    project-context.md
  templates/
    archignore
    gitignore-snippet.txt
```

Changes are embedded at compile time — rebuild zrk to pick them up:

```bash
cargo build
```

Existing installs are only updated when you run `zrk update`.

---

## Adding a new content file

**1. Create the file:**

```bash
# Global scope
touch content/global/my-new-file.md

# Workspace scope
touch content/workspace/my-new-file.md
```

Write plain markdown. No frontmatter.

**2. Register it in `src/content/mod.rs`:**

```rust
pub fn all_content() -> Vec<ContentFile> {
    vec![
        // ... existing files ...
        ContentFile {
            name: "my-new-file.md".into(),
            scope: ContentScope::Global,  // or Workspace or Template
            raw: include_str!("../../content/global/my-new-file.md"),
        },
    ]
}
```

**3. Done.** The new file automatically:

- Appears in `zrk list` output
- Gets installed by `zrk install-global` (if Global scope)
- Gets installed by `zrk install` (if Workspace scope)
- Gets tracked in `.zrk-manifest.json`
- Shows up in `zrk status`

---

## Writing guidelines

**Trigger-oriented headings.** Start with what action the agent should take:

```markdown
# Prep Review ← good: imperative, what to do

# Review Preparation ← worse: noun phrase
```

**Keep files short.** Each file has a single responsibility. Under 80 lines is a good target.

**No agent-specific syntax.** Don't write Kiro-specific YAML, Cursor-specific MDC, or any agent format. The transform layer handles that.

**No absolute paths.** Reference `.materials/`, `.archignore`, etc. as relative paths. Users have different project roots.

**Imperative instructions.** The agent reads these files and follows them:

```markdown
## Steps

1. Read `.archignore` if it exists
2. Apply patterns to the file list
```

---

## Testing content changes

```bash
# Build with the new content embedded
cargo build

# Verify it shows up
cargo run -- list

# Dry-run install to see what would happen
cargo run -- install-all --dry-run

# Run tests
cargo test content::
```

---

## Next

- [Architecture](architecture.md)
- [Adding a new agent](adding-an-agent.md)
