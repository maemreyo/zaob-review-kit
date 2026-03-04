# Adding a New Agent

Adding support for a new AI coding agent requires two files and one registry entry. Nothing else changes.

---

## Step 1: Create the agent struct

Create `src/agent/newtool.rs`:

```rust
use super::{Agent, TransformOutput};
use crate::content::transform::wrap_yaml_frontmatter;
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

pub struct NewTool {
    pub home_override: Option<PathBuf>, // for test isolation
}

impl NewTool {
    pub fn new() -> Self {
        Self { home_override: None }
    }

    fn home(&self) -> Option<PathBuf> {
        self.home_override.clone().or_else(dirs::home_dir)
    }
}

impl Agent for NewTool {
    fn name(&self) -> &str {
        "newtool"  // used in --target flag
    }

    fn label(&self) -> &str {
        "NewTool"  // used in output messages
    }

    fn global_dir(&self) -> Option<PathBuf> {
        // Return Some(path) if the agent has a filesystem global config dir
        // Return None if global config is UI-only (like Cursor/Windsurf)
        Some(self.home()?.join(".newtool").join("rules"))
    }

    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".newtool").join("rules")
    }

    fn transform_global(&self, file: &ContentFile) -> TransformOutput {
        let description = format!("zrk: {}", file.name.trim_end_matches(".md"));
        TransformOutput {
            filename: file.name.clone(),
            content: wrap_yaml_frontmatter(&file.name, &description, file.raw),
            manual_only: false,
        }
    }

    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput {
        let description = format!("zrk: {}", file.name.trim_end_matches(".md"));
        TransformOutput {
            filename: file.name.clone(),
            content: wrap_yaml_frontmatter(&file.name, &description, file.raw),
            manual_only: false,
        }
    }
}
```

### Choosing a transform function

| Agent format              | Function                                                        |
| ------------------------- | --------------------------------------------------------------- |
| `.md` + YAML frontmatter  | `wrap_yaml_frontmatter()`                                       |
| `.mdc` + YAML frontmatter | `wrap_mdc_frontmatter()` + `change_extension("file.md", "mdc")` |
| `.md` + HTML comment      | `wrap_comment_header()`                                         |

All transform functions are in `src/content/transform.rs`.

### When global_dir() returns None

If the agent requires UI-only global configuration (like Cursor or Windsurf), return `None`. The planner will automatically emit `ManualInstruction` actions instead of `WriteFile` actions, and the output will guide the user to paste the content manually.

---

## Step 2: Register the agent

In `src/agent/registry.rs`, add to both functions:

```rust
use super::newtool::NewTool;  // add this import

pub fn all_agents() -> Vec<Box<dyn Agent>> {
    vec![
        Box::new(Kiro::new()),
        Box::new(ClaudeCode::new()),
        Box::new(Cursor::new()),
        Box::new(Windsurf::new()),
        Box::new(NewTool::new()),  // add this
    ]
}

pub fn get_agent(name: &str) -> Option<Box<dyn Agent>> {
    all_agents().into_iter().find(|a| a.name() == name)
    // works automatically — no changes needed here
}
```

Add the module to `src/agent/mod.rs`:

```rust
pub mod newtool;  // add this
```

---

## Step 3: Write tests

Add tests to `src/agent/newtool.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContentFile, ContentScope};
    use std::path::Path;

    fn test_file() -> ContentFile {
        ContentFile {
            name: "review-roles.md".into(),
            scope: ContentScope::Global,
            raw: "# Test",
        }
    }

    #[test]
    fn name_and_label() {
        let t = NewTool::new();
        assert_eq!(t.name(), "newtool");
        assert_eq!(t.label(), "NewTool");
    }

    #[test]
    fn global_dir_correct_path() {
        let mut t = NewTool::new();
        t.home_override = Some(PathBuf::from("/fake/home"));
        let dir = t.global_dir().unwrap();
        assert_eq!(dir, PathBuf::from("/fake/home/.newtool/rules"));
    }

    #[test]
    fn workspace_dir_correct_path() {
        let t = NewTool::new();
        let dir = t.workspace_dir(Path::new("/project"));
        assert_eq!(dir, PathBuf::from("/project/.newtool/rules"));
    }

    #[test]
    fn transform_produces_expected_format() {
        let t = NewTool::new();
        let output = t.transform_workspace(&test_file());
        assert!(output.content.starts_with("---\n"));
        assert!(!output.manual_only);
    }
}
```

---

## Step 4: Verify

```bash
cargo test agent::
cargo clippy
zrk list  # should show NewTool in the agents list
zrk install-all --target newtool --dry-run
```

---

## That's it

The CLI, planner, executor, status command, and list command all work automatically with any registered agent. No other changes are needed.

---

## Next

- [Architecture overview](architecture.md)
- [Content file authoring](content-authoring.md)
