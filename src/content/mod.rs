pub mod transform;

/// Scope of a content file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentScope {
    Global,
    Workspace,
    Template,
}

/// A content file embedded at compile time.
#[derive(Debug, Clone)]
pub struct ContentFile {
    pub name: String,
    pub scope: ContentScope,
    pub raw: &'static str,
}

pub fn all_content() -> Vec<ContentFile> {
    vec![
        // Global
        ContentFile {
            name: "review-roles.md".into(),
            scope: ContentScope::Global,
            raw: include_str!("../../content/global/review-roles.md"),
        },
        ContentFile {
            name: "review-prompting.md".into(),
            scope: ContentScope::Global,
            raw: include_str!("../../content/global/review-prompting.md"),
        },
        ContentFile {
            name: "review-ignore.md".into(),
            scope: ContentScope::Global,
            raw: include_str!("../../content/global/review-ignore.md"),
        },
        ContentFile {
            name: "review-memory.md".into(),
            scope: ContentScope::Global,
            raw: include_str!("../../content/global/review-memory.md"),
        },
        ContentFile {
            name: "review-security.md".into(),
            scope: ContentScope::Global,
            raw: include_str!("../../content/global/review-security.md"),
        },
        ContentFile {
            name: "review-performance.md".into(),
            scope: ContentScope::Global,
            raw: include_str!("../../content/global/review-performance.md"),
        },
        // Workspace
        ContentFile {
            name: "prep-review.md".into(),
            scope: ContentScope::Workspace,
            raw: include_str!("../../content/workspace/prep-review.md"),
        },
        ContentFile {
            name: "pack-materials.md".into(),
            scope: ContentScope::Workspace,
            raw: include_str!("../../content/workspace/pack-materials.md"),
        },
        ContentFile {
            name: "project-context.md".into(),
            scope: ContentScope::Workspace,
            raw: include_str!("../../content/workspace/project-context.md"),
        },
        ContentFile {
            name: "review-checklist.md".into(),
            scope: ContentScope::Workspace,
            raw: include_str!("../../content/workspace/review-checklist.md"),
        },
        // Templates
        ContentFile {
            name: "archignore".into(),
            scope: ContentScope::Template,
            raw: include_str!("../../content/templates/archignore"),
        },
        ContentFile {
            name: "gitignore-snippet.txt".into(),
            scope: ContentScope::Template,
            raw: include_str!("../../content/templates/gitignore-snippet.txt"),
        },
    ]
}

pub fn by_scope(scope: ContentScope) -> Vec<ContentFile> {
    all_content().into_iter().filter(|f| f.scope == scope).collect()
}

pub fn by_name(name: &str) -> Option<ContentFile> {
    all_content().into_iter().find(|f| f.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_content_returns_12_files() {
        assert_eq!(all_content().len(), 12);
    }

    #[test]
    fn by_scope_global_returns_6() {
        assert_eq!(by_scope(ContentScope::Global).len(), 6);
    }

    #[test]
    fn by_scope_workspace_returns_4() {
        assert_eq!(by_scope(ContentScope::Workspace).len(), 4);
    }

    #[test]
    fn by_scope_template_returns_2() {
        assert_eq!(by_scope(ContentScope::Template).len(), 2);
    }

    #[test]
    fn all_content_is_non_empty() {
        for file in all_content() {
            assert!(!file.raw.is_empty(), "Content file '{}' is empty", file.name);
        }
    }

    #[test]
    fn by_name_found() {
        assert!(by_name("review-roles.md").is_some());
    }

    #[test]
    fn by_name_not_found() {
        assert!(by_name("nope").is_none());
    }
}
