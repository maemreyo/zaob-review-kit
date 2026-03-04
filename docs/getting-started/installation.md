# Installation

## Via cargo (recommended)

```bash
cargo install zrk
```

Requires Rust toolchain. Install Rust via [rustup.rs](https://rustup.rs) if needed.

## Via curl (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/zaob-dev/zrk/main/install.sh | sh
```

Installs the binary to `~/.local/bin/zrk`. The script adds this to your PATH if needed.

## Via Homebrew

```bash
brew install zrk
```

_(Coming soon)_

## Manual binary download

Download the binary for your platform from [GitHub Releases](https://github.com/zaob-dev/zrk/releases):

| Platform            | Binary                   |
| ------------------- | ------------------------ |
| macOS Apple Silicon | `zrk-macos-aarch64`      |
| macOS Intel         | `zrk-macos-x86_64`       |
| Linux x86_64        | `zrk-linux-x86_64`       |
| Linux ARM64         | `zrk-linux-aarch64`      |
| Windows x86_64      | `zrk-windows-x86_64.exe` |

```bash
# Example: macOS Apple Silicon
curl -L https://github.com/zaob-dev/zrk/releases/latest/download/zrk-macos-aarch64 -o zrk
chmod +x zrk
mv zrk ~/.local/bin/
```

---

## Verify the install

```bash
zrk --version
# zrk 0.1.0
```

```bash
zrk --help
```

---

## Troubleshooting

**`zrk: command not found`**

The binary is not on your PATH. Add `~/.local/bin` to PATH:

```bash
# Add to ~/.zshrc or ~/.bashrc
export PATH="$HOME/.local/bin:$PATH"

# Apply immediately
source ~/.zshrc
```

**`Permission denied` on Linux/macOS**

```bash
chmod +x ~/.local/bin/zrk
```

**`cargo install` fails**

Update Rust:

```bash
rustup update stable
```

---

## Next

- [Quickstart — up and running in 2 minutes](quickstart.md)
- [Choose your agent](../agents/overview.md)
