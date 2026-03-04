use super::{Agent, TransformOutput};
use crate::content::transform::wrap_comment_header;
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

pub struct Windsurf;

impl Windsurf {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for Windsurf {
    fn name(&self) -> &str {
        "windsurf"
    }

    fn label(&self) -> &str {
        "Windsurf"
    }

    fn global_dir(&self) -> Option<PathBuf> {
        None // Windsurf global rules are UI-only
    }

    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".windsurf").join("rules")
    }

    fn transform_global(&self, file: &ContentFile) -> TransformOutput {
        let trigger = file.name.trim_end_matches(".md");
        TransformOutput {
            filename: file.name.clone(),
            content: wrap_comment_header(trigger, file.raw),
            manual_only: true,
        }
    }

    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput {
        let trigger = file.name.trim_end_matches(".md");
        TransformOutput {
            filename: file.name.clone(),
            content: wrap_comment_header(trigger, file.raw),
            manual_only: false,
        }
    }
}
