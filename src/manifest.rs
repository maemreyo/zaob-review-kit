use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Tracks installed files and their content hashes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub agent: String,
    pub files: BTreeMap<String, String>, // relative path → SHA-256 hash
}

impl Manifest {
    pub fn new(agent: &str) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            agent: agent.to_string(),
            files: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, relative_path: &str, content: &str) {
        let hash = sha256_hex(content);
        self.files.insert(relative_path.to_string(), hash);
    }

    pub fn manifest_path(base_dir: &Path) -> PathBuf {
        base_dir.join(".zrk-manifest.json")
    }

    pub fn save(&self, base_dir: &Path) -> Result<(), std::io::Error> {
        let path = Self::manifest_path(base_dir);
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(base_dir: &Path) -> Result<Option<Self>, std::io::Error> {
        let path = Self::manifest_path(base_dir);
        if !path.exists() {
            return Ok(None);
        }
        let json = std::fs::read_to_string(path)?;
        let manifest: Self = serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(manifest))
    }
}

pub fn sha256_hex(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_roundtrip() {
        let mut m = Manifest::new("kiro");
        m.add_file("review-roles.md", "# Roles");
        m.add_file("prep-review.md", "# Prep");

        let json = serde_json::to_string_pretty(&m).unwrap();
        let m2: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(m, m2);
    }

    #[test]
    fn manifest_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let mut m = Manifest::new("cursor");
        m.add_file("test.mdc", "content");
        m.save(dir.path()).unwrap();

        let loaded = Manifest::load(dir.path()).unwrap().unwrap();
        assert_eq!(loaded, m);
    }

    #[test]
    fn manifest_load_missing() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = Manifest::load(dir.path()).unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn sha256_deterministic() {
        let h1 = sha256_hex("hello");
        let h2 = sha256_hex("hello");
        assert_eq!(h1, h2);
        assert_ne!(sha256_hex("hello"), sha256_hex("world"));
    }
}
