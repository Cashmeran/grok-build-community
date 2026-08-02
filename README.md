<!-- Language switcher -->
<div align="right">
  <a href="README.md">English</a> | <a href="README_CN.md">中文</a>
</div>

# Grok Build Community Edition

[![CI](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml/badge.svg)](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml)

An independent, community-maintained fork of [SpaceXAI Grok Build](https://github.com/xai-org/grok-build).

**Grok Build CE is to official Grok Build what [VSCodium](https://github.com/VSCodium/vscodium) is to VS Code.**

---

## What's Different

### Privacy
- Removed Mixpanel user analytics
- Removed Sentry crash reporting
- Removed Google Cloud session upload
- Removed OpenTelemetry data export
- All telemetry data written to **local files** (`~/.grok/logs/`)
- Auto-update from x.ai **blocked**

### Search — 4 Backends
Automatically selected based on your model:

| Backend | Works With |
|---------|-----------|
| Responses API | Grok, GPT-4 |
| Messages API | DeepSeek, Claude |
| Chat Completions | OpenAI search models |
| DuckDuckGo | All models (free fallback) |

### New Tools
| Tool | Description |
|------|-------------|
| `glob` | File pattern matching (`*`, `**`, `?`) |
| `calculator` | Math expressions + statistics |
| `json_query` | JSON query / filter / aggregate |
| `csv_ops` | CSV query / sort / group |
| `text` | Regex extract / replace / count |
| `codec` | Encode / decode + hash |

### Prompt
Optimized system prompt with communication style, accuracy, and execution discipline guidance. Less AI-cliché, fewer hallucinations.

### Multi-Language (v0.2.109+)
6 languages, auto-detected from system, switch with `/lang`:

| `/lang en` | English | `/lang zh-CN` | 中文 |
| `/lang ja` | 日本語 | `/lang ko` | 한국어 |
| `/lang ru` | Русский | `/lang fr` | Français |

278 translations covering the entire UI surface.

---

## Install

### One-command install

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.ps1 | iex
```

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.sh | bash
```

After install, run:
```
grokce
```

Prebuilt binaries are also published on the [Releases](https://github.com/Cashmeran/grok-build-community/releases) page.

### Build from source
```bash
cargo build -p xai-grok-pager-bin --release
```

### Multi-model config
```toml
# ~/.grok/config.toml
[models]
default = "deepseek"

[model.deepseek]
model = "deepseek-v4-pro"
base_url = "https://api.deepseek.com/v1"
env_key = "DEEPSEEK_API_KEY"
context_window = 1000000
```

---

## Documentation

| Document | Content |
|----------|---------|
| [CHANGELOG](docs/CHANGELOG.md) | Version history and release notes |
| [更新日志](docs/CHANGELOG_CN.md) | 中文更新日志 |
| [Maintenance Guide](docs/MAINTENANCE.md) | Development & upstream sync |
| [维护指南](docs/MAINTENANCE_CN.md) | 中文维护指南 |
| [Patches](patches/README.md) | Patch details and workflow |

---

## Contributing

Pull requests welcome. See [docs/MAINTENANCE.md](docs/MAINTENANCE.md) for development workflow.

## License

Apache 2.0, same as upstream.

## Disclaimer

Not affiliated with SpaceXAI / xAI. Use your own API keys.
