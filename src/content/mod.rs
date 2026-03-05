pub mod transform;

/// Scope of a content file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentScope {
    Global,
    Workspace,
    Template,
    /// Lazy-loaded role standards. Installed into `role-standards/` under the
    /// workspace directory. Agents load exactly one at a time, immediately
    /// before writing the corresponding review file, rather than pre-loading
    /// all of them at context startup.
    RoleStandard,
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
        ContentFile {
            name: "review-best-practices.md".into(),
            scope: ContentScope::Workspace,
            raw: include_str!("../../content/workspace/review-best-practices.md"),
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
        // Role Standards (lazy-loaded — one at a time, immediately before writing each review file)
        ContentFile {
            name: "00-loading-guide.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/00-loading-guide.md"),
        },
        ContentFile {
            name: "01-swe-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/01-swe-standard.md"),
        },
        ContentFile {
            name: "02-sa-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/02-sa-standard.md"),
        },
        ContentFile {
            name: "03-qa-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/03-qa-standard.md"),
        },
        ContentFile {
            name: "04-pe-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/04-pe-standard.md"),
        },
        ContentFile {
            name: "05-se-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/05-se-standard.md"),
        },
        ContentFile {
            name: "06-oe-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/06-oe-standard.md"),
        },
        ContentFile {
            name: "07-de-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/07-de-standard.md"),
        },
        ContentFile {
            name: "08-ux-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/08-ux-standard.md"),
        },
        ContentFile {
            name: "09-cl-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/09-cl-standard.md"),
        },
        ContentFile {
            name: "10-ceo-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/10-ceo-standard.md"),
        },
        ContentFile {
            name: "11-da-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/11-da-standard.md"),
        },
        ContentFile {
            name: "12-mle-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/12-mle-standard.md"),
        },
        ContentFile {
            name: "13-api-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/13-api-standard.md"),
        },
        ContentFile {
            name: "14-finops-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/14-finops-standard.md"),
        },
        ContentFile {
            name: "15-dx-standard.md".into(),
            scope: ContentScope::RoleStandard,
            raw: include_str!("../../content/role-standards/15-dx-standard.md"),
        },
    ]
}

pub fn by_scope(scope: ContentScope) -> Vec<ContentFile> {
    all_content()
        .into_iter()
        .filter(|f| f.scope == scope)
        .collect()
}

pub fn by_name(name: &str) -> Option<ContentFile> {
    all_content().into_iter().find(|f| f.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_content_returns_29_files() {
        // 6 global + 5 workspace + 2 templates + 16 role-standards = 29
        assert_eq!(all_content().len(), 29);
    }

    #[test]
    fn by_scope_global_returns_6() {
        assert_eq!(by_scope(ContentScope::Global).len(), 6);
    }

    #[test]
    fn by_scope_workspace_returns_5() {
        assert_eq!(by_scope(ContentScope::Workspace).len(), 5);
    }

    #[test]
    fn by_scope_template_returns_2() {
        assert_eq!(by_scope(ContentScope::Template).len(), 2);
    }

    #[test]
    fn by_scope_role_standard_returns_16() {
        // 00-loading-guide + 15 role standards (01–15)
        assert_eq!(by_scope(ContentScope::RoleStandard).len(), 16);
    }

    #[test]
    fn role_standards_include_all_roles() {
        let standards = by_scope(ContentScope::RoleStandard);
        let names: Vec<&str> = standards.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"01-swe-standard.md"));
        assert!(names.contains(&"05-se-standard.md"));
        assert!(names.contains(&"12-mle-standard.md"));
        assert!(names.contains(&"15-dx-standard.md"));
    }

    #[test]
    fn all_content_is_non_empty() {
        for file in all_content() {
            assert!(
                !file.raw.is_empty(),
                "Content file '{}' is empty",
                file.name
            );
        }
    }

    #[test]
    fn by_name_found() {
        assert!(by_name("review-roles.md").is_some());
    }

    #[test]
    fn by_name_role_standard_found() {
        assert!(by_name("01-swe-standard.md").is_some());
        assert!(by_name("00-loading-guide.md").is_some());
    }

    #[test]
    fn by_name_not_found() {
        assert!(by_name("nope").is_none());
    }
}
