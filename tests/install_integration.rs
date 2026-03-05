use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn zrk() -> Command {
    Command::cargo_bin("zrk").unwrap()
}

#[test]
fn install_creates_workspace_files() {
    let dir = TempDir::new().unwrap();
    zrk()
        .args([
            "install",
            "--target",
            "kiro",
            "--cwd",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    let steering = dir.path().join(".kiro").join("steering");
    assert!(steering.join("prep-review.md").exists());
    assert!(steering.join("pack-materials.md").exists());
    assert!(steering.join("project-context.md").exists());

    // Role standards installed in subdirectory
    let role_standards = steering.join("role-standards");
    assert!(
        role_standards.exists(),
        "role-standards/ dir should be created"
    );
    assert!(role_standards.join("01-swe-standard.md").exists());
    assert!(role_standards.join("05-se-standard.md").exists());
    assert!(role_standards.join("00-loading-guide.md").exists());
}

#[test]
fn install_all_creates_workspace_and_templates() {
    let dir = TempDir::new().unwrap();
    zrk()
        .args([
            "install-all",
            "--target",
            "kiro",
            "--cwd",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    // Workspace files
    let steering = dir.path().join(".kiro").join("steering");
    assert!(steering.join("prep-review.md").exists());

    // Templates
    assert!(dir.path().join(".archignore").exists());

    // reports/ top-level directory created for storing finished reviews
    assert!(
        dir.path().join("reports").exists(),
        "reports/ directory must be created by install-all"
    );
}

#[test]
fn dry_run_creates_no_files() {
    let dir = TempDir::new().unwrap();
    zrk()
        .args([
            "install-all",
            "--target",
            "kiro",
            "--cwd",
            dir.path().to_str().unwrap(),
            "--dry-run",
        ])
        .assert()
        .success();

    // Nothing should be created
    assert!(!dir.path().join(".kiro").exists());
    assert!(!dir.path().join(".archignore").exists());
}

#[test]
fn update_overwrites_existing_files() {
    let dir = TempDir::new().unwrap();

    // First install
    zrk()
        .args([
            "install",
            "--target",
            "kiro",
            "--cwd",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    let file = dir
        .path()
        .join(".kiro")
        .join("steering")
        .join("prep-review.md");
    std::fs::write(&file, "modified content").unwrap();

    // Update (force reinstall)
    zrk()
        .args([
            "update",
            "--target",
            "kiro",
            "--cwd",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = std::fs::read_to_string(&file).unwrap();
    assert!(content.contains("---\n")); // Should have frontmatter again, not "modified content"
}

#[test]
fn install_all_with_all_targets() {
    let dir = TempDir::new().unwrap();
    zrk()
        .args([
            "install-all",
            "--all-targets",
            "--cwd",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    // All 6 agents should have workspace dirs
    assert!(dir.path().join(".kiro").join("steering").exists());
    assert!(dir
        .path()
        .join(".claude")
        .join("commands")
        .join("review-kit")
        .exists());
    assert!(dir.path().join(".cursor").join("rules").exists());
    assert!(dir.path().join(".windsurf").join("rules").exists());
    assert!(dir.path().join(".agent").join("rules").exists());
    assert!(dir.path().join(".trae").join("rules").exists());
}

#[test]
fn unknown_agent_returns_error() {
    let dir = TempDir::new().unwrap();
    zrk()
        .args([
            "install",
            "--target",
            "vscode",
            "--cwd",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown agent"));
}
