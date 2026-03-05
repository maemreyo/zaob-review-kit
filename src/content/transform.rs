/// Filter agent-specific tagged sections from content.
///
/// Content files can mark agent-specific blocks using HTML comment tags:
///
/// ```markdown
/// <!-- agent:kiro:start -->
/// > **Kiro:** heredoc patterns hang the terminal tool — always pipe directly.
/// <!-- agent:kiro:end -->
/// ```
///
/// Sections tagged for a *different* agent are stripped (tag lines included).
/// Untagged content is always kept. Tags are matched by exact agent name so
/// `kiro` blocks are kept by Kiro and stripped by Cursor, Windsurf, etc.
///
/// The filter is line-oriented and handles nested/stacked blocks correctly
/// using a skip-depth counter.
pub fn filter_agent_sections(content: &str, agent_name: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut skip_depth: usize = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(tag_agent) = parse_agent_start(trimmed) {
            if tag_agent != agent_name {
                skip_depth += 1;
            }
            // always drop the tag line itself
            continue;
        }

        if parse_agent_end(trimmed).is_some() {
            if skip_depth > 0 {
                skip_depth -= 1;
            }
            // always drop the tag line itself
            continue;
        }

        if skip_depth == 0 {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

fn parse_agent_start(line: &str) -> Option<&str> {
    // <!-- agent:NAME:start -->
    let inner = line.strip_prefix("<!-- agent:")?.strip_suffix(" -->")?;
    inner.strip_suffix(":start")
}

fn parse_agent_end(line: &str) -> Option<&str> {
    // <!-- agent:NAME:end -->
    let inner = line.strip_prefix("<!-- agent:")?.strip_suffix(" -->")?;
    inner.strip_suffix(":end")
}

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
    format!("<!-- trigger: {} -->\n\n{}", trigger, content)
}

/// Identity transform — no frontmatter, no headers.
/// Used by agents that read plain markdown (Antigravity, TRAE).
pub fn as_plain(content: &str) -> String {
    content.to_string()
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
    fn filter_keeps_untagged_content() {
        let content = "# Title\n\nSome content.\n";
        let result = filter_agent_sections(content, "kiro");
        assert_eq!(result, "# Title\n\nSome content.\n");
    }

    #[test]
    fn filter_keeps_matching_agent_block() {
        let content =
            "before\n<!-- agent:kiro:start -->\nkiro-only\n<!-- agent:kiro:end -->\nafter\n";
        let result = filter_agent_sections(content, "kiro");
        assert!(result.contains("kiro-only"));
        assert!(result.contains("before"));
        assert!(result.contains("after"));
        assert!(!result.contains("<!-- agent:"));
    }

    #[test]
    fn filter_strips_other_agent_block() {
        let content =
            "before\n<!-- agent:kiro:start -->\nkiro-only\n<!-- agent:kiro:end -->\nafter\n";
        let result = filter_agent_sections(content, "cursor");
        assert!(!result.contains("kiro-only"));
        assert!(result.contains("before"));
        assert!(result.contains("after"));
        assert!(!result.contains("<!-- agent:"));
    }

    #[test]
    fn filter_strips_tag_lines_for_matching_agent() {
        let content = "<!-- agent:kiro:start -->\ncontent\n<!-- agent:kiro:end -->\n";
        let result = filter_agent_sections(content, "kiro");
        assert!(!result.contains("<!-- agent:"));
        assert!(result.contains("content"));
    }

    #[test]
    fn filter_handles_multiple_agent_blocks() {
        let content = concat!(
            "shared\n",
            "<!-- agent:kiro:start -->\nkiro-note\n<!-- agent:kiro:end -->\n",
            "also-shared\n",
            "<!-- agent:cursor:start -->\ncursor-note\n<!-- agent:cursor:end -->\n",
            "end\n",
        );
        let kiro_result = filter_agent_sections(content, "kiro");
        assert!(kiro_result.contains("kiro-note"));
        assert!(!kiro_result.contains("cursor-note"));
        assert!(kiro_result.contains("shared"));
        assert!(kiro_result.contains("also-shared"));

        let cursor_result = filter_agent_sections(content, "cursor");
        assert!(!cursor_result.contains("kiro-note"));
        assert!(cursor_result.contains("cursor-note"));
    }

    #[test]
    fn filter_preserves_code_blocks_outside_tagged_sections() {
        let content = "```bash\ngit diff | repomix --stdin\n```\n";
        let result = filter_agent_sections(content, "cursor");
        assert_eq!(result, content);
    }

    #[test]
    fn yaml_frontmatter_structure() {
        let result = wrap_yaml_frontmatter(
            "review-roles.md",
            "Review roles for code review",
            "# Roles\nContent here",
        );
        assert!(result.starts_with("---\n"));
        assert!(result.contains("description: Review roles for code review"));
        assert!(result.contains("name: review-roles"));
        assert!(result.contains("# Roles\nContent here"));
    }

    #[test]
    fn mdc_frontmatter_structure() {
        let result =
            wrap_mdc_frontmatter("review-roles.mdc", "Review roles", "**/*.rs", "# Content");
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
    fn plain_transform_is_identity() {
        let content = "# Test\n\nSome content.";
        assert_eq!(as_plain(content), content);
    }

    #[test]
    fn change_extension_md_to_mdc() {
        assert_eq!(
            change_extension("review-roles.md", "mdc"),
            "review-roles.mdc"
        );
    }

    #[test]
    fn change_extension_no_ext() {
        assert_eq!(change_extension("archignore", "md"), "archignore.md");
    }
}
