use super::{Agent, TransformOutput};
use crate::content::transform::{change_extension, wrap_mdc_frontmatter};
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

pub struct Cursor;

impl Cursor {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for Cursor {
    fn name(&self) -> &str {
        "cursor"
    }

    fn label(&self) -> &str {
        "Cursor"
    }

    fn global_dir(&self) -> Option<PathBuf> {
        None // Cursor global rules are UI-only
    }

    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".cursor").join("rules")
    }

    fn transform_global(&self, file: &ContentFile) -> TransformOutput {
        let filename = change_extension(&file.name, "mdc");
        let description = format!("zrk: {}", file.name.trim_end_matches(".md"));
        TransformOutput {
            filename,
            content: wrap_mdc_frontmatter(&file.name, &description, "", file.raw),
            manual_only: true,
        }
    }

    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput {
        let filename = change_extension(&file.name, "mdc");
        let description = format!("zrk: {}", file.name.trim_end_matches(".md"));
        TransformOutput {
            filename,
            content: wrap_mdc_frontmatter(&file.name, &description, "", file.raw),
            manual_only: false,
        }
    }
}
