use std::path::PathBuf;

/// Shared home directory resolver for agents that need `~`-based config paths.
pub struct HomeResolver {
    pub override_path: Option<PathBuf>,
}

impl HomeResolver {
    pub fn new() -> Self {
        Self { override_path: None }
    }

    pub fn resolve(&self) -> Option<PathBuf> {
        self.override_path.clone().or_else(dirs::home_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_resolver_uses_override() {
        let r = HomeResolver { override_path: Some(PathBuf::from("/custom/home")) };
        assert_eq!(r.resolve(), Some(PathBuf::from("/custom/home")));
    }

    #[test]
    fn home_resolver_falls_back_to_dirs() {
        let r = HomeResolver::new();
        // Just verify it returns something (real home dir on the machine).
        assert!(r.resolve().is_some());
    }
}
