# Architecture

How `zrk` is structured internally.

---

## Design principles

1. **Single binary** — all content embedded at compile time, no runtime dependencies
2. **Planner/Executor split** — pure planning logic separated from filesystem side effects
3. **Agent trait** — new agents added in isolation, no core changes needed
4. **Format-agnostic content** — source markdown has no agent-specific syntax

---

## Module map

```
src/
  main.rs         ← clap CLI, dispatches to commands/
  error.rs        ← ZrkError enum
  planner.rs      ← pure: User intent → Vec<InstallAction>
  executor.rs     ← side effects: Vec<InstallAction> → files on disk
  manifest.rs     ← SHA-256 tracking, .zrk-manifest.json
  commands/
    install.rs    ← run_install(), run_install_global(), run_install_all()
    update.rs     ← run_update() = install_all(force=true)
    status.rs     ← check_status(), FileState enum
    list.rs       ← run_list()
    init.rs       ← InitConfig (pure) + run_init_with_config() (testable)
  agent/
    mod.rs        ← Agent trait, TransformOutput struct
    registry.rs   ← all_agents(), get_agent()
    kiro.rs       ← Kiro struct
    claude_code.rs← ClaudeCode struct
    cursor.rs     ← Cursor struct
    windsurf.rs   ← Windsurf struct
  content/
    mod.rs        ← ContentFile, ContentScope, all_content(), by_scope(), by_name()
    transform.rs  ← wrap_yaml_frontmatter(), wrap_mdc_frontmatter(), wrap_comment_header()
  util/
    fs.rs         ← ensure_dir(), write_file_safe(), append_gitignore()
    output.rs     ← success(), warning(), error(), info(), neutral()
```

---

## The planner/executor split

The key architectural decision:

```
User command
    │
    ▼
Planner (pure functions, no I/O)
    │
    ▼
Vec<InstallAction>
    │
    ├─ --dry-run → print actions, return
    │
    └─ execute() → write files, update manifests, print results
```

**Planner** is pure Rust — takes agent, cwd, and flags, returns a list of actions. No filesystem reads or writes. Fully testable without tempdir.

**Executor** does all I/O — reads existing files to check if they exist, writes new ones, saves manifests.

This gives `--dry-run` for free and makes the planning logic easy to test.

### InstallAction enum

```rust
pub enum InstallAction {
    CreateDir { path: PathBuf },
    WriteFile { path: PathBuf, content: String, overwrite: bool },
    SkipExisting { path: PathBuf },
    ManualInstruction { agent_label: String, filename: String, content: String },
    WriteManifest { base_dir: PathBuf, agent_name: String },
    AppendGitignore { cwd: PathBuf, snippet: String },
    CopyTemplate { dest: PathBuf, content: String, overwrite: bool },
}
```

Every possible side effect the executor can take is represented as a variant. Dry-run just prints each variant's description.

---

## The Agent trait

```rust
pub struct TransformOutput {
    pub filename: String,   // output filename (may differ: .md → .mdc)
    pub content: String,    // content with agent-specific frontmatter
    pub manual_only: bool,  // true = global install not supported, print instead
}

pub trait Agent {
    fn name(&self) -> &str;                              // "kiro"
    fn label(&self) -> &str;                             // "Kiro"
    fn global_dir(&self) -> Option<PathBuf>;             // None = manual only
    fn workspace_dir(&self, cwd: &Path) -> PathBuf;
    fn transform_global(&self, file: &ContentFile) -> TransformOutput;
    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput;
}
```

- `global_dir()` returning `None` causes the planner to emit `ManualInstruction` actions instead of `WriteFile`
- `transform_*()` applies agent-specific frontmatter to the source content
- Each agent struct holds `home_override: Option<PathBuf>` for test isolation

---

## Content model

```rust
pub enum ContentScope { Global, Workspace, Template }

pub struct ContentFile {
    pub name: String,
    pub scope: ContentScope,
    pub raw: &'static str,   // embedded at compile time via include_str!
}
```

`all_content()` returns all 9 files. `by_scope()` and `by_name()` are convenience filters.

Content files contain pure markdown — no agent-specific syntax. Frontmatter is added by transform functions at install time.

---

## Error handling

```rust
pub enum ZrkError {
    Io(std::io::Error),
    UnknownAgent(String),
    ContentNotFound(String),
    PermissionDenied(PathBuf),
}
```

All errors implement `Display` with human-readable messages and actionable suggestions. No panics in production paths.

---

## Testing strategy

- **Unit tests in-module** — every module has `#[cfg(test)] mod tests { ... }`
- **Planner tests** — no filesystem, just verify `Vec<InstallAction>` contents
- **Executor tests** — use `tempfile::tempdir()` for isolated filesystem
- **Integration tests** — `tests/install_integration.rs` and `tests/status_integration.rs` use `assert_cmd` to run the binary end-to-end

67 tests total, all in `cargo test`.

---

## Next

- [Adding a new agent](adding-an-agent.md)
- [Adding content files](content-authoring.md)
