<!-- 语言切换 -->
<div align="right">
  <a href="README.md">English</a> | <a href="README_CN.md">中文</a>
</div>

# Grok Build 社区版

[![CI](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml/badge.svg)](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml)

基于 [SpaceXAI Grok Build](https://github.com/xai-org/grok-build) 的独立社区维护分支。

**定位**：Grok Build 社区版 之于 官方 Grok Build，如同 [VSCodium](https://github.com/VSCodium/vscodium) 之于 VS Code。

---

## 做了什么

### 隐私
- 删除 Mixpanel 用户行为分析
- 删除 Sentry 崩溃上报
- 删除 Google Cloud 会话上传
- 删除 OpenTelemetry 数据导出
- 遥测数据改为 **本地存储**（`~/.grok/logs/`）
- **阻止**自动更新

### 搜索 — 4 种后端
根据模型自动适配：

| 后端 | 适用模型 |
|------|---------|
| Responses API | Grok、GPT-4 |
| Messages API | DeepSeek、Claude |
| Chat Completions | OpenAI 搜索模型 |
| DuckDuckGo | 所有模型（免费回退） |

### 新增工具
| 工具 | 说明 |
|------|------|
| `glob` | 文件模式匹配（`*`、`**`、`?`） |
| `calculator` | 数学表达式 + 统计函数 |
| `json_query` | JSON 查询/过滤/聚合 |
| `csv_ops` | CSV 查询/排序/分组 |
| `text` | 正则提取/替换/统计 |
| `codec` | 编解码 + 哈希 |

### 提示词优化
优化的系统提示词，减少 AI 腔调，降低幻觉。

---

## 安装

### 一条命令安装

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.ps1 | iex
```

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.sh | bash
```

安装后运行：
```
grokce
```

预编译二进制也发布在 [Releases](https://github.com/Cashmeran/grok-build-community/releases) 页面。

### 从源码构建
```bash
cargo build -p xai-grok-pager-bin --release
```

多模型配置：

```toml
# ~/.grok/config.toml
[models]
default = "deepseek"

[model.deepseek]
model = "deepseek-v4-pro"
base_url = "https://api.deepseek.com/v1"
env_key = "DEEPSEEK_API_KEY"
context_window = 1000000
```

---

## 参与贡献

欢迎提交 PR。

## 许可证

Apache 2.0，与上游一致。

## 免责声明

与 SpaceXAI / xAI 无关。使用你自己的 API 密钥。
