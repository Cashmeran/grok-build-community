# Grok Build Community Edition

An independent, community-maintained fork of [SpaceXAI Grok Build](https://github.com/xai-org/grok-build).

**Positioning**: Grok Build Community Edition is to official Grok Build what VSCodium is to VS Code.

## Core Principles

- **Privacy First**: All telemetry and user behavior tracking removed. Data stays local.
- **Model Freedom**: No vendor lock-in. Works with any OpenAI-compatible or Anthropic-compatible API.
- **Developer Friendly**: Missing tools and capabilities added. Community PRs welcome.

## Key Differences from Official

### Completed Changes

#### Privacy & Security

| Official | Community Edition |
|----------|-------------------|
| Mixpanel user analytics | ✅ Removed |
| Sentry crash reporting | ✅ Removed |
| GCS session upload | ✅ Removed |
| OpenTelemetry data export | ✅ Stubbed (local-only) |
| Hook event logging | ✅ Removed |

All telemetry data is now **written to local files** (`~/.grok/logs/events.log`).

#### Web Search

| Official | Community Edition |
|----------|-------------------|
| Grok-only Responses API search | ✅ 4 backends with auto-routing |
| No search for other models | ✅ DuckDuckGo free fallback |

Search backends:

| Backend | Works With | Notes |
|---------|-----------|-------|
| Responses API | Grok, GPT-4 | Original, preserved |
| Messages API (new) | DeepSeek, Claude | Anthropic web_search tool |
| Chat Completions (new) | OpenAI search models | web_search_options |
| DuckDuckGo (new) | All models | Free, zero-config fallback |

Backend is **auto-selected** based on the model's `api_backend` — no manual config needed.

#### New Tools

| Tool | Description |
|------|-------------|
| `glob` | File pattern matching (`*`, `**`, `?`), sorted by modification time |
| `calculator` | Math expressions, statistics (mean/median/stdev), multi-step variables |
| `json_query` | JSON path navigation, filtering, aggregation (sum/avg/min/max) |
| `csv_ops` | CSV/TSV schema inference, filtering, sorting, grouped aggregation |
| `text` | Regex extract/replace/count with linear-time matching |
| `codec` | Base64/hex/URL encode/decode, sha256/sha512/md5/crc32/blake3 |

#### Prompt Optimizations

The system prompt has been improved with additional guidance for communication style, accuracy, and execution discipline — reducing AI clichés and hallucinations.

### Planned

## Build

```bash
# Requires Rust toolchain
cargo build -p xai-grok-pager-bin --release
```

## Contributing

Pull requests welcome. We accept community improvements, fixes, and suggestions.

## License

Apache 2.0, same as upstream.

## Disclaimer

Not affiliated with SpaceXAI / xAI. No warranty provided. Model inference still requires your own API keys.
