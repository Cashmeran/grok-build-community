# Grok Build Community Edition - Maintenance Guide

## Directory Structure

```
├── patches/          ← 17 patches, one per CE change
├── ce-tools/         ← CE tool source (never touched by upstream)
├── docs/             ← Documentation
├── build.bat         ← One-click: apply patches → build
└── README.md         ← Project overview
```

## Daily Workflow

### After upstream update
```bash
git pull upstream main
./build.bat
# Failed patch? Fix code → git diff > patches/failed.patch
```

### Adding a new CE feature
```bash
# 1. Code changes as normal
# 2. Export as patch
git diff > patches/18-feature-name.patch
# 3. Add to build.bat list
# 4. Update this guide
```

### Just modifying CE tools
Edit files in `ce-tools/src/`. These are reference copies - 
actual compilation uses the patches system.

## Patch List

| # | File | What |
|---|------|------|
| 01 | remove-telemetry | Delete Mixpanel, Sentry, Google Cloud uploads |
| 02 | web-search | Multi-backend: Messages, Chat, DuckDuckGo |
| 03 | community-docs | Community edition docs (EN+CN) |
| 04 | add-glob-tool | glob file pattern matching |
| 05 | add-calculator-tool | Calculator + statistics |
| 06 | add-json-csv-tools | json_query + csv_ops |
| 07 | add-text-codec-tools | text + codec |
| 08 | community-branding | CE branding |
| 09 | optimize-system-prompt | Better system prompt |
| 10 | community-tool-docs | CE tool documentation |
| 11 | ci-cd-build | CI/CD pipeline + block auto-update |
| 12 | install-scripts | One-command install |
| 13 | clean-dead-code | Remove dead code + warnings |
| 14 | inject-tools | Register all 6 CE tools in toolsets |
| 15 | binary-name-grokce | Binary named grokce |
| 16 | websearch-zero-config | Zero-config web search |
| 17 | ci-fixes | CI fmt + clippy fixes |

## Architecture

Same approach as VSCodium, ungoogled-chromium, LineageOS:
- Upstream code kept clean
- CE changes stored as independent patches
- Each patch fails independently, easy to diagnose
