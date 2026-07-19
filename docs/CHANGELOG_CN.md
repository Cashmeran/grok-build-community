# 更新日志

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
