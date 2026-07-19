# CE Patches

17 independent patches for Grok Build Community Edition.
Apply in numbered order. See patches/README.md for details.

## Functional Groups

| Group | Patches | What |
|-------|---------|------|
| Privacy | 01 | Remove Mixpanel/Sentry/GCS telemetry → local logs |
| Web Search | 02, 16 | Multi-backend: Messages, Chat, DuckDuckGo, Bing |
| Tools | 04-07, 14 | glob, calculator, json_query, csv_ops, text, codec + toolset injection |
| Branding | 08, 09 | Community branding + optimized system prompt |
| Build | 11, 13, 17 | CI/CD, dead code cleanup, clippy fixes |
| Install | 12, 15 | One-command install scripts + grokce binary name |
| Docs | 03, 10 | Community documentation + tool docs |

## Upstream Update Workflow

```bash
git pull upstream main
./build.bat
# If a patch fails, fix the code and update the patch:
git diff > patches/failed-patch.patch
```

## Adding New CE Features

```bash
# Code normally, then export as patch
git diff > patches/18-your-feature.patch
# Add to build.bat patch list
```
