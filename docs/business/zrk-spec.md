# zrk — Design Specification

**Version**: 1.0  
**Language**: Rust  
**Status**: Draft

---

## 1. Motivation

### The workflow gap

Solo developers working with large, multi-language codebases increasingly rely on two distinct AI tools:

- **AI coding agents** (Kiro, Claude Code, Cursor, Windsurf) — where code is _written_, inside the IDE
- **Claude.ai web** — where code is _reviewed_, as a conversation

These two tools don't talk to each other. The gap between them is manual, repetitive, and lossy.

### The review problem

The most natural way to review code is to send it to Claude.ai. But:

| Approach                                       | Problem                                                                |
| ---------------------------------------------- | ---------------------------------------------------------------------- |
| Zip and upload the whole codebase              | Hits context limit immediately on any real project                     |
| Send a git patch                               | Missing context — changed files reference other files Claude can't see |
| Manually copy-paste related files              | Slow, inconsistent, easy to miss dependencies                          |
| Write the review prompt from scratch each time | Repetitive, no structure, no role perspective                          |

None of these scale. Code review either doesn't happen, or happens poorly.

### The insight

What Claude.ai actually needs is not the whole codebase — it needs **smart context**:

1. **The diff** — what changed, stripped of formatting noise
2. **The referenced context** — files imported by / importing the changed files, resolved automatically
3. **A structured prompt** — the right roles, the right questions, the right output format, calibrated to the nature of the change

Generating this manually every time is the real friction. It should take one sentence.

### What zrk does

`zrk` installs a set of review workflows into whatever AI coding agent you use. Once installed, you type one sentence into your agent's chat:

> _"tạo materials để review 3 commit gần nhất"_
> _"pack phần authentication để review"_
> _"tạo materials của toàn bộ docs về architect"_

The agent produces a `review_context.xml` and a `review_prompt.md` — ready to upload to Claude.ai. The prompt is already structured with the right roles (SWE, SA, QA, CEO, Devil's Advocate), calibrated to the complexity of the change, and formatted for Claude's best output.

The CLI (`zrk`) handles the distribution: getting those workflow files into whichever agent you use, in whatever format that agent requires.

## 2. Goals

| Goal                         | Description                                                  |
| ---------------------------- | ------------------------------------------------------------ |
| **Single source of truth**   | All command content lives in one place, versioned, editable  |
| **Multi-agent distribution** | One command installs to any supported agent                  |
| **Zero runtime dependency**  | Single static binary — no Python, no Node, no runtime needed |
| **Extensible**               | New agents added via adapter trait, no core changes          |
| **Fast**                     | Startup < 50ms, install < 500ms                              |

---

## 3. Architecture

```
zaob-review-kit/
├── Cargo.toml
├── Cargo.lock
├── README.md
│
├── src/
│   ├── main.rs               # CLI entrypoint, arg parsing
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── install.rs        # install, install-global, install-all
│   │   ├── update.rs         # update (force reinstall)
│   │   ├── status.rs         # status check
│   │   ├── list.rs           # list agents / content files
│   │   └── init.rs           # interactive first-time setup
│   ├── agent/
│   │   ├── mod.rs            # Agent trait definition
│   │   ├── registry.rs       # agent registry (name → impl)
│   │   ├── kiro.rs
│   │   ├── claude_code.rs
│   │   ├── cursor.rs
│   │   └── windsurf.rs
│   ├── content/
│   │   ├── mod.rs            # ContentStore — load embedded content
│   │   └── transform.rs      # apply frontmatter/wrapper per agent
│   └── util/
│       ├── fs.rs             # file write, dir creation, gitignore append
│       └── output.rs         # colored terminal output helpers
│
└── content/                  # embedded at compile time via include_str!
    ├── global/
    │   ├── review-roles.md
    │   ├── review-prompting.md
    │   ├── review-ignore.md
    │   └── review-memory.md
    ├── workspace/
    │   ├── prep-review.md
    │   ├── pack-materials.md
    │   └── project-context.md
    └── templates/
        ├── archignore
        └── gitignore-snippet.txt
```

---

## 4. Content Model

Content files are **format-agnostic markdown** — they contain no agent-specific syntax.  
Agents apply their own wrapper/frontmatter at install time via the `Agent` trait.

```
content/global/review-roles.md     ← pure content, no agent syntax
     ↓ Agent::transform_global()
.kiro/steering/review-roles.md     ← Kiro frontmatter added
.cursor/rules/review-roles.mdc     ← MDC frontmatter added
~/.claude/commands/review-kit/...  ← Claude Code frontmatter added
```

### ContentScope

```rust
pub enum ContentScope {
    Global,    // applies to all projects (roles, prompting, ignore, memory)
    Workspace, // per-project (prep-review, pack-materials, project-context)
    Template,  // config files copied to project root (.archignore, etc.)
}
```

### ContentFile

```rust
pub struct ContentFile {
    pub name: String,          // "review-roles.md"
    pub scope: ContentScope,
    pub raw: &'static str,     // embedded at compile time
}
```

Content is embedded via `include_str!` at compile time → single binary, no external files needed.

---

## 5. Agent Trait

```rust
pub struct TransformOutput {
    pub filename: String,      // output filename (may differ: .md → .mdc)
    pub content: String,       // transformed content with frontmatter
    pub manual_only: bool,     // true = can't write to global, print instead
}

pub trait Agent {
    fn name(&self) -> &str;
    fn label(&self) -> &str;

    // Where files land
    fn global_dir(&self) -> Option<PathBuf>;              // None = manual only
    fn workspace_dir(&self, cwd: &Path) -> PathBuf;

    // How content is wrapped for this agent
    fn transform_global(&self, file: &ContentFile) -> TransformOutput;
    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput;
}
```

### Agent implementations

| Agent        | global_dir                       | workspace_dir                  | Format                    |
| ------------ | -------------------------------- | ------------------------------ | ------------------------- |
| `Kiro`       | `~/.kiro/steering/`              | `.kiro/steering/`              | `.md` + YAML frontmatter  |
| `ClaudeCode` | `~/.claude/commands/review-kit/` | `.claude/commands/review-kit/` | `.md` + YAML frontmatter  |
| `Cursor`     | `None` (UI only)                 | `.cursor/rules/`               | `.mdc` + YAML frontmatter |
| `Windsurf`   | `None` (UI only)                 | `.windsurf/rules/`             | `.md` + comment header    |

### Agent Registry

```rust
// src/agent/registry.rs
pub fn all_agents() -> Vec<Box<dyn Agent>> { ... }
pub fn get_agent(name: &str) -> Option<Box<dyn Agent>> { ... }
```

---

## 6. CLI Interface

```
USAGE
    zrk <command> [options]

COMMANDS
    install           Install workspace files into current project
    install-global    Install global files into agent's global config
    install-all       Install both (recommended first time)
    update            Force reinstall all with latest content
    status            Show installation status
    list              List available agents and content files
    init              Interactive first-time setup wizard
    help              Show this message

OPTIONS
    --target <agent>  Target agent (default: kiro)
                      Supported: kiro, claude-code, cursor, windsurf
    --all-targets     Apply to all supported agents
    --force           Overwrite existing files
    --cwd <path>      Project directory (default: cwd)
    --no-color        Disable colored output
    --quiet           Suppress non-essential output

EXAMPLES
    zrk install-all                        # Kiro, current dir
    zrk install-all --target cursor        # Cursor
    zrk install-all --all-targets          # All agents at once
    zrk update --target kiro
    zrk status --all-targets
    zrk init                               # guided setup wizard
```

---

## 7. Command Behaviors

### `install` / `install-global` / `install-all`

```
1. Load adapter for --target
2. Read content files (embedded) for relevant scope
3. For each file:
   a. Call agent.transform_*() to get TransformOutput
   b. If manual_only=true: print content with instructions, save to ~/.zrk/reference/<agent>/
   c. Else: write to target dir (skip if exists and !force)
4. Write templates (.archignore, gitignore snippet) to cwd
5. Print summary: N installed, M skipped, P manual
```

### `update`

Same as `install-all` with `--force` always true.

### `status`

For each agent (filtered by --target or all):

```
Kiro
  ✓  Global  ~/.kiro/steering/          (4 files)
  ✓  Workspace  .kiro/steering/         (3 files)
  ✓  .archignore
  –  .materials/review_memory.md        (created after first run)

Cursor
  ~  Global  (manual setup required)
  ✓  Workspace  .cursor/rules/          (3 files)
```

### `init` (interactive wizard)

```
? Which agent are you using? (select)
  > Kiro
    Claude Code
    Cursor
    Windsurf
    Multiple (select all that apply)

? Install global rules? (affects all your projects) [Y/n]

? Generate project-context.md scaffold? [Y/n]

→ Installing...
→ Done. Next: ask your agent to fill in project-context.md
```

---

## 8. Output Format

Colored terminal output, no external crate dependency for colors (use ANSI directly or `colored` crate):

```
✓  green   success / installed
⚠  yellow  warning / skipped / manual
✗  red     error / not installed
→  cyan    info / action
–  dim     neutral / not yet created (expected)
```

---

## 9. Error Handling

All errors use Rust's `Result<T, KrkError>`. No panics in production paths.

```rust
pub enum KrkError {
    Io(std::io::Error),
    UnknownAgent(String),
    ContentNotFound(String),
    PermissionDenied(PathBuf),
}
```

All errors print a human-readable message + actionable suggestion:

```
✗ Permission denied: /usr/local/.kiro/steering/
  → Try running with appropriate permissions, or use --cwd to target a different directory.
```

---

## 10. Content Embedding

Content files are embedded at compile time using `include_str!`:

```rust
// src/content/mod.rs
pub fn all_content() -> Vec<ContentFile> {
    vec![
        ContentFile {
            name: "review-roles.md".into(),
            scope: ContentScope::Global,
            raw: include_str!("../../content/global/review-roles.md"),
        },
        // ...
    ]
}
```

**Benefit**: single binary distribution — `zrk` carries all content inside itself. No external files needed after install.

---

## 11. Crate Dependencies

Minimal dependency surface — principle: prefer stdlib, add crate only when clearly justified.

| Crate       | Version | Purpose                           | Justification                         |
| ----------- | ------- | --------------------------------- | ------------------------------------- |
| `clap`      | 4.x     | Arg parsing                       | derive macro, standard for Rust CLIs  |
| `dirs`      | 5.x     | `~` home dir resolution           | cross-platform home dir               |
| `colored`   | 2.x     | Terminal colors                   | simpler than raw ANSI, cross-platform |
| `dialoguer` | 0.11    | `init` wizard interactive prompts | only for `init` command               |

No async runtime needed — all operations are synchronous file I/O.

---

## 12. Distribution

### Binary releases (primary)

```
zrk-linux-x86_64
zrk-linux-aarch64
zrk-macos-x86_64
zrk-macos-aarch64    (Apple Silicon)
zrk-windows-x86_64.exe
```

Build via GitHub Actions with `cross` for cross-compilation.

### Install methods

```bash
# curl (unix)
curl -fsSL https://raw.githubusercontent.com/<org>/zrk/main/install.sh | sh

# cargo
cargo install zrk

# homebrew (future)
brew install zrk
```

### install.sh behavior

1. Detect OS + arch
2. Download correct binary from GitHub Releases
3. Place in `~/.local/bin/zrk` (or `/usr/local/bin/` with sudo)
4. Add to PATH if needed

---

## 13. Adding a New Agent

1. Add `src/agent/<newagent>.rs` implementing the `Agent` trait
2. Register in `src/agent/registry.rs`
3. No other changes required — CLI, content, install logic all work automatically

```rust
// src/agent/newtool.rs
pub struct NewTool;

impl Agent for NewTool {
    fn name(&self) -> &str { "newtool" }
    fn label(&self) -> &str { "NewTool" }
    fn global_dir(&self) -> Option<PathBuf> {
        Some(dirs::home_dir()?.join(".newtool").join("rules"))
    }
    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".newtool").join("rules")
    }
    fn transform_global(&self, file: &ContentFile) -> TransformOutput { ... }
    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput { ... }
}
```

---

## 14. File Layout After Install

```
~/.kiro/steering/                   ← global (Kiro)
  review-roles.md
  review-prompting.md
  review-ignore.md
  review-memory.md

your-project/
  .kiro/steering/                   ← workspace (Kiro)
    prep-review.md
    pack-materials.md
    project-context.md
  .archignore
  .gitignore                        ← .materials/ rules appended
  .materials/
    review_memory.md                ← grows over time (commit this)
    REVIEW_LOG.md                   ← history (commit this)
    20250304_abc-def/               ← per-review output (gitignore)
      review_context.xml
      review.patch
      review_prompt.md
```

---

## 15. Out of Scope (v1)

- Syncing content updates across existing installs (use `zrk update`)
- Per-file version tracking
- Remote content registry / plugin system
- Windows PATH manipulation in install.sh
- GUI

---

## 16. Open Questions

| #   | Question                                                           | Default decision                    |
| --- | ------------------------------------------------------------------ | ----------------------------------- |
| 1   | Should `zrk update` also update `.archignore`?                     | No — user may have customized it    |
| 2   | Should content files support variables (e.g. `{{project_name}}`)?  | No in v1, add in v2 if needed       |
| 3   | Single binary with content embedded vs. content as separate files? | **Embedded** — simpler distribution |
| 4   | Should `init` detect which agent is installed on the machine?      | Yes, detect via known binary names  |
