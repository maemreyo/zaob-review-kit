# Content Files Reference

All nine content files are embedded in the `zrk` binary at compile time via `include_str!`. No external files needed at runtime.

---

## File inventory

| File                    | Scope     | Format per agent                                                             | Purpose                                        |
| ----------------------- | --------- | ---------------------------------------------------------------------------- | ---------------------------------------------- |
| `review-roles.md`       | Global    | YAML frontmatter (Kiro/Claude Code), MDC (Cursor), comment header (Windsurf) | Five reviewer role definitions                 |
| `review-prompting.md`   | Global    | Same as above                                                                | Output structure and size calibration          |
| `review-ignore.md`      | Global    | Same as above                                                                | Default patterns excluded from context packing |
| `review-memory.md`      | Global    | Same as above                                                                | Review memory format and instructions          |
| `prep-review.md`        | Workspace | Same as above                                                                | Main workflow trigger                          |
| `pack-materials.md`     | Workspace | Same as above                                                                | `review_context.xml` packing rules             |
| `project-context.md`    | Workspace | Same as above                                                                | Project description scaffold (you fill in)     |
| `archignore`            | Template  | Copied as `.archignore` to project root                                      | Project-specific ignore patterns               |
| `gitignore-snippet.txt` | Template  | Appended to `.gitignore`                                                     | Gitignore rules for `.materials/` output dirs  |

---

## ContentScope

```rust
pub enum ContentScope {
    Global,    // installed to agent's global config directory
    Workspace, // installed to project-local agent config directory
    Template,  // copied to project root
}
```

**Global** files are installed once per machine. Your agent loads them for every project.

**Workspace** files are installed per project. They contain project-specific workflow behavior.

**Template** files go to the project root (not the agent's config dir).

---

## Format transformation

The same source markdown gets transformed into agent-specific formats at install time:

```
content/global/review-roles.md  (source — pure markdown)
       │
       ├─ Kiro       →  ~/.kiro/steering/review-roles.md
       │                (YAML frontmatter added)
       │
       ├─ ClaudeCode →  ~/.claude/commands/review-kit/review-roles.md
       │                (YAML frontmatter added)
       │
       ├─ Cursor     →  .cursor/rules/review-roles.mdc
       │                (MDC frontmatter + .mdc extension)
       │
       └─ Windsurf   →  .windsurf/rules/review-roles.md
                        (HTML comment trigger header added)
```

Source files contain no agent-specific syntax. All agent-specific formatting is added by the `transform_global` and `transform_workspace` methods on each agent implementation.

---

## Adding content to a file

The workflow files (`prep-review.md`, `pack-materials.md`, etc.) are editable markdown. After `zrk install-all`, you can edit the installed copies directly to customize behavior for your project. These are your files — `zrk update` will overwrite them, so keep customizations in `project-context.md` or `.archignore` instead.

---

## Checking what's installed

```bash
zrk list
```

Shows all 9 content files with their scope, and all 4 agents with their install paths.

---

## Next

- [Workflow overview](../workflow/overview.md)
- [File layout after install](file-layout.md)
- [Architecture — content embedding model](../contributing/architecture.md)
