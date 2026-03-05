use super::home::HomeResolver;
use super::{Agent, TransformOutput};
use crate::content::transform::wrap_yaml_frontmatter;
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

pub struct Kiro {
    pub home: HomeResolver,
}

impl Kiro {
    pub fn new() -> Self {
        Self {
            home: HomeResolver::new(),
        }
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
        Some(self.home.resolve()?.join(".kiro").join("steering"))
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

    /// Role standards use `inclusion: agent-requested` so Kiro does NOT
    /// auto-load them into context at startup. The agent reads exactly one
    /// standard file immediately before writing each role's review output,
    /// then proceeds without holding it in context.
    fn transform_role_standard(&self, file: &ContentFile) -> TransformOutput {
        let description = format!(
            "zrk role standard: {} — load only when writing corresponding review file",
            file.name.trim_end_matches(".md")
        );
        let name = file.name.trim_end_matches(".md");
        let content = format!(
            "---\ndescription: {}\ninclusion: agent-requested\nname: {}\n---\n\n{}",
            description, name, file.raw
        );
        TransformOutput {
            filename: file.name.clone(),
            content,
            manual_only: false,
        }
    }
}
