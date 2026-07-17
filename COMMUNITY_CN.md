# Grok Build 社区版

Grok Build 社区版是基于 [SpaceXAI Grok Build](https://github.com/xai-org/grok-build) 的独立社区维护分支。

**定位**：Grok Build 社区版 之于 官方 Grok Build，如同 VSCodium 之于 VS Code。

## 核心理念

- **隐私第一**：所有遥测和用户行为追踪已移除，数据仅存本地。
- **模型自由**：不绑定任何特定模型厂商，支持所有 OpenAI 兼容 / Anthropic 兼容 API。
- **开发者友好**：添加官方版缺失的工具和能力，接受社区 PR。

## 与官方版的主要区别

### 已完成的改动

#### 隐私与安全

| 官方版 | 社区版 |
|--------|--------|
| Mixpanel 用户行为分析 | ✅ 已删除 |
| Sentry 崩溃上报 | ✅ 已删除 |
| Google Cloud Storage 会话上传 | ✅ 已删除 |
| OpenTelemetry 数据导出 | ✅ 已改为空壳（本地） |
| Hook 事件日志上报 | ✅ 已删除 |

所有原远程遥测数据**改为写入本地文件**（`~/.grok/logs/events.log`）。

#### Web 搜索

| 官方版 | 社区版 |
|--------|--------|
| 仅支持 Grok 模型的 Responses API 搜索 | ✅ 4 种搜索后端自动适配 |
| 其他模型无法使用搜索 | ✅ DuckDuckGo 免费回退 |

新增搜索后端：

| 后端 | 适用模型 | 说明 |
|------|---------|------|
| Responses API | Grok, GPT-4 | 官方原有，保留 |
| Messages API（新增） | DeepSeek, Claude | Anthropic web_search 工具 |
| Chat Completions（新增） | OpenAI 搜索模型 | web_search_options 参数 |
| DuckDuckGo（新增） | 所有模型 | 免费，零配置，自动回退 |

搜索后端根据模型的 `api_backend` 配置**自动选择**，无需手动设置。

### 计划中的改动

- [ ] Windows 编译修复
- [ ] 系统提示词汉化
- [ ] 去 xAI 品牌化
- [ ] Glob 文件匹配工具
- [ ] 桌面 GUI（基于 Caelum Tauri 壳）

## 构建

```bash
# 需要 Rust 工具链
cargo build -p xai-grok-pager-bin --release
```

## 贡献

欢迎提交 PR。我们接受来自社区的改进、修复和建议。

## 许可证

Apache 2.0，与上游一致。

## 免责声明

本项目与 SpaceXAI / xAI 无任何关联，不提供任何担保。模型推理仍需要使用你自己的 API 密钥。
