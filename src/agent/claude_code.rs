use super::{Agent, TransformOutput};
use crate::content::transform::wrap_yaml_frontmatter;
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

pub struct ClaudeCode {
    pub home_override: Option<PathBuf>,
}

impl ClaudeCode {
    pub fn new() -> Self {
        Self { home_override: None }
    }

    fn home(&self) -> Option<PathBuf> {
        self.home_override.clone().or_else(dirs::home_dir)
    }
}

impl Agent for ClaudeCode {
    fn name(&self) -> &str {
        "claude-code"
    }

    fn label(&self) -> &str {
        "Claude Code"
    }

    fn global_dir(&self) -> Option<PathBuf> {
        Some(self.home()?.join(".claude").join("commands").join("review-kit"))
    }

    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".claude").join("commands").join("review-kit")
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
