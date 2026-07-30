# 更新日志

## v0.2.109-ce.1 (2026-07-23)

### 多语言支持 (i18n)
- 全新 `xai-grok-i18n` crate，轻量翻译引擎
- 6 语言支持：English、中文、日本語、한국어、Русский、Français
- 首次启动自动检测系统语言（`LANG` / `LC_ALL` 环境变量）
- `/lang` 命令：实时切换，Tab 下拉选择，无需重启
- 278 条翻译覆盖：斜杠命令、快捷键、设置面板、欢迎屏幕、提示消息、权限弹窗

### Bug 修复
- **WebFetch**：网页抓取工具默认启用（之前 Agent 看不到该工具）
- **会话兼容**：`ConversationItem::Unknown` 变体防止旧会话加载失败
- **编译清理**：修复全部编译器警告，零警告构建

### 品牌清理
- 第三方 API 错误提示移除 "SuperGrok" 推广文案
- 所有 `grok.com/supergrok` 硬编码 URL 已清除
- Image/Video generation 限制提示改为通用措辞

### 上游同步
- 基于上游 v0.2.109

---

## 0.2.105-ce.1 (2026-07-19)

### 上游同步
- 合并上游 v0.2.105：Grok 4.5 模型支持、事件循环重写、MCP OAuth 改进、`/btw` 命令、行编辑器、dashboard 重构（364 个文件，+30K/-16K 行）

### CE 基础设施
- 建立 patch 维护体系（17 个独立补丁 + `build.bat` 一键构建）
- CE 工具源码移至 `ce-tools/` 目录（上游更新不会被覆盖）
- 版本方案：上游版本 + `-ce.N` 补丁号
- 文档重组：README → CHANGELOG + MAINTENANCE + patches

---

## 早期版本 (v0.1.0 - v0.3.0)

CE 早期独立版本号。主要节点：
- 遥测删除（Mixpanel、Sentry、Google Cloud）
- 多后端 Web 搜索（Messages API、Chat Completions、DuckDuckGo、Bing）
- 6 个新工具：glob、calculator、json_query、csv_ops、text、codec
- 社区品牌去化 + 系统提示词优化
- 一键安装脚本，二进制命令 `grokce`
- CI/CD 流水线 + release 自动化
