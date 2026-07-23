# Changelog

## v0.2.109-ce.1 (2026-07-23)

### Multi-Language Support (i18n)
- New `xai-grok-i18n` crate with lightweight translation engine
- 6 languages: English, 中文, 日本語, 한국어, Русский, Français
- Auto-detects system language on first startup (`LANG` / `LC_ALL` env vars)
- `/lang` command: real-time switching, Tab dropdown picker, no restart needed
- 278 translations: slash commands, shortcuts, settings, welcome screen, toasts, permissions

### Bug Fixes
- **WebFetch**: URL fetch tool now enabled by default (was invisible to Agent)
- **Session compatibility**: `ConversationItem::Unknown` variant prevents old session crashes
- **Build hygiene**: All compiler warnings fixed, zero-warning release build

### De-branding
- Third-party API errors no longer show "SuperGrok" promotional copy
- All `grok.com/supergrok` hardcoded URLs removed
- Image/video generation tier-restricted messages use generic wording

### Upstream sync
- Based on upstream v0.2.109

---

## 0.2.105-ce.1 (2026-07-19)

### Upstream sync
- Merged upstream v0.2.105: Grok 4.5 model support, event loop rewrite, MCP OAuth improvements, `/btw` command, line editor, dashboard refactor (364 files, +30K/-16K lines)

### CE infrastructure
- Established patch-based maintenance system (17 independent patches + `build.bat`)
- CE tool source moved to `ce-tools/` directory (safe from upstream overwrites)
- Versioning scheme: upstream version + `-ce.N` patch number
- Documentation reorganized: README → CHANGELOG + MAINTENANCE + patches/README

---

## Previous (v0.1.0 - v0.3.0)

Early CE releases with independent versioning. Key milestones:
- Telemetry removal (Mixpanel, Sentry, Google Cloud)
- Multi-backend web search (Messages API, Chat Completions, DuckDuckGo, Bing)
- 6 new tools: glob, calculator, json_query, csv_ops, text, codec
- Community branding and optimized system prompt
- One-command install scripts, binary named `grokce`
- CI/CD pipeline with release automation
