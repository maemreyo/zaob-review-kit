use super::home::HomeResolver;
use super::{Agent, TransformOutput};
use crate::content::transform::as_plain;
use crate::content::ContentFile;
use std::path::{Path, PathBuf};

/// Google Antigravity (Gemini CLI) agent.
///
/// Global:    ~/.gemini/GEMINI.md  — all global rules consolidated into one file
/// Workspace rules:     .agent/rules/      — review-* content files
/// Workspace workflows: .agent/workflows/  — prep-review, pack-materials
pub struct Antigravity {
    pub home: HomeResolver,
}

impl Antigravity {
    pub fn new() -> Self {
        Self { home: HomeResolver::new() }
    }

    /// Files that belong in the workflow directory rather than rules.
    fn is_workflow_file(name: &str) -> bool {
        matches!(name, "prep-review.md" | "pack-materials.md" | "project-context.md")
    }
}

impl Agent for Antigravity {
    fn name(&self) -> &str {
        "antigravity"
    }

    fn label(&self) -> &str {
        "Google Antigravity"
    }

    fn global_dir(&self) -> Option<PathBuf> {
        Some(self.home.resolve()?.join(".gemini"))
    }

    fn workspace_dir(&self, cwd: &Path) -> PathBuf {
        cwd.join(".agent").join("rules")
    }

    fn workflow_dir(&self, cwd: &Path) -> Option<PathBuf> {
        Some(cwd.join(".agent").join("workflows"))
    }

    fn transform_global(&self, file: &ContentFile) -> TransformOutput {
        TransformOutput {
            filename: "GEMINI.md".to_string(),
            content: as_plain(file.raw),
            manual_only: false,
        }
    }

    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput {
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

    fn global_file() -> ContentFile {
        ContentFile {
            name: "review-roles.md".into(),
            scope: ContentScope::Global,
            raw: "# Roles",
        }
    }

    fn workflow_file() -> ContentFile {
        ContentFile {
            name: "prep-review.md".into(),
            scope: ContentScope::Workspace,
            raw: "# Prep",
        }
    }

    #[test]
    fn antigravity_name_and_label() {
        let a = Antigravity::new();
        assert_eq!(a.name(), "antigravity");
        assert_eq!(a.label(), "Google Antigravity");
    }

    #[test]
    fn antigravity_global_dir_uses_home_override() {
        let mut a = Antigravity::new();
        a.home.override_path = Some(PathBuf::from("/fake/home"));
        assert_eq!(a.global_dir().unwrap(), PathBuf::from("/fake/home/.gemini"));
    }

    #[test]
    fn antigravity_workspace_dir() {
        let a = Antigravity::new();
        assert_eq!(
            a.workspace_dir(Path::new("/project")),
            PathBuf::from("/project/.agent/rules")
        );
    }

    #[test]
    fn antigravity_workflow_dir() {
        let a = Antigravity::new();
        assert_eq!(
            a.workflow_dir(Path::new("/project")),
            Some(PathBuf::from("/project/.agent/workflows"))
        );
    }

    #[test]
    fn antigravity_transform_global_targets_gemini_md() {
        let a = Antigravity::new();
        let out = a.transform_global(&global_file());
        assert_eq!(out.filename, "GEMINI.md");
        assert_eq!(out.content, "# Roles");
        assert!(!out.manual_only);
    }

    #[test]
    fn antigravity_transform_workspace_is_plain() {
        let a = Antigravity::new();
        let out = a.transform_workspace(&workflow_file());
        assert_eq!(out.filename, "prep-review.md");
        assert_eq!(out.content, "# Prep");
        assert!(!out.manual_only);
    }

    #[test]
    fn antigravity_is_workflow_file_classification() {
        assert!(Antigravity::is_workflow_file("prep-review.md"));
        assert!(Antigravity::is_workflow_file("pack-materials.md"));
        assert!(Antigravity::is_workflow_file("project-context.md"));
        assert!(!Antigravity::is_workflow_file("review-roles.md"));
        assert!(!Antigravity::is_workflow_file("review-memory.md"));
    }
}
