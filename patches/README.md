# CE Patches

Community Edition patches for Grok Build.

| Patch | Size | Description |
|-------|------|-------------|
| `00-community-foundation.patch` | 13.7 MB | Core: de-branding, telemetry (entry-point cuts), models, auth, shell, config, pager |
| `01-community-ci.patch` | 11 KB | CI/CD workflows, build.bat, install.ps1, install.sh |
| `02-community-prompt.patch` | 8 KB | Prompt templates, system prompts, optimization rules |
| `03-community-tools.patch` | 551 KB | Community tools (ce-tools), web search (cloud-only HostedTool path), tool registration |
| `04-community-i18n.patch` | 0 KB | (Removed — i18n deleted to reduce conflicts) |

## Quick Start

```powershell
# Sync with upstream
.\sync.ps1           # Fetch + regenerate patches
.\sync.ps1 -Build    # Also compile

# Or build from clean upstream clone
git clone https://github.com/xai-org/grok-build.git
cd grok-build
git apply patches/00-community-foundation.patch
git apply patches/01-community-ci.patch
git apply patches/02-community-prompt.patch
git apply patches/03-community-tools.patch
cargo build -p xai-grok-pager-bin --release
```

## Key Changes (v0.2.109-ce.6)

- **Telemetry**: Entry-point early returns (20 lines, 2 files) instead of module gutting
- **Web Search**: Moved to cloud-only HostedTool path (Messages/Chat/Responses all supported)
- **i18n**: Removed entirely to reduce merge conflicts
- **Formatting noise**: Reverted upstream drift files
