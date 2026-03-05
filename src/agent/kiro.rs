use super::home::HomeResolver;
use super::{Agent, TransformOutput};
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

/// Kiro inclusion mode + metadata for a given steering file.
enum KiroInclusion {
    /// Load in every Kiro interaction. Use for small, always-relevant files.
    Always,
    /// Semantic auto-load: Kiro decides based on description match.
    /// Also activatable as a slash command /<name> in chat.
    Auto {
        name: &'static str,
        description: &'static str,
    },
    /// Load on demand only — agent types #filename or user types /<name> in chat.
    Manual,
}

/// Map filename → Kiro inclusion mode.
///
/// Design rationale (Kiro docs: kiro.dev/docs/steering/):
///
///   always  → tiny files that are relevant to EVERY Kiro interaction.
///              Keep this list short — every `always` file burns context on
///              unrelated coding tasks (e.g. fixing a typo, renaming a var).
///
///   auto    → workflow and reference files needed during review sessions.
///              Kiro loads them when the chat topic matches the description.
///              They also appear as slash commands so the agent can load them
///              explicitly when the auto-match misses.
///
///   manual  → heavy reference docs and all role standard files.
///              Never auto-loaded. Agent loads exactly one standard file
///              immediately before writing each role's review section, then
///              proceeds without holding it in context (sequential read-write-proceed).
fn kiro_inclusion(filename: &str) -> KiroInclusion {
    match filename {
        // ── Always ──────────────────────────────────────────────────────────
        // project-context: core project knowledge, referenced across all tasks
        // 00-loading-guide: tells the agent how to load the rest of the system
        "project-context.md" | "00-loading-guide.md" => KiroInclusion::Always,

        // ── Auto ────────────────────────────────────────────────────────────
        // Descriptions are tuned for Kiro's semantic matching. They must be
        // concrete enough to match review-related requests without triggering
        // on unrelated coding sessions.
        "prep-review.md" => KiroInclusion::Auto {
            name: "prep-review",
            description: "prepare code review materials, pack context with repomix, \
                          run review workflow, tạo materials để review, prep review",
        },
        "pack-materials.md" => KiroInclusion::Auto {
            name: "pack-materials",
            description: "build review_context.xml, repomix commands, \
                          pack code context for AI review upload",
        },
        "review-roles.md" => KiroInclusion::Auto {
            name: "review-roles",
            description: "activate code reviewer roles and perspectives, \
                          role trigger conditions for multi-role code review",
        },
        "review-prompting.md" => KiroInclusion::Auto {
            name: "review-prompting",
            description: "structure multi-file review output, role standards loading protocol, \
                          severity labels, verdict format, temp directory protocol",
        },
        "review-checklist.md" => KiroInclusion::Auto {
            name: "review-checklist",
            description: "pre-review quality checklist, review submission gates",
        },

        // ── Manual ──────────────────────────────────────────────────────────
        // Everything else: loaded on demand via #filename or /<name> slash command.
        // This includes all role standards (00-loading-guide is always, but the
        // individual role standards are manual — loaded one-at-a-time during review).
        _ => KiroInclusion::Manual,
    }
}

/// Build Kiro YAML frontmatter for the given filename and content.
fn kiro_frontmatter(filename: &str, content: &str) -> String {
    let name = filename.trim_end_matches(".md");
    match kiro_inclusion(filename) {
        KiroInclusion::Always => format!("---\ninclusion: always\nname: {name}\n---\n\n{content}"),
        KiroInclusion::Auto {
            name: auto_name,
            description,
        } => format!(
            "---\ninclusion: auto\nname: {auto_name}\ndescription: {description}\n---\n\n{content}"
        ),
        KiroInclusion::Manual => format!("---\ninclusion: manual\nname: {name}\n---\n\n{content}"),
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
        TransformOutput {
            filename: file.name.clone(),
            content: kiro_frontmatter(&file.name, file.raw),
            manual_only: false,
        }
    }

    fn transform_workspace(&self, file: &ContentFile) -> TransformOutput {
        TransformOutput {
            filename: file.name.clone(),
            content: kiro_frontmatter(&file.name, file.raw),
            manual_only: false,
        }
    }

    /// Role standards use `inclusion: manual` so Kiro never auto-loads them.
    ///
    /// Exception: `00-loading-guide.md` uses `always` — it's the master protocol
    /// doc that tells the agent how to load all other standards. It must be present
    /// in every session so the agent knows the sequential read-write-proceed pattern.
    ///
    /// All other standards (01–15) are manual: the agent loads exactly one
    /// standard file immediately before writing that role's output section, then
    /// proceeds without holding it in context.
    fn transform_role_standard(&self, file: &ContentFile) -> TransformOutput {
        TransformOutput {
            filename: file.name.clone(),
            content: kiro_frontmatter(&file.name, file.raw),
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

    #[test]
    fn review_roles_gets_auto_inclusion() {
        let k = Kiro::new();
        let out = k.transform_global(&global_file());
        assert!(
            out.content.contains("inclusion: auto"),
            "review-roles.md must be auto"
        );
        assert!(out.content.contains("name: review-roles"));
        assert!(out.content.contains("description:"));
    }

    #[test]
    fn project_context_gets_always_inclusion() {
        let k = Kiro::new();
        let file = ContentFile {
            name: "project-context.md".into(),
            scope: ContentScope::Workspace,
            raw: "# Project",
        };
        let out = k.transform_workspace(&file);
        assert!(out.content.contains("inclusion: always"));
        assert!(!out.content.contains("description:"));
    }

    #[test]
    fn review_ignore_gets_manual_inclusion() {
        let k = Kiro::new();
        let file = ContentFile {
            name: "review-ignore.md".into(),
            scope: ContentScope::Global,
            raw: "# Ignore",
        };
        let out = k.transform_global(&file);
        assert!(out.content.contains("inclusion: manual"));
        assert!(!out.content.contains("description:"));
    }

    #[test]
    fn prep_review_gets_auto_with_description() {
        let k = Kiro::new();
        let file = ContentFile {
            name: "prep-review.md".into(),
            scope: ContentScope::Workspace,
            raw: "# Prep",
        };
        let out = k.transform_workspace(&file);
        assert!(out.content.contains("inclusion: auto"));
        assert!(out.content.contains("name: prep-review"));
        assert!(out.content.contains("repomix"));
    }

    #[test]
    fn role_standards_get_manual_inclusion() {
        let k = Kiro::new();
        let file = ContentFile {
            name: "01-swe-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: "# SWE checklist",
        };
        let out = k.transform_role_standard(&file);
        assert!(
            out.content.contains("inclusion: manual"),
            "role standards must be manual"
        );
        assert!(out.content.contains("name: 01-swe-standard"));
        assert!(!out.content.contains("always"));
        assert!(!out.content.contains("agent-requested"));
        assert_eq!(out.filename, "01-swe-standard.md");
        assert!(!out.manual_only);
    }

    #[test]
    fn loading_guide_gets_always_inclusion() {
        let k = Kiro::new();
        let file = ContentFile {
            name: "00-loading-guide.md".into(),
            scope: ContentScope::RoleStandard,
            raw: "# Loading Guide",
        };
        let out = k.transform_role_standard(&file);
        assert!(
            out.content.contains("inclusion: always"),
            "00-loading-guide must be always"
        );
    }

    #[test]
    fn kiro_workspace_dir() {
        let k = Kiro::new();
        assert_eq!(
            k.workspace_dir(Path::new("/project")),
            PathBuf::from("/project/.kiro/steering")
        );
    }

    #[test]
    fn kiro_global_dir_uses_home_override() {
        let mut k = Kiro::new();
        k.home.override_path = Some(PathBuf::from("/fake/home"));
        assert_eq!(
            k.global_dir().unwrap(),
            PathBuf::from("/fake/home/.kiro/steering")
        );
    }
}
