use super::{Agent, TransformOutput};
use crate::content::transform::as_plain;
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

/// TRAE (ByteDance) agent.
///
/// Workspace: .trae/rules/project_rules.md — all rules consolidated into one file
/// Global:    UI-only (manual)
pub struct Trae;

impl Trae {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for Trae {
    fn name(&self) -> &str {
        "trae"
    }

    fn label(&self) -> &str {
        "TRAE"
    }

    fn global_dir(&self) -> Option<PathBuf> {
        None
    }

    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".trae").join("rules")
    }

    fn consolidates_to_single_file(&self) -> bool {
        true
    }

    fn transform_global(&self, file: &ContentFile) -> TransformOutput {
        TransformOutput {
            filename: file.name.clone(),
            content: as_plain(file.raw),
            manual_only: true,
        }
    }

    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput {
        TransformOutput {
            filename: "project_rules.md".to_string(),
            content: as_plain(file.raw),
            manual_only: false,
        }
    }

    /// Role standards must stay as individual files even though TRAE normally
    /// consolidates workspace files into project_rules.md. Loading standards
    /// one-at-a-time requires each to be a separate, addressable file.
    fn transform_role_standard(&self, file: &ContentFile) -> TransformOutput {
        TransformOutput {
            filename: file.name.clone(),
            content: as_plain(file.raw),
            manual_only: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContentFile, ContentScope};

    fn workspace_file() -> ContentFile {
        ContentFile {
            name: "prep-review.md".into(),
            scope: ContentScope::Workspace,
            raw: "# Prep",
        }
    }

    #[test]
    fn trae_name_and_label() {
        assert_eq!(Trae::new().name(), "trae");
        assert_eq!(Trae::new().label(), "TRAE");
    }

    #[test]
    fn trae_global_dir_is_none() {
        assert!(Trae::new().global_dir().is_none());
    }

    #[test]
    fn trae_workspace_dir() {
        assert_eq!(
            Trae::new().workspace_dir(Path::new("/project")),
            PathBuf::from("/project/.trae/rules")
        );
    }

    #[test]
    fn trae_consolidates_to_single_file() {
        assert!(Trae::new().consolidates_to_single_file());
    }

    #[test]
    fn trae_transform_workspace_targets_project_rules() {
        let out = Trae::new().transform_workspace(&workspace_file());
        assert_eq!(out.filename, "project_rules.md");
        assert_eq!(out.content, "# Prep");
        assert!(!out.manual_only);
    }

    #[test]
    fn trae_transform_global_is_manual_only() {
        let file = ContentFile {
            name: "review-roles.md".into(),
            scope: ContentScope::Global,
            raw: "# Roles",
        };
        let out = Trae::new().transform_global(&file);
        assert!(out.manual_only);
    }
}
