use super::claude_code::ClaudeCode;
use super::cursor::Cursor;
use super::kiro::Kiro;
use super::windsurf::Windsurf;
use super::Agent;

pub fn all_agents() -> Vec<Box<dyn Agent>> {
    vec![
        Box::new(Kiro::new()),
        Box::new(ClaudeCode::new()),
        Box::new(Cursor::new()),
        Box::new(Windsurf::new()),
    ]
}

pub fn get_agent(name: &str) -> Option<Box<dyn Agent>> {
    all_agents().into_iter().find(|a| a.name() == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_4_agents() {
        assert_eq!(all_agents().len(), 4);
    }

    #[test]
    fn lookup_kiro() {
        assert!(get_agent("kiro").is_some());
    }

    #[test]
    fn lookup_claude_code() {
        assert!(get_agent("claude-code").is_some());
    }

    #[test]
    fn lookup_cursor() {
        assert!(get_agent("cursor").is_some());
    }

    #[test]
    fn lookup_windsurf() {
        assert!(get_agent("windsurf").is_some());
    }

    #[test]
    fn lookup_unknown() {
        assert!(get_agent("vscode").is_none());
    }
}
