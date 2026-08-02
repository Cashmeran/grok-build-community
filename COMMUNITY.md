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
| Mixpanel user analytics | 鉁?Removed |
| Sentry crash reporting | 鉁?Removed |
| GCS session upload | 鉁?Removed |
| OpenTelemetry data export | 鉁?Stubbed (local-only) |
| Hook event logging | 鉁?Removed |

All telemetry data is now **written to local files** (`~/.grok/logs/events.log`).

#### Web Search

| Official | Community Edition |
|----------|-------------------|
| Grok-only Responses API search | 鉁?4 backends with auto-routing |
| No search for other models | 鉁?DuckDuckGo free fallback |

Search backends:

| Backend | Works With | Notes |
|---------|-----------|-------|
| Responses API | Grok, GPT-4 | Original, preserved |
| Messages API (new) | DeepSeek, Claude | Anthropic web_search tool |
| Chat Completions (new) | OpenAI search models | web_search_options |
| DuckDuckGo (new) | All models | Free, zero-config fallback |

Backend is **auto-selected** based on the model's `api_backend` 鈥?no manual config needed.

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

The system prompt has been improved with additional guidance for communication style, accuracy, and execution discipline 鈥?reducing AI clich茅s and hallucinations.

#### Multi-Language Support (new)

| Language | Code |
|----------|------|
| English | `en` |
| 涓枃 (Simplified) | `zh-CN` |
| 鏃ユ湰瑾?| `ja` |
| 頃滉淡鞏?| `ko` |
| 袪褍褋褋泻懈泄 | `ru` |
| Fran莽ais | `fr` |

- **Auto-detect**: Matches system language on first startup
- **Real-time switch**: `/lang zh-CN` takes effect immediately, no restart needed
- **Dropdown picker**: `/lang` then press Tab for language selection
- **Coverage**: 278 translations (slash commands, shortcuts, settings, welcome screen, toasts, permission modals)

#### API Error Message De-branding

- Third-party API quota errors no longer show "SuperGrok" promotional copy
- All hardcoded `grok.com/supergrok` URLs removed
- Error messages use generic wording ("API quota exhausted, check your API account")

#### WebFetch Tool

- WebFetch (URL content fetching) tool enabled by default
- Set `GROK_DISABLE_WEB_FETCH=1` env var to disable

#### Session Forward Compatibility

- Added `ConversationItem::Unknown` variant 鈥?old sessions won't fail after version upgrades
- Unknown message types are silently skipped instead of crashing

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
