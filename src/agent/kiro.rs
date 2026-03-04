use super::{Agent, TransformOutput};
use crate::content::transform::wrap_yaml_frontmatter;
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

pub struct Kiro {
    pub home_override: Option<PathBuf>,
}

impl Kiro {
    pub fn new() -> Self {
        Self { home_override: None }
    }

    fn home(&self) -> Option<PathBuf> {
        self.home_override.clone().or_else(dirs::home_dir)
    }
}

impl Agent for Kiro {
    fn name(&self) -> &str {
        "kiro"
    }

    fn label(&self) -> &str {
        "Kiro"
    }

    fn global_dir(&self) -> Option<PathBuf> {
        Some(self.home()?.join(".kiro").join("steering"))
    }

    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".kiro").join("steering")
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
