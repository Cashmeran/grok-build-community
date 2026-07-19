# Changelog

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
