<!-- Language switcher -->
<div align="right">
  <a href="README.md">English</a> | <a href="README_CN.md">中文</a>
</div>

# Grok Build Community Edition

[![CI](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml/badge.svg)](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml)

An independent, community-maintained fork of [SpaceXAI Grok Build](https://github.com/xai-org/grok-build).

**Positioning**: Grok Build CE is to official Grok Build what [VSCodium](https://github.com/VSCodium/vscodium) is to VS Code.

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

---

## Install

Prebuilt binaries for Windows, macOS, and Linux are published on the [Releases](https://github.com/Cashmeran/grok-build-community/releases) page.

**Build from source:**
```bash
cargo build -p xai-grok-pager-bin --release
```

Multi-model config:

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

## Contributing

Pull requests welcome.

## License

Apache 2.0, same as upstream.

## Disclaimer

Not affiliated with SpaceXAI / xAI. Use your own API keys.
