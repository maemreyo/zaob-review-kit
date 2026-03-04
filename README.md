# zrk

**Install review workflows into your AI coding agent. Get code review materials ready for Claude.ai with one sentence.**

---

Type this in your agent:

```
tạo materials để review 3 commit gần nhất
```

Your agent produces `review_context.xml` + `review_prompt.md` — ready to upload to Claude.ai.

---

## The problem

You write code in your IDE with Kiro, Claude Code, Cursor, or Windsurf. You review it by uploading to Claude.ai. The gap between them is manual, repetitive, and lossy.

| Approach                             | Problem                                            |
| ------------------------------------ | -------------------------------------------------- |
| Zip the whole codebase               | Hits context limit immediately                     |
| Send a git patch                     | Missing context — referenced files aren't included |
| Copy-paste files manually            | Slow, inconsistent, easy to miss dependencies      |
| Write the review prompt from scratch | Repetitive, no structure, no role perspective      |

`zrk` installs the workflow files that bridge this gap.

## Install

```bash
cargo install zrk
```

Or via curl (Linux/macOS):

```bash
curl -fsSL https://raw.githubusercontent.com/zaob-dev/zrk/main/install.sh | sh
```

## Quickstart

```bash
# First time: interactive setup
zrk init

# Or skip the wizard
zrk install-all --target kiro
```

Then in your agent: _"tạo materials để review 3 commit gần nhất"_

Upload the generated files from `.materials/` to Claude.ai.

## Supported agents

| Agent       | Global config                    | Workspace config               | Format                    |
| ----------- | -------------------------------- | ------------------------------ | ------------------------- |
| Kiro        | `~/.kiro/steering/`              | `.kiro/steering/`              | `.md` + YAML frontmatter  |
| Claude Code | `~/.claude/commands/review-kit/` | `.claude/commands/review-kit/` | `.md` + YAML frontmatter  |
| Cursor      | (manual)                         | `.cursor/rules/`               | `.mdc` + YAML frontmatter |
| Windsurf    | (manual)                         | `.windsurf/rules/`             | `.md` + comment header    |

## Commands

```
zrk install           Install workspace files into current project
zrk install-global    Install global files into agent's global config
zrk install-all       Install both (recommended first time)
zrk update            Force reinstall with latest content
zrk status            Show installation status and drift
zrk list              List available agents and content files
zrk init              Interactive first-time setup wizard
```

## Documentation

- [What is zrk?](docs/getting-started/what-is-zrk.md)
- [Installation](docs/getting-started/installation.md)
- [Quickstart](docs/getting-started/quickstart.md)
- [First review walkthrough](docs/getting-started/first-review.md)
- [CLI command reference](docs/commands/overview.md)
- [Agent setup guides](docs/agents/overview.md)
- [How the review workflow works](docs/workflow/overview.md)
- [CLI flags reference](docs/reference/cli-flags.md)
- [Architecture](docs/contributing/architecture.md)
- [Adding a new agent](docs/contributing/adding-an-agent.md)

## License

MIT
