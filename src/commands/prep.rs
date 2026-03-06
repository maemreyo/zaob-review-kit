use crate::content;
use crate::error::ZrkError;
use crate::util::{fs::write_file_safe, output};
use crate::Cli;
use std::path::{Path, PathBuf};
use std::process::Command;

// ── Clap args ─────────────────────────────────────────────────────────────────

/// Arguments for `zrk prep`.
///
/// Three input modes (mutually exclusive by priority):
///   1. `--topic KEYWORD`               — ripgrep content search
///   2. `zrk prep HEAD~3..HEAD`         — git range (single arg containing "..")
///   3. `zrk prep abc123 def456 ...`    — space-separated commit hashes
///
/// Use `--include` to add extra files to the repomix context (e.g. docs, spec files
/// the user referenced in their request).
#[derive(clap::Args, Debug)]
pub struct PrepArgs {
    /// Git range (HEAD~3..HEAD), commit hashes (abc123 def456), or use --topic
    #[arg(value_name = "SCOPE")]
    pub scope: Vec<String>,

    /// Content search topic: finds files via ripgrep instead of git diff
    #[arg(long, value_name = "KEYWORD")]
    pub topic: Option<String>,

    /// Extra files to include in review_context.xml beyond the changed files
    /// (e.g. architecture docs, spec files, referenced context)
    #[arg(long = "include", value_name = "FILE", num_args = 1..)]
    pub extra_files: Vec<String>,

    /// Documentation folders to scan and include (all .md files, excluding reviews/)
    /// e.g. --docs docs/architecture/v1 docs/tdd
    #[arg(long = "docs", value_name = "DIR", num_args = 1..)]
    pub doc_dirs: Vec<String>,
}

// ── Scope model ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum PrepScope {
    /// A git range string, e.g. "HEAD~3..HEAD" or "feature/auth..main"
    GitRange(String),
    /// One or more non-contiguous commit hashes
    CommitHashes(Vec<String>),
    /// Ripgrep topic search
    Topic(String),
}

impl PrepScope {
    fn description(&self) -> String {
        match self {
            PrepScope::GitRange(r) => format!("git range `{}`", r),
            PrepScope::CommitHashes(h) if h.len() == 1 => format!("commit `{}`", h[0]),
            PrepScope::CommitHashes(h) => {
                format!("{} commits: {}", h.len(), h.join(", "))
            }
            PrepScope::Topic(t) => format!("topic search: \"{}\"", t),
        }
    }
}

fn parse_scope(args: &PrepArgs) -> Result<PrepScope, ZrkError> {
    if let Some(t) = &args.topic {
        return Ok(PrepScope::Topic(t.clone()));
    }
    match args.scope.len() {
        0 => Err(ZrkError::Prep(
            "No scope provided.\n\n  \
             Examples:\n    \
             zrk prep HEAD~3..HEAD\n    \
             zrk prep feature/auth..main\n    \
             zrk prep abc123 def456 ghi789\n    \
             zrk prep --topic \"phase-0\""
                .into(),
        )),
        1 if args.scope[0].contains("..") => Ok(PrepScope::GitRange(args.scope[0].clone())),
        _ => Ok(PrepScope::CommitHashes(args.scope.clone())),
    }
}

// ── Role trigger definitions ──────────────────────────────────────────────────

pub struct RoleTrigger {
    pub id: &'static str,
    pub code: &'static str,
    pub standard: &'static str,
    pub label: &'static str,
    /// Lowercase path fragment patterns. Any match in any changed file triggers the role.
    /// Empty slice = no auto-detection (role must be requested manually).
    pub patterns: &'static [&'static str],
}

/// Triggerable roles in activation order (01/02/03 core are always included separately).
static ROLE_TRIGGERS: &[RoleTrigger] = &[
    RoleTrigger {
        id: "04",
        code: "pe",
        standard: "04-pe-standard.md",
        label: "Performance Engineer",
        patterns: &["migration", ".sql", "query", "queries", "async", "cache", "index"],
    },
    RoleTrigger {
        id: "05",
        code: "se",
        standard: "05-se-standard.md",
        label: "Security Engineer",
        patterns: &[
            "auth", "login", "token", "permission", "session", "secret", "password",
            "crypto", "jwt", "oauth", "rbac",
        ],
    },
    RoleTrigger {
        id: "06",
        code: "oe",
        standard: "06-oe-standard.md",
        label: "Operations Engineer",
        patterns: &[
            "hand.toml", ".env", "cron", "docker", "compose", "deploy",
            "k8s", "helm", "terraform", "systemd", "supervisor", "nginx",
            "justfile", "makefile", "ci.yml", "workflow.yml",
        ],
    },
    RoleTrigger {
        id: "07",
        code: "de",
        standard: "07-de-standard.md",
        label: "Database Engineer",
        patterns: &["migration", "schema", ".sql", "seed", "database", "db_", "_db.", "diesel", "sqlx", "prisma"],
    },
    RoleTrigger {
        id: "08",
        code: "ux",
        standard: "08-ux-standard.md",
        label: "Frontend / UX Engineer",
        patterns: &[
            ".vue", ".tsx", ".jsx", ".css", ".scss", "component", "page",
            "view", "style", ".html",
        ],
    },
    RoleTrigger {
        id: "09",
        code: "cl",
        standard: "09-cl-standard.md",
        label: "Compliance Engineer",
        patterns: &["gdpr", "pii", "cookie", "consent", "privacy", "compliance", "licence", "license"],
    },
    RoleTrigger {
        id: "10",
        code: "ceo",
        standard: "10-ceo-standard.md",
        label: "CEO / Business",
        patterns: &["changelog", "release", "breaking", "api/v"],
    },
    RoleTrigger {
        id: "11",
        code: "da",
        standard: "11-da-standard.md",
        label: "Devil's Advocate",
        patterns: &[], // no auto-detect — add via review_prompt.md ## Additional Roles
    },
    RoleTrigger {
        id: "12",
        code: "mle",
        standard: "12-mle-standard.md",
        label: "ML / AI Engineer",
        patterns: &["/llm", "_llm", "llm_", "llm.", "embedding", "dataset", "inference", "openai", "anthropic", "vector_store", "torch", "tensorflow", "ml_model"],
    },
    RoleTrigger {
        id: "13",
        code: "api",
        standard: "13-api-standard.md",
        label: "API Design",
        patterns: &["route", "handler", "endpoint", "openapi", "swagger", "graphql", "grpc"],
    },
    RoleTrigger {
        id: "14",
        code: "finops",
        standard: "14-finops-standard.md",
        label: "FinOps",
        patterns: &[".tf", "terraform", "lambda", "cloudformation", "kubernetes", "k8s", "helm"],
    },
    RoleTrigger {
        id: "15",
        code: "dx",
        standard: "15-dx-standard.md",
        label: "Developer Experience",
        patterns: &["readme", "changelog", "contributing", "docs/", "sdk", "cli"],
    },
];

/// Returns the subset of ROLE_TRIGGERS activated by the given file paths.
/// Core roles (01-03) are always included by the caller — this returns extras only.
pub fn detect_triggered_roles(changed_files: &[String]) -> Vec<&'static RoleTrigger> {
    ROLE_TRIGGERS
        .iter()
        .filter(|t| {
            !t.patterns.is_empty()
                && changed_files.iter().any(|f| {
                    let lower = f.to_lowercase();
                    t.patterns.iter().any(|p| lower.contains(p))
                })
        })
        .collect()
}

/// Returns the first `"file (pattern: pat)"` match for a trigger, for role-plan annotation.
fn first_match_annotation(trigger: &RoleTrigger, changed_files: &[String]) -> String {
    for f in changed_files {
        let lower = f.to_lowercase();
        for p in trigger.patterns {
            if lower.contains(p) {
                return format!("`{}` (pattern: `{}`)", f, p);
            }
        }
    }
    "pattern matched".to_string()
}

// ── Git / rg helpers ──────────────────────────────────────────────────────────

fn git_changed_files_range(range: &str, cwd: &Path) -> Result<Vec<String>, ZrkError> {
    let out = Command::new("git")
        .args(["diff", range, "--name-only"])
        .current_dir(cwd)
        .output()
        .map_err(|e| ZrkError::Prep(format!("git not found: {}", e)))?;

    if !out.status.success() {
        return Err(ZrkError::Prep(format!(
            "git diff {} failed:\n  {}",
            range,
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    Ok(lines_from_bytes(&out.stdout))
}

fn git_changed_files_hashes(hashes: &[String], cwd: &Path) -> Result<Vec<String>, ZrkError> {
    let mut files = std::collections::BTreeSet::new();
    for hash in hashes {
        let out = Command::new("git")
            .args(["show", "--name-only", "--pretty=format:", hash])
            .current_dir(cwd)
            .output()
            .map_err(|e| ZrkError::Prep(format!("git not found: {}", e)))?;

        if !out.status.success() {
            return Err(ZrkError::Prep(format!(
                "git show {} failed:\n  {}",
                hash,
                String::from_utf8_lossy(&out.stderr).trim()
            )));
        }
        for f in lines_from_bytes(&out.stdout) {
            files.insert(f);
        }
    }
    Ok(files.into_iter().collect())
}

fn rg_topic_files(topic: &str, cwd: &Path) -> Result<Vec<String>, ZrkError> {
    let out = Command::new("rg")
        .args(["-l", topic])
        .current_dir(cwd)
        .output()
        .map_err(|_| ZrkError::Prep(
            "ripgrep (rg) not found.\n  Install: brew install ripgrep  /  apt install ripgrep".into(),
        ))?;
    // rg exits 1 when no matches — that is not an error
    Ok(lines_from_bytes(&out.stdout))
}

fn expand_doc_dirs(dirs: &[String], cwd: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for dir in dirs {
        let path = cwd.join(dir);
        if path.is_dir() {
            collect_md_files(&path, &mut files, cwd);
        }
    }
    files
}

fn collect_md_files(dir: &Path, out: &mut Vec<String>, cwd: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let p = entry.path();
        // Skip reviews/ subdirectories — stale analysis inflates token budget
        if p.is_dir() {
            if p.file_name().map_or(false, |n| n == "reviews") {
                continue;
            }
            collect_md_files(&p, out, cwd);
        } else if p.extension().map_or(false, |e| e == "md") {
            if let Ok(rel) = p.strip_prefix(cwd) {
                out.push(rel.to_string_lossy().into_owned());
            }
        }
    }
}

fn lines_from_bytes(bytes: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}

fn command_available(cmd: &str) -> bool {
    // Try "which" (Unix) then "where" (Windows)
    Command::new("which").arg(cmd).output().map(|o| o.status.success()).unwrap_or(false)
        || Command::new("where").arg(cmd).output().map(|o| o.status.success()).unwrap_or(false)
}

// ── Repomix & patch ───────────────────────────────────────────────────────────

fn run_repomix(scope: &PrepScope, extra_files: &[String], cwd: &Path, output_path: &Path) -> bool {
    let out = output_path.display().to_string();

    // Build the file-list pipeline segment.
    // If extra files are present, use a group { ...; ...; } so all sources
    // feed into a single sort -u before repomix.
    let (file_list_cmd, needs_group) = match scope {
        PrepScope::GitRange(range) => (
            format!("git diff {} --name-only", range),
            !extra_files.is_empty(),
        ),
        PrepScope::CommitHashes(hashes) => (
            format!("git show {} --name-only --pretty=format:", hashes.join(" ")),
            !extra_files.is_empty(),
        ),
        PrepScope::Topic(topic) => (
            format!("rg -l {}", shell_quote(topic)),
            !extra_files.is_empty(),
        ),
    };

    let repomix_flags = match scope {
        PrepScope::GitRange(range) => {
            let logs = extract_n_commits(range)
                .map(|n| format!(" --include-logs-count {}", n))
                .unwrap_or_default();
            format!("--stdin --include-diffs{} --style xml --output {}", logs, out)
        }
        _ => format!("--stdin --style xml --output {}", out),
    };

    let cmd = if needs_group {
        // { git-cmd; printf 'extra\n' 'files\n'; } | sort -u | repomix ...
        let extra_printf = extra_files
            .iter()
            .map(|f| shell_quote(f))
            .collect::<Vec<_>>()
            .join(" ");
        format!(
            "{{ {}; printf '%s\\n' {}; }} | sort -u | repomix {}",
            file_list_cmd, extra_printf, repomix_flags
        )
    } else {
        format!("{} | repomix {}", file_list_cmd, repomix_flags)
    };

    Command::new("sh")
        .args(["-c", &cmd])
        .current_dir(cwd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn write_patch(scope: &PrepScope, cwd: &Path, patch_path: &Path) -> bool {
    let cmd = match scope {
        PrepScope::GitRange(range) => {
            format!("git diff {} > {}", range, patch_path.display())
        }
        PrepScope::CommitHashes(hashes) => {
            format!("git show {} > {}", hashes.join(" "), patch_path.display())
        }
        PrepScope::Topic(_) => return false, // no patch for topic mode
    };
    Command::new("sh")
        .args(["-c", &cmd])
        .current_dir(cwd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn extract_n_commits(range: &str) -> Option<usize> {
    // HEAD~3..HEAD → 3  |  HEAD~3..HEAD~1 → None
    let before = range.split("..").next()?;
    before.strip_prefix("HEAD~")?.parse().ok()
}

fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

// ── Timestamp / date helpers ──────────────────────────────────────────────────

/// Returns a compact UTC timestamp string: "YYYYMMDD-HHMMSS".
fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let sec = secs % 60;
    let min = (secs / 60) % 60;
    let hour = (secs / 3600) % 24;
    let (y, mo, d) = days_to_date(secs / 86400);
    format!("{:04}{:02}{:02}-{:02}{:02}{:02}", y, mo, d, hour, min, sec)
}

/// Returns "YYYY-MM-DD" for today (UTC).
fn today_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (y, mo, d) = days_to_date(secs / 86400);
    format!("{:04}-{:02}-{:02}", y, mo, d)
}

/// Z algorithm: converts days-since-Unix-epoch to (year, month, day) in UTC.
/// Reference: https://howardhinnant.github.io/date_algorithms.html
pub fn days_to_date(days: u64) -> (u64, u64, u64) {
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let mo = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if mo <= 2 { y + 1 } else { y };
    (y, mo, d)
}

// ── File writers ──────────────────────────────────────────────────────────────

/// Write role standard files (plain content — no agent frontmatter).
/// Always writes core (00-03) + protocol files + triggered standards.
fn write_standards(materials_dir: &Path, triggered: &[&RoleTrigger]) -> Result<(), ZrkError> {
    let standards_dir = materials_dir.join("standards");
    let all = content::all_content();

    let always = [
        "00-loading-guide.md",
        "01-swe-standard.md",
        "02-sa-standard.md",
        "03-qa-standard.md",
    ];

    for name in &always {
        if let Some(f) = all.iter().find(|f| f.name == *name) {
            write_file_safe(&standards_dir.join(&f.name), f.raw)?;
        }
    }

    for t in triggered {
        if let Some(f) = all.iter().find(|f| f.name == t.standard) {
            write_file_safe(&standards_dir.join(&f.name), f.raw)?;
        }
    }

    Ok(())
}

/// Write temp/ stub files: role-plan.md (pre-filled), file-map.md, findings.md.
fn write_temp_stubs(
    materials_dir: &Path,
    scope: &PrepScope,
    triggered: &[&RoleTrigger],
    changed_files: &[String],
) -> Result<(), ZrkError> {
    let temp_dir = materials_dir.join("temp");
    let today = today_date();

    // ── role-plan.md ──
    let mut rows = String::new();
    rows.push_str("| 01  | SWE — Senior Software Engineer | core — always |\n");
    rows.push_str("| 02  | SA  — Software Architect       | core — always |\n");
    rows.push_str("| 03  | QA  — Quality Assurance        | core — always |\n");
    for t in triggered {
        let annotation = first_match_annotation(t, changed_files);
        rows.push_str(&format!(
            "| {}  | {} — {} | {} |\n",
            t.id,
            t.code.to_uppercase(),
            t.label,
            annotation
        ));
    }

    let mut order_parts = vec!["01-swe".to_string(), "02-sa".to_string(), "03-qa".to_string()];
    for t in triggered {
        order_parts.push(format!("{}-{}", t.id, t.code));
    }
    order_parts.push("99-verdict".to_string());
    let execution_order = order_parts.join(" → ");

    let role_plan = format!(
        "# Role Plan\n\
         _Pre-filled by `zrk prep` on {today}. Agent confirms — does not re-derive._\n\
         _Scope: {scope}_\n\n\
         ## Triggered roles\n\n\
         | #   | Role | Trigger |\n\
         |-----|------|---------|\n\
         {rows}\n\
         ## Execution order\n\n\
         {order}\n\n\
         ## Additional roles\n\
         <!-- If review_prompt.md `## Additional Roles` lists extras, add them here -->\n\n\
         ## Skipped roles\n\
         <!-- If review_prompt.md `## Skip Roles` lists suppressions, add them here -->\n",
        today = today,
        scope = scope.description(),
        rows = rows,
        order = execution_order,
    );
    write_file_safe(&temp_dir.join("role-plan.md"), &role_plan)?;

    // ── file-map.md ──
    write_file_safe(
        &temp_dir.join("file-map.md"),
        "# File → Role Map\n\n\
         _Fill while reading diff content — append one row per file immediately\n\
         after reading that file's diff. Do not batch for the end._\n\n\
         | File | Change type | Roles | Key observation |\n\
         |------|-------------|-------|-----------------|\n",
    )?;

    // ── findings.md ──
    write_file_safe(
        &temp_dir.join("findings.md"),
        "# Running Findings Log\n\n\
         _Append one entry per [BLOCKER] or [MAJOR] immediately after writing each\n\
         role file — BEFORE moving to the next role._\n\n\
         _99-verdict.md reads ONLY this file. Never re-read individual role files._\n\n\
         Format:  `[ROLE][SEVERITY] path:line — short description`\n\
         Example: `[SE][BLOCKER] src/auth/handler.rs:34 — IDOR: no tenant check`\n\n\
         <!-- Append findings below this line -->\n",
    )?;

    Ok(())
}

/// Write reports/ stub files: 00-summary.md and 99-verdict.md with scope pre-filled.
fn write_report_stubs(materials_dir: &Path, scope: &PrepScope) -> Result<(), ZrkError> {
    let reports_dir = materials_dir.join("reports");
    let today = today_date();

    write_file_safe(
        &reports_dir.join("00-summary.md"),
        &format!(
            "# Review Summary\n\n\
             **Scope**: {scope}\n\
             **Date**: {today}\n\
             **Risk**: _TBD — fill after all role files are done_\n\
             **Effort**: _TBD_\n\n\
             ## What Changed\n\
             <!-- One paragraph: what changed and why -->\n\n\
             ## File Walkthrough\n\n\
             | File | Change type | What changed | Notes |\n\
             |------|-------------|--------------|-------|\n\
             <!-- Copy from temp/file-map.md and expand the Notes column -->\n\n\
             ## Risk Assessment\n\n\
             **Level**: _TBD_\n\
             **Justification**: <!-- one sentence -->\n\
             **Review Effort**: _[x/5]_\n\n\
             ## Review Files\n\
             <!-- Fill in LAST — links to all generated role files and verdict -->\n",
            scope = scope.description(),
            today = today,
        ),
    )?;

    write_file_safe(
        &reports_dir.join("99-verdict.md"),
        "[← Back to Summary](00-summary.md)\n\n\
         # Verdict\n\n\
         ## Suggested Tests\n\
         <!-- Format per test:\n\
           - **[test name]** (type: unit | integration | e2e)\n\
             Scenario: <one sentence>\n\
             Input / condition: <specific values>\n\
             Expected: <what should happen>\n\
             Catches: <what bug this prevents>\n\
         -->\n\n\
         ## Prioritised Recommendations\n\
         <!-- Synthesised from temp/findings.md — do NOT re-read individual role files -->\n\n\
         ### Blockers (fix before merge)\n\n\
         ### Major (fix soon)\n\n\
         ### Suggestions (optional)\n\n\
         ## Final Verdict\n\
         <!-- Ship / Ship with changes / Needs rework / Needs discussion -->\n\n\
         ---\n\
         *AI-generated review — human sign-off required for any [BLOCKER] or [MAJOR] finding.*\n",
    )?;

    Ok(())
}

/// Generate a self-contained review_prompt.md — no external files needed beyond what is uploaded.
fn write_review_prompt(
    materials_dir: &Path,
    scope: &PrepScope,
    triggered: &[&RoleTrigger],
) -> Result<(), ZrkError> {
    let today = today_date();

    let mut ctx_files = vec![
        "- `review_context.xml` — full source and git diff".to_string(),
        "- `standards/00-loading-guide.md` — loading protocol and severity label reference"
            .to_string(),
        "- `standards/01-swe-standard.md` — SWE checklist (core)".to_string(),
        "- `standards/02-sa-standard.md` — SA checklist (core)".to_string(),
        "- `standards/03-qa-standard.md` — QA checklist (core)".to_string(),
    ];
    for t in triggered {
        ctx_files.push(format!(
            "- `standards/{file}` — {label} checklist (auto-triggered)",
            file = t.standard,
            label = t.label,
        ));
    }
    ctx_files.push(
        "- `temp/role-plan.md` — pre-decided execution order (confirm, do not re-derive)"
            .to_string(),
    );
    ctx_files.push(
        "- `temp/file-map.md` — file-to-role mapping from prep (fill in during review)"
            .to_string(),
    );
    ctx_files.push(
        "- `temp/findings.md` — append every [BLOCKER] and [MAJOR] here after each role file"
            .to_string(),
    );

    let mut order_parts = vec!["01-swe".to_string(), "02-sa".to_string(), "03-qa".to_string()];
    for t in triggered {
        order_parts.push(format!("{}-{}", t.id, t.code));
    }
    order_parts.push("99-verdict".to_string());
    let suggested_order = order_parts.join(" → ");

    let prompt = format!(
        "# Code Review Request\n\n\
         I want you to perform a structured multi-role code review so that every \
         significant risk — correctness, security, architecture, performance — is \
         surfaced with actionable findings before this code is merged.\n\n\
         ## Context Files\n\n\
         {ctx}\n\n\
         ## Scope\n\n\
         **Input**: {scope}\n\
         **Date**: {today}\n\
         **Output directory**: `reports/`\n\n\
         ## Protocol\n\n\
         ### Sequential read-write-proceed\n\n\
         For each role in execution order:\n\
         1. READ   `standards/<NN>-<role>-standard.md`\n\
         2. APPLY  its checklist to the diff in `review_context.xml`\n\
         3. WRITE  `reports/<NN>-<role>-review.md` with findings\n\
         4. APPEND every [BLOCKER] and [MAJOR] to `temp/findings.md` immediately:\n\
            `[ROLE][SEVERITY] path:line — short description`\n\
            Example: `[SE][BLOCKER] src/auth.rs:34 — IDOR: no tenant check`\n\
         5. PROCEED to next role — do not re-read previous standard\n\n\
         After all role files:\n\
         6. READ  `temp/findings.md` only — never re-read individual role files\n\
         7. WRITE `reports/99-verdict.md` synthesising from that log\n\
         8. FILL  `reports/00-summary.md` LAST (table of contents + final assessment)\n\n\
         ### Always\n\
         - Cite file path and line number for every finding\n\
         - Append to `temp/findings.md` before moving to the next role\n\
         - Write `reports/99-verdict.md` from `temp/findings.md` only\n\
         - Write `reports/00-summary.md` after ALL role files and verdict are done\n\n\
         ### Never\n\
         - Load all role standards simultaneously — one standard file at a time\n\
         - Skip `temp/findings.md` — it is the sole input for verdict synthesis\n\
         - Leave a finding without a severity label\n\
         - Write full `00-summary.md` before role files are complete\n\n\
         ### Severity labels\n\n\
         | Label | Meaning | Blocks merge? |\n\
         |---|---|---|\n\
         | `[BLOCKER]` | Incorrect behavior, security hole, data loss | Yes |\n\
         | `[MAJOR]` | Significant flaw, hard to fix later | Yes |\n\
         | `[SUGGESTION]` | Better approach exists, current code works | No |\n\
         | `[NIT]` | Style, naming, minor readability | No |\n\
         | `[QUESTION]` | Needs clarification | No |\n\n\
         ## Success Brief\n\n\
         **Output**: `reports/00-summary.md` + one file per activated role + `reports/99-verdict.md`\n\
         **Success means**: Every [BLOCKER] and [MAJOR] has a file path, line number, \
         explanation, and a suggested fix the author can act on immediately\n\
         **Does NOT sound like**: Vague advice without code references, or a summary-only \
         verdict that skips individual role perspectives\n\n\
         ## Specific Questions\n\n\
         <!-- Paste any focus areas or specific questions here -->\n\n\
         ## Additional Roles\n\
         <!-- Roles beyond auto-triggered defaults:\n\
              pe, se, oe, de, ux, cl, ceo, da, mle, api, finops, dx\n\
              Example: - da  (major architectural change) -->\n\n\
         ## Skip Roles\n\
         <!-- Suppress auto-triggered roles not relevant to this scope\n\
              Example: - ceo  (internal refactor, no user-visible impact) -->\n\n\
         ## Plan Before Executing\n\n\
         Before writing any file, state:\n\
         1. The 3 rules from the Protocol section above that matter most for this scope\n\
         2. Your role execution order — should match `temp/role-plan.md`:\n\
            Suggested: {order}\n\n\
         Only begin executing once you have written the plan.\n",
        ctx = ctx_files.join("\n"),
        scope = scope.description(),
        today = today,
        order = suggested_order,
    );

    write_file_safe(&materials_dir.join("review_prompt.md"), &prompt)?;
    Ok(())
}

/// Generate UPLOAD_ORDER.md — the human-readable guide for what to upload.
fn write_upload_order(materials_dir: &Path, triggered: &[&RoleTrigger]) -> Result<(), ZrkError> {
    let ts = materials_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let mut files = vec![
        "review_context.xml".to_string(),
        "standards/00-loading-guide.md".to_string(),
        "standards/01-swe-standard.md".to_string(),
        "standards/02-sa-standard.md".to_string(),
        "standards/03-qa-standard.md".to_string(),
    ];
    for t in triggered {
        files.push(format!("standards/{}", t.standard));
    }
    files.push("temp/role-plan.md".to_string());
    files.push("temp/file-map.md".to_string());

    let numbered: String = files
        .iter()
        .enumerate()
        .map(|(i, f)| format!("{}. `{}`\n", i + 1, f))
        .collect();

    let content = format!(
        "# Upload to Claude.ai\n\n\
         **Easiest**: zip the entire materials folder and attach, then paste `review_prompt.md` as your message.\n\n\
         ```bash\n\
         cd .materials && zip -r ../review-{ts}.zip {ts}/ && cd ..\n\
         ```\n\n\
         This creates `review-{ts}.zip` in the project root (next to `.materials/`).\n\n\
         **Alternative** — upload files individually in this order:\n\n\
         {numbered}\n\
         Then paste `review_prompt.md` as your message (or upload it).\n\n\
         ---\n\
         Finished review files land in `reports/` — move them to your project `reports/` when done.\n",
        ts = ts,
        numbered = numbered,
    );

    write_file_safe(&materials_dir.join("UPLOAD_ORDER.md"), &content)?;
    Ok(())
}

// ── Main entry point ──────────────────────────────────────────────────────────

fn resolve_cwd(cli: &Cli) -> PathBuf {
    cli.cwd.clone().unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    })
}

pub fn run_prep(cli: &Cli, args: &PrepArgs) -> Result<(), ZrkError> {
    let cwd = resolve_cwd(cli);
    let scope = parse_scope(args)?;
    let ts = timestamp();
    let materials_dir = cwd.join(".materials").join(&ts);

    if !cli.quiet {
        output::info(&format!("Preparing review for: {}", scope.description()));
    }

    // ── 1. Get changed files ──────────────────────────────────────────────────
    let changed_files = if cli.dry_run {
        vec![]
    } else {
        match &scope {
            PrepScope::GitRange(r) => git_changed_files_range(r, &cwd)?,
            PrepScope::CommitHashes(h) => git_changed_files_hashes(h, &cwd)?,
            PrepScope::Topic(t) => rg_topic_files(t, &cwd)?,
        }
    };

    if !cli.quiet && !changed_files.is_empty() {
        output::info(&format!("  {} changed files detected", changed_files.len()));
    }

    // ── 2. Detect triggered roles ─────────────────────────────────────────────
    let triggered = detect_triggered_roles(&changed_files);

    if !cli.quiet {
        if triggered.is_empty() {
            output::info("  Roles: swe, sa, qa (core only — no extra triggers detected)");
        } else {
            let codes: Vec<&str> = triggered.iter().map(|t| t.code).collect();
            output::info(&format!(
                "  Triggered: swe, sa, qa + {}",
                codes.join(", ")
            ));
        }
    }

    // ── 3. Dry run ────────────────────────────────────────────────────────────
    if cli.dry_run {
        output::info(&format!("Would create: .materials/{}/", ts));
        output::info("  standards/   core + triggered role standards");
        output::info("  reports/     00-summary.md, 99-verdict.md stubs");
        output::info("  temp/        role-plan.md, file-map.md, findings.md");
        output::info("  review_prompt.md, UPLOAD_ORDER.md");
        output::info("  review_context.xml (via repomix)");
        return Ok(());
    }

    // ── 4. Create directory structure ─────────────────────────────────────────
    for d in &[
        materials_dir.clone(),
        materials_dir.join("standards"),
        materials_dir.join("reports"),
        materials_dir.join("temp"),
    ] {
        std::fs::create_dir_all(d).map_err(ZrkError::Io)?;
    }

    // ── 5. Write all generated files ──────────────────────────────────────────
    write_standards(&materials_dir, &triggered)?;
    write_temp_stubs(&materials_dir, &scope, &triggered, &changed_files)?;
    write_report_stubs(&materials_dir, &scope)?;
    write_review_prompt(&materials_dir, &scope, &triggered)?;
    write_upload_order(&materials_dir, &triggered)?;

    // ── 6. Run repomix (optional) ─────────────────────────────────────────────
    let context_path = materials_dir.join("review_context.xml");

    // Merge --include files and --docs folder expansions
    let mut all_extra = args.extra_files.clone();
    all_extra.extend(expand_doc_dirs(&args.doc_dirs, &cwd));
    all_extra.sort();
    all_extra.dedup();

    if !cli.quiet && !all_extra.is_empty() {
        output::info(&format!("  Extra files: {}", all_extra.len()));
    }

    let has_context;
    if command_available("repomix") {
        if !cli.quiet {
            output::info("  Running repomix...");
        }
        has_context = run_repomix(&scope, &all_extra, &cwd, &context_path);
        if !has_context {
            output::warning("repomix exited with an error — review_context.xml may be incomplete");
        }
    } else {
        output::warning("repomix not found — skipping review_context.xml");
        output::warning("  Install: npm install -g repomix");
        output::warning("  Then generate manually: see pack-materials.md");
        has_context = false;
    }

    // ── 7. Write patch (git modes only) ──────────────────────────────────────
    let has_patch = !matches!(scope, PrepScope::Topic(_))
        && write_patch(&scope, &cwd, &materials_dir.join("review.patch"));

    // ── 8. Print summary ──────────────────────────────────────────────────────
    if !cli.quiet {
        print_done(&materials_dir, &ts, &triggered, has_context, has_patch);
    }

    Ok(())
}

fn print_done(
    materials_dir: &Path,
    ts: &str,
    triggered: &[&RoleTrigger],
    has_context: bool,
    has_patch: bool,
) {
    println!();
    output::success(&format!("Materials ready in .materials/{}/", ts));
    println!();

    let ctx_note = if has_context { "" } else { "  ← missing: run repomix manually" };
    println!("  review_context.xml{}", ctx_note);
    if has_patch {
        println!("  review.patch");
    }
    println!("  standards/");
    for name in &[
        "00-loading-guide.md",
        "01-swe-standard.md",
        "02-sa-standard.md",
        "03-qa-standard.md",
    ] {
        println!("    {}", name);
    }
    for t in triggered {
        println!("    {}  ← triggered: {}", t.standard, t.code);
    }
    println!("  temp/");
    println!(
        "    role-plan.md    ← pre-filled ({} roles)",
        3 + triggered.len()
    );
    println!("    file-map.md     ← stub");
    println!("    findings.md     ← stub");
    println!("  reports/");
    println!("    00-summary.md   ← stub, fill LAST");
    println!("    99-verdict.md   ← stub, from findings.md only");
    println!();
    output::info(&format!("See .materials/{}/UPLOAD_ORDER.md", ts));
    output::info("Paste review_prompt.md as your Claude.ai message");
    println!();

    // Remind if context is missing
    if !has_context {
        output::warning("review_context.xml not generated. Run manually:");
        let dir_display = materials_dir.display();
        println!("  git diff <range> --name-only | repomix --stdin --style xml --output {}/review_context.xml", dir_display);
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use crate::Command as ZrkCommand;

    // ── parse_scope ──────────────────────────────────────────────────────────

    #[test]
    fn parse_scope_git_range_with_dots() {
        let args = PrepArgs { scope: vec!["HEAD~3..HEAD".into()], topic: None,
            extra_files: vec![],
            doc_dirs: vec![]
        };
        assert!(matches!(parse_scope(&args).unwrap(), PrepScope::GitRange(_)));
    }

    #[test]
    fn parse_scope_branch_range() {
        let args = PrepArgs { scope: vec!["feature/auth..main".into()], topic: None,
            extra_files: vec![],
            doc_dirs: vec![]
        };
        assert!(matches!(parse_scope(&args).unwrap(), PrepScope::GitRange(_)));
    }

    #[test]
    fn parse_scope_multiple_hashes() {
        let args = PrepArgs {
            scope: vec!["abc123".into(), "def456".into(), "ghi789".into()],
            topic: None,
            extra_files: vec![],
            doc_dirs: vec![]
        };
        let s = parse_scope(&args).unwrap();
        if let PrepScope::CommitHashes(h) = s {
            assert_eq!(h.len(), 3);
        } else {
            panic!("expected CommitHashes");
        }
    }

    #[test]
    fn parse_scope_single_hash_no_dots() {
        let args = PrepArgs { scope: vec!["abc1234def".into()], topic: None,
            extra_files: vec![],
            doc_dirs: vec![]
        };
        assert!(matches!(parse_scope(&args).unwrap(), PrepScope::CommitHashes(_)));
    }

    #[test]
    fn parse_scope_topic_flag() {
        let args = PrepArgs { scope: vec![], topic: Some("phase-0".into()),
            extra_files: vec![],
            doc_dirs: vec![]
        };
        assert!(matches!(parse_scope(&args).unwrap(), PrepScope::Topic(_)));
    }

    #[test]
    fn parse_scope_topic_overrides_positional() {
        let args = PrepArgs {
            scope: vec!["HEAD~3..HEAD".into()],
            topic: Some("auth".into()),
            extra_files: vec![],
            doc_dirs: vec![]
        };
        assert!(matches!(parse_scope(&args).unwrap(), PrepScope::Topic(_)));
    }

    #[test]
    fn parse_scope_empty_is_error() {
        let args = PrepArgs { scope: vec![], topic: None,
            extra_files: vec![],
            doc_dirs: vec![]
        };
        assert!(parse_scope(&args).is_err());
    }

    // ── detect_triggered_roles ───────────────────────────────────────────────

    #[test]
    fn detects_se_from_auth_path() {
        let files = vec!["src/auth/handler.rs".into()];
        let roles = detect_triggered_roles(&files);
        assert!(roles.iter().any(|r| r.code == "se"), "SE should trigger on auth/");
    }

    #[test]
    fn detects_de_and_pe_from_migration() {
        let files = vec!["migrations/001_add_users.sql".into()];
        let roles = detect_triggered_roles(&files);
        assert!(roles.iter().any(|r| r.code == "de"), "DE from migration");
        assert!(roles.iter().any(|r| r.code == "pe"), "PE from .sql");
    }

    #[test]
    fn detects_ux_from_tsx() {
        let files = vec!["src/components/Button.tsx".into()];
        let roles = detect_triggered_roles(&files);
        assert!(roles.iter().any(|r| r.code == "ux"));
    }

    #[test]
    fn da_never_auto_triggered() {
        // DA has no patterns — must never appear in auto-detect regardless of files
        let files: Vec<String> = (0..50).map(|i| format!("src/module{}.rs", i)).collect();
        let roles = detect_triggered_roles(&files);
        assert!(!roles.iter().any(|r| r.code == "da"), "DA must not auto-trigger");
    }

    #[test]
    fn no_triggers_for_plain_rust_files() {
        let files = vec!["src/lib.rs".into(), "src/util/mod.rs".into()];
        let roles = detect_triggered_roles(&files);
        assert!(!roles.iter().any(|r| r.code == "se"));
        assert!(!roles.iter().any(|r| r.code == "de"));
    }

    #[test]
    fn pattern_matching_is_case_insensitive() {
        let files = vec!["src/Auth/Handler.rs".into()];
        let roles = detect_triggered_roles(&files);
        assert!(roles.iter().any(|r| r.code == "se"));
    }

    // ── days_to_date ─────────────────────────────────────────────────────────

    #[test]
    fn days_to_date_unix_epoch() {
        assert_eq!(days_to_date(0), (1970, 1, 1));
    }

    #[test]
    fn days_to_date_leap_day_2024() {
        // 2024-02-29 = 19782 days since epoch
        assert_eq!(days_to_date(19_782), (2024, 2, 29));
    }

    #[test]
    fn days_to_date_march_2024() {
        // 2024-03-01 = 19783 days since epoch
        assert_eq!(days_to_date(19_783), (2024, 3, 1));
    }

    // ── extract_n_commits ────────────────────────────────────────────────────

    #[test]
    fn extract_n_commits_head_tilde_3() {
        assert_eq!(extract_n_commits("HEAD~3..HEAD"), Some(3));
    }

    #[test]
    fn extract_n_commits_branch_range_is_none() {
        assert_eq!(extract_n_commits("feature/auth..main"), None);
    }

    #[test]
    fn extract_n_commits_head_tilde_to_tilde_returns_prefix() {
        // HEAD~5..HEAD~2 — function returns the prefix N (5), which is used as
        // an approximate hint for `repomix --include-logs-count`. Approximate is fine.
        assert_eq!(extract_n_commits("HEAD~5..HEAD~2"), Some(5));
    }

    // ── integration: full scaffold ───────────────────────────────────────────

    #[test]
    fn run_prep_creates_full_structure() {
        let dir = tempfile::tempdir().unwrap();
        let cwd = dir.path();

        // Set up a minimal git repo with an auth file so SE triggers
        Command::new("git").args(["init"]).current_dir(cwd).output().ok();
        Command::new("git")
            .args(["commit", "--allow-empty", "-m", "init"])
            .envs([
                ("GIT_AUTHOR_NAME", "t"),
                ("GIT_AUTHOR_EMAIL", "t@t.com"),
                ("GIT_COMMITTER_NAME", "t"),
                ("GIT_COMMITTER_EMAIL", "t@t.com"),
            ])
            .current_dir(cwd)
            .output()
            .ok();

        std::fs::create_dir_all(cwd.join("src/auth")).unwrap();
        std::fs::write(cwd.join("src/auth/handler.rs"), "// auth handler").unwrap();
        std::fs::create_dir_all(cwd.join("migrations")).unwrap();
        std::fs::write(cwd.join("migrations/001.sql"), "CREATE TABLE t (id INT);").unwrap();

        Command::new("git").args(["add", "."]).current_dir(cwd).output().ok();
        Command::new("git")
            .args(["commit", "-m", "add auth + migration"])
            .envs([
                ("GIT_AUTHOR_NAME", "t"),
                ("GIT_AUTHOR_EMAIL", "t@t.com"),
                ("GIT_COMMITTER_NAME", "t"),
                ("GIT_COMMITTER_EMAIL", "t@t.com"),
            ])
            .current_dir(cwd)
            .output()
            .ok();

        let cli = crate::Cli {
            command: ZrkCommand::Prep(PrepArgs {
                scope: vec!["HEAD~1..HEAD".into()],
                topic: None,
                extra_files: vec![],
                doc_dirs: vec![],
            }),
            target: "kiro".to_string(),
            all_targets: false,
            force: false,
            cwd: Some(cwd.to_path_buf()),
            no_color: true,
            quiet: true,
            dry_run: false,
        };
        let args = PrepArgs {
            scope: vec!["HEAD~1..HEAD".into()],
            topic: None,
            extra_files: vec![],
            doc_dirs: vec![],
        };

        run_prep(&cli, &args).unwrap();

        // Find the materials timestamp dir
        let materials = cwd.join(".materials");
        assert!(materials.exists());
        let ts_dir = std::fs::read_dir(&materials)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();

        // Directory structure
        assert!(ts_dir.join("standards").is_dir());
        assert!(ts_dir.join("reports").is_dir());
        assert!(ts_dir.join("temp").is_dir());

        // Core standards always present
        for name in &[
            "review-prompting.md",
            "review-roles.md",
            "00-loading-guide.md",
            "01-swe-standard.md",
            "02-sa-standard.md",
            "03-qa-standard.md",
        ] {
            assert!(
                ts_dir.join("standards").join(name).exists(),
                "missing: standards/{}", name
            );
        }

        // SE triggered by src/auth/handler.rs
        assert!(
            ts_dir.join("standards/05-se-standard.md").exists(),
            "SE should be triggered by auth path"
        );
        // DE + PE triggered by migrations/001.sql
        assert!(ts_dir.join("standards/07-de-standard.md").exists(), "DE from .sql");
        assert!(ts_dir.join("standards/04-pe-standard.md").exists(), "PE from .sql");

        // OE from route/handler patterns in auth handler
        assert!(ts_dir.join("standards/06-oe-standard.md").exists(), "OE from handler");

        // Stubs
        assert!(ts_dir.join("temp/role-plan.md").exists());
        assert!(ts_dir.join("temp/file-map.md").exists());
        assert!(ts_dir.join("temp/findings.md").exists());
        assert!(ts_dir.join("reports/00-summary.md").exists());
        assert!(ts_dir.join("reports/99-verdict.md").exists());
        assert!(ts_dir.join("review_prompt.md").exists());
        assert!(ts_dir.join("UPLOAD_ORDER.md").exists());

        // role-plan.md mentions SE and is pre-filled
        let role_plan = std::fs::read_to_string(ts_dir.join("temp/role-plan.md")).unwrap();
        assert!(role_plan.contains("se"), "role-plan should list SE");
        assert!(role_plan.contains("01-swe"), "execution order should start with 01-swe");
        assert!(role_plan.contains("99-verdict"), "execution order should end with 99-verdict");

        // findings.md has the protocol comment
        let findings = std::fs::read_to_string(ts_dir.join("temp/findings.md")).unwrap();
        assert!(findings.contains("[BLOCKER]"));
        assert!(findings.contains("findings.md"));

        // review_prompt.md is self-contained with inline protocol
        let prompt = std::fs::read_to_string(ts_dir.join("review_prompt.md")).unwrap();
        assert!(prompt.contains("[BLOCKER]"));
        assert!(prompt.contains("findings.md"));
        assert!(prompt.contains("sequential") || prompt.contains("Sequential"));
        assert!(prompt.contains("00-summary.md"));
        assert!(prompt.contains("Plan Before Executing"));
        assert!(prompt.contains("05-se-standard.md"), "prompt should list SE standard");

        // UPLOAD_ORDER.md lists the triggered standards
        let upload = std::fs::read_to_string(ts_dir.join("UPLOAD_ORDER.md")).unwrap();
        assert!(upload.contains("05-se-standard.md"));
    }

    #[test]
    fn cli_parse_prep_with_include() {
        let cli = Cli::try_parse_from([
            "zrk", "prep", "abc123", "--include", "AGENTS.md", "README.md",
        ])
        .unwrap();
        if let ZrkCommand::Prep(args) = cli.command {
            assert_eq!(args.extra_files, vec!["AGENTS.md", "README.md"]);
        } else {
            panic!("expected Prep");
        }
    }

    #[test]
    fn cli_parse_prep_with_docs() {
        let cli = Cli::try_parse_from([
            "zrk", "prep", "HEAD~1..HEAD", "--docs", "docs/architecture/v1",
        ])
        .unwrap();
        if let ZrkCommand::Prep(args) = cli.command {
            assert_eq!(args.doc_dirs, vec!["docs/architecture/v1"]);
        } else {
            panic!("expected Prep");
        }
    }

    #[test]
    fn expand_doc_dirs_returns_md_files() {
        let dir = tempfile::tempdir().unwrap();
        let arch = dir.path().join("docs/arch");
        std::fs::create_dir_all(&arch).unwrap();
        std::fs::write(arch.join("overview.md"), "# Overview").unwrap();
        std::fs::write(arch.join("infra.md"), "# Infra").unwrap();
        std::fs::write(arch.join("schema.sql"), "CREATE TABLE t (id INT);").unwrap();

        let files = expand_doc_dirs(&["docs/arch".to_string()], dir.path());
        assert_eq!(files.len(), 2, "only .md files");
        assert!(files.iter().all(|f| f.ends_with(".md")));
    }

    #[test]
    fn expand_doc_dirs_skips_reviews_subdir() {
        let dir = tempfile::tempdir().unwrap();
        let arch = dir.path().join("docs/arch");
        let reviews = arch.join("reviews");
        std::fs::create_dir_all(&reviews).unwrap();
        std::fs::write(arch.join("overview.md"), "# Overview").unwrap();
        std::fs::write(reviews.join("old-review.md"), "old").unwrap();

        let files = expand_doc_dirs(&["docs/arch".to_string()], dir.path());
        assert_eq!(files.len(), 1);
        assert!(!files[0].contains("reviews"));
    }

    #[test]
    fn expand_doc_dirs_missing_dir_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let files = expand_doc_dirs(&["nonexistent/path".to_string()], dir.path());
        assert!(files.is_empty());
    }

    #[test]
    fn run_prep_dry_run_creates_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let cwd = dir.path();

        let cli = crate::Cli {
            command: ZrkCommand::Prep(PrepArgs {
                scope: vec!["HEAD~1..HEAD".into()],
                topic: None,
                extra_files: vec![],
                doc_dirs: vec![],
            }),
            target: "kiro".to_string(),
            all_targets: false,
            force: false,
            cwd: Some(cwd.to_path_buf()),
            no_color: true,
            quiet: true,
            dry_run: true,
        };
        let args = PrepArgs {
            scope: vec!["HEAD~1..HEAD".into()],
            topic: None,
            extra_files: vec![],
            doc_dirs: vec![],
        };

        run_prep(&cli, &args).unwrap();

        // dry_run should create no files
        assert!(!cwd.join(".materials").exists());
    }
}