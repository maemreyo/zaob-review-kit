/// Wrap content with YAML frontmatter (for Kiro, Claude Code).
pub fn wrap_yaml_frontmatter(filename: &str, description: &str, content: &str) -> String {
    format!(
        "---\ndescription: {}\nglobs: \nname: {}\n---\n\n{}",
        description,
        filename.trim_end_matches(".md"),
        content
    )
}

/// Wrap content with MDC frontmatter (for Cursor .mdc files).
pub fn wrap_mdc_frontmatter(
    filename: &str,
    description: &str,
    globs: &str,
    content: &str,
) -> String {
    format!(
        "---\ndescription: {}\nglobs: {}\nname: {}\n---\n\n{}",
        description,
        globs,
        filename.trim_end_matches(".mdc").trim_end_matches(".md"),
        content
    )
}

/// Wrap content with HTML comment header (for Windsurf).
pub fn wrap_comment_header(trigger: &str, content: &str) -> String {
    format!(
        "<!-- trigger: {} -->\n\n{}",
        trigger, content
    )
}

/// Change the extension of a filename.
pub fn change_extension(filename: &str, new_ext: &str) -> String {
    match filename.rfind('.') {
        Some(pos) => format!("{}.{}", &filename[..pos], new_ext),
        None => format!("{}.{}", filename, new_ext),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_frontmatter_structure() {
        let result = wrap_yaml_frontmatter("review-roles.md", "Review roles for code review", "# Roles\nContent here");
        assert!(result.starts_with("---\n"));
        assert!(result.contains("description: Review roles for code review"));
        assert!(result.contains("name: review-roles"));
        assert!(result.contains("# Roles\nContent here"));
    }

    #[test]
    fn mdc_frontmatter_structure() {
        let result = wrap_mdc_frontmatter("review-roles.mdc", "Review roles", "**/*.rs", "# Content");
        assert!(result.starts_with("---\n"));
        assert!(result.contains("description: Review roles"));
        assert!(result.contains("globs: **/*.rs"));
        assert!(result.contains("name: review-roles"));
        assert!(result.contains("# Content"));
    }

    #[test]
    fn comment_header_structure() {
        let result = wrap_comment_header("review-roles", "# Content");
        assert!(result.starts_with("<!-- trigger: review-roles -->"));
        assert!(result.contains("# Content"));
    }

    #[test]
    fn change_extension_md_to_mdc() {
        assert_eq!(change_extension("review-roles.md", "mdc"), "review-roles.mdc");
    }

    #[test]
    fn change_extension_no_ext() {
        assert_eq!(change_extension("archignore", "md"), "archignore.md");
    }
}
