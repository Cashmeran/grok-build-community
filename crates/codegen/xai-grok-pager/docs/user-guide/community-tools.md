# Community Edition Tools

Tools added by Grok Build Community Edition, not available in the official release.

---

## Web Search (Multi-Backend)

The community edition supports 4 search backends, auto-selected based on your model's `api_backend`:

| Backend | Works With | How |
|---------|-----------|-----|
| **Responses API** | Grok, GPT-4 | Uses the model's native `/responses` endpoint |
| **Messages API** | DeepSeek, Claude | Uses Anthropic `web_search_20250305` tool via `/messages` |
| **Chat Completions** | OpenAI search models | Uses `web_search_options` via `/chat/completions` |
| **DuckDuckGo** | All models (fallback) | Free Instant Answer API, no key required |

When native search fails, DuckDuckGo is used automatically. No extra configuration needed.

To use a specific search model, configure it under `[models]`:

```toml
[models]
web_search = "deepseek"

[model.deepseek]
model = "deepseek-v4-pro"
base_url = "https://api.deepseek.com/anthropic"
api_backend = "messages"
env_key = "DEEPSEEK_API_KEY"
context_window = 1000000
```

---

## Structured Data

### json_query

Query JSON data with path navigation, filtering, and aggregation.

```
/json_query {"operation": "get", "path": "users[0].name", "json": "..."}
/json_query {"operation": "filter", "path": "items", "condition": "price > 100", "json": "..."}
/json_query {"operation": "sum", "path": "orders", "aggregate_on": "total", "json": "..."}
```

Operations: `get`, `keys`, `count`, `filter`, `pick`, `sum`, `avg`, `min`, `max`

Path syntax: dot-notation (`user.name`), array index (`[0]`), wildcard (`[*]`)

### csv_ops

Query CSV/TSV data with filtering, sorting, and grouped aggregation.

```
/csv_ops {"operation": "schema", "csv_text": "..."}
/csv_ops {"operation": "select", "condition": "age > 25", "csv_text": "..."}
/csv_ops {"operation": "count", "group_by": "city", "csv_text": "..."}
```

Operations: `schema`, `select`, `columns`, `sort`, `count`, `sum`, `avg`

Supports custom delimiters (`delimiter` field) and headerless CSV (`has_header: false`).

---

## File & Code

### glob

Find files by glob pattern — fast filename matching, not content search.

```
/glob {"pattern": "**/*.rs"}
/glob {"pattern": "*.toml", "path": "src/"}
/glob {"pattern": "*.rs", "head_limit": 50}
```

Supports `*`, `**` (recursive), `?` patterns. Results sorted by modification time (newest first).

---

## Text Processing

### text

Regex operations on text with linear-time matching (no ReDoS risk).

```
/text {"mode": "extract", "pattern": "\\d{4}-\\d{2}-\\d{2}", "input": "..."}
/text {"mode": "replace", "pattern": "world", "replacement": "there", "input": "hello world"}
/text {"mode": "count", "unit": "words", "input": "hello world test"}
```

Modes: `extract` (with capture groups), `replace` (`$1/$2` references), `count` (`chars`/`words`/`lines`/`matches`).

Flags: `i` (case-insensitive), `m` (multiline), `s` (dot-all).

---

## Encoding & Hashing

### codec

Encode, decode, or hash strings.

```
/codec {"operation": "base64_encode", "input": "hello"}
/codec {"operation": "sha256", "input": "hello"}
/codec {"operation": "url_decode", "input": "%E4%B8%AD%E6%96%87"}
```

Operations: `base64_encode`/`decode`, `base64url_encode`/`decode`, `hex_encode`/`decode`, `url_encode`/`decode`, `sha256`, `sha512`, `md5`, `crc32`, `blake3`.

---

## Math

### calculator

Evaluate mathematical expressions with statistics support.

```
/calculator {"expression": "2 + 3 * 4"}
/calculator {"expression": "mean([1, 2, 3, 4, 5])"}
/calculator {"expression": "x = 5; y = x * 2; y + 1"}
```

Supports: arithmetic, trig (`sin`, `cos`), statistics (`mean`, `median`, `stdev`, `variance`, `mode`, `min`, `max`, `sum`, `percentile`), multi-step expressions (`;` separator), equality checks (`2+2=4` → true).

---

## Privacy

The community edition removes all telemetry and user tracking. See `COMMUNITY.md` for the full list of privacy changes.
