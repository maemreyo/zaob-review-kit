pub mod antigravity;
pub mod claude_code;
pub mod cursor;
pub mod home;
pub mod kiro;
pub mod registry;
pub mod trae;
pub mod windsurf;

use crate::content::ContentFile;
use std::path::{Path, PathBuf};

/// Output of transforming a content file for a specific agent.
pub struct TransformOutput {
    pub filename: String,
    pub content: String,
    #[allow(dead_code)]
    pub manual_only: bool,
}

/// Trait for agent-specific behavior.
pub trait Agent {
    fn name(&self) -> &str;
    fn label(&self) -> &str;
    fn global_dir(&self) -> Option<PathBuf>;
    fn workspace_dir(&self, cwd: &Path) -> PathBuf;
    fn transform_global(&self, file: &ContentFile) -> TransformOutput;
    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput;

    /// Directory for lazy-loaded role standard files.
    /// Default: `<workspace_dir>/role-standards/`
    /// Agents that need a custom location (e.g. routing to a separate path)
    /// can override this.
    fn role_standards_dir(&self, cwd: &Path) -> PathBuf {
        self.workspace_dir(cwd).join("role-standards")
    }

    /// Transform a role-standard file for installation.
    /// Default: delegates to `transform_workspace` so each agent's native
    /// format (YAML frontmatter, MDC, comment header, plain) is applied
    /// consistently. Override when the standard workspace transform is
    /// unsuitable — e.g. Kiro adds `inclusion: agent-requested` to prevent
    /// auto-loading; TRAE overrides because its workspace transform always
    /// consolidates into a single file.
    fn transform_role_standard(&self, file: &ContentFile) -> TransformOutput {
        self.transform_workspace(file)
    }

    /// Filter agent-specific tagged sections from raw content before wrapping.
    ///
    /// Strips `<!-- agent:OTHER:start --> ... <!-- agent:OTHER:end -->` blocks
    /// that belong to a different agent. Call this on `file.raw` inside every
    /// `transform_*` method instead of using `file.raw` directly.
    ///
    /// Default implementation delegates to `content::transform::filter_agent_sections`.
    /// The lint suppression is needed because Rust cannot trace calls through
    /// dynamic trait dispatch at compile time.
    #[allow(dead_code)]
    fn filter_content(&self, raw: &str) -> String {
        crate::content::transform::filter_agent_sections(raw, self.name())
    }

    /// Optional secondary workspace directory for workflow files (e.g. Antigravity's .agent/workflows/).
    /// When Some, the planner routes files where is_workflow_file() returns true there instead of workspace_dir.
    fn workflow_dir(&self, _cwd: &Path) -> Option<PathBuf> {
        None
    }

    /// Returns true when the named workspace file should go into workflow_dir instead of workspace_dir.
    /// Only meaningful when workflow_dir() returns Some.
    fn is_workflow_file(&self, _name: &str) -> bool {
        false
    }

    /// When true, the planner uses AppendToFile instead of WriteFile for workspace files,
    /// consolidating all content into a single file (e.g. TRAE's project_rules.md).
    /// Role standard files are never consolidated regardless of this flag.
    fn consolidates_to_single_file(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContentFile, ContentScope};
    use std::path::Path;

    fn test_file() -> ContentFile {
        ContentFile {
            name: "review-roles.md".into(),
            scope: ContentScope::Global,
            raw: "# Test content",
        }
    }

    // Kiro tests
    #[test]
    fn kiro_name_and_label() {
        let k = kiro::Kiro::new();
        assert_eq!(k.name(), "kiro");
        assert_eq!(k.label(), "Kiro");
    }

    #[test]
    fn kiro_global_dir_exists() {
        let mut k = kiro::Kiro::new();
        k.home.override_path = Some(PathBuf::from("/fake/home"));
        let dir = k.global_dir().unwrap();
        assert_eq!(dir, PathBuf::from("/fake/home/.kiro/steering"));
    }

    #[test]
    fn kiro_workspace_dir() {
        let k = kiro::Kiro::new();
        let dir = k.workspace_dir(Path::new("/project"));
        assert_eq!(dir, PathBuf::from("/project/.kiro/steering"));
    }

    #[test]
    fn kiro_role_standards_dir() {
        let k = kiro::Kiro::new();
        let dir = k.role_standards_dir(Path::new("/project"));
        assert_eq!(dir, PathBuf::from("/project/.kiro/steering/role-standards"));
    }

    #[test]
    fn kiro_transform_produces_yaml() {
        let k = kiro::Kiro::new();
        let output = k.transform_global(&test_file()); // review-roles.md → auto
        assert!(output.content.starts_with("---\n"));
        assert!(output.content.contains("inclusion: auto"));
        assert!(!output.manual_only);
        assert_eq!(output.filename, "review-roles.md");
    }

    #[test]
    fn kiro_transform_role_standard_has_manual_inclusion() {
        let k = kiro::Kiro::new();
        let file = ContentFile {
            name: "01-swe-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: "# SWE checklist",
        };
        let output = k.transform_role_standard(&file);
        assert!(output.content.contains("inclusion: manual"));
        assert!(!output.content.contains("agent-requested"));
        assert_eq!(output.filename, "01-swe-standard.md");
        assert!(!output.manual_only);
    }

    // Claude Code tests
    #[test]
    fn claude_code_name_and_label() {
        let c = claude_code::ClaudeCode::new();
        assert_eq!(c.name(), "claude-code");
        assert_eq!(c.label(), "Claude Code");
    }

    #[test]
    fn claude_code_global_dir_exists() {
        let mut c = claude_code::ClaudeCode::new();
        c.home.override_path = Some(PathBuf::from("/fake/home"));
        let dir = c.global_dir().unwrap();
        assert_eq!(dir, PathBuf::from("/fake/home/.claude/commands/review-kit"));
    }

    // Cursor tests
    #[test]
    fn cursor_name_and_label() {
        let c = cursor::Cursor::new();
        assert_eq!(c.name(), "cursor");
        assert_eq!(c.label(), "Cursor");
    }

    #[test]
    fn cursor_global_dir_is_none() {
        let c = cursor::Cursor::new();
        assert!(c.global_dir().is_none());
    }

    #[test]
    fn cursor_workspace_dir() {
        let c = cursor::Cursor::new();
        let dir = c.workspace_dir(Path::new("/project"));
        assert_eq!(dir, PathBuf::from("/project/.cursor/rules"));
    }

    #[test]
    fn cursor_role_standards_dir() {
        let c = cursor::Cursor::new();
        let dir = c.role_standards_dir(Path::new("/project"));
        assert_eq!(dir, PathBuf::from("/project/.cursor/rules/role-standards"));
    }

    #[test]
    fn cursor_transform_produces_mdc() {
        let c = cursor::Cursor::new();
        let output = c.transform_workspace(&test_file());
        assert!(output.filename.ends_with(".mdc"));
        assert!(output.content.contains("description:"));
    }

    #[test]
    fn cursor_transform_role_standard_produces_mdc() {
        let c = cursor::Cursor::new();
        let file = ContentFile {
            name: "01-swe-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: "# SWE",
        };
        let output = c.transform_role_standard(&file);
        // Cursor delegates to transform_workspace → .mdc extension
        assert!(output.filename.ends_with(".mdc"));
    }

    #[test]
    fn cursor_global_is_manual_only() {
        let c = cursor::Cursor::new();
        let output = c.transform_global(&test_file());
        assert!(output.manual_only);
    }

    // Windsurf tests
    #[test]
    fn windsurf_name_and_label() {
        let w = windsurf::Windsurf::new();
        assert_eq!(w.name(), "windsurf");
        assert_eq!(w.label(), "Windsurf");
    }

    #[test]
    fn windsurf_global_dir_is_none() {
        let w = windsurf::Windsurf::new();
        assert!(w.global_dir().is_none());
    }

    #[test]
    fn windsurf_transform_produces_comment_header() {
        let w = windsurf::Windsurf::new();
        let output = w.transform_workspace(&test_file());
        assert!(output.content.contains("<!-- trigger:"));
        assert!(!output.manual_only);
    }

    #[test]
    fn windsurf_global_is_manual_only() {
        let w = windsurf::Windsurf::new();
        let output = w.transform_global(&test_file());
        assert!(output.manual_only);
    }

    // TRAE role_standard test — must NOT consolidate into project_rules.md
    #[test]
    fn trae_transform_role_standard_preserves_filename() {
        let t = trae::Trae::new();
        let file = ContentFile {
            name: "05-se-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: "# SE checklist",
        };
        let output = t.transform_role_standard(&file);
        // Must keep original filename, not "project_rules.md"
        assert_eq!(output.filename, "05-se-standard.md");
        assert!(!output.manual_only);
    }
}
