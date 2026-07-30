# Grok Build 社区版 - 维护指南

## 目录结构

```
├── patches/          ← 17 个补丁，每个对应一项 CE 改动
├── ce-tools/         ← CE 工具源码（上游更新不会覆盖）
├── docs/             ← 文档
├── build.bat         ← 一键构建：apply patches → cargo build
└── README.md         ← 项目介绍
```

## 日常操作

### 上游更新后
```bash
git pull upstream main
./build.bat
# 哪个补丁失败就修哪个：改代码 → git diff > patches/失败的补丁.patch
```

### 新增 CE 功能
```bash
# 1. 正常改代码
# 2. 导出为补丁
git diff > patches/18-功能名.patch
# 3. 更新 build.bat 中的补丁列表
# 4. 更新本文档和 CHANGELOG
```

### 修改 CE 工具
直接在 `ce-tools/src/` 中编辑。这些是参考副本——实际编译走 patch 系统。

## 补丁列表

| # | 文件 | 功能 |
|---|------|------|
| 01 | remove-telemetry | 删除 Mixpanel、Sentry、Google Cloud 上传 |
| 02 | web-search | 多后端搜索（Messages、Chat、DuckDuckGo） |
| 03 | community-docs | 社区版文档（中英双版） |
| 04 | add-glob-tool | glob 文件匹配 |
| 05 | add-calculator-tool | 计算器 + 统计 |
| 06 | add-json-csv-tools | json_query + csv_ops |
| 07 | add-text-codec-tools | text + codec |
| 08 | community-branding | CE 品牌 |
| 09 | optimize-system-prompt | 系统提示词优化 |
| 10 | community-tool-docs | CE 工具文档 |
| 11 | ci-cd-build | CI/CD + 阻止自动更新 |
| 12 | install-scripts | 一键安装脚本 |
| 13 | clean-dead-code | 清理死代码 |
| 14 | inject-tools | 6 个 CE 工具注册到所有 toolset |
| 15 | binary-name-grokce | 命令名为 grokce |
| 16 | websearch-zero-config | 搜索零配置 |
| 17 | ci-fixes | CI fmt + clippy 修复 |

## 架构

与 VSCodium、ungoogled-chromium、LineageOS 相同：
- 上游代码保持原样
- CE 改动存为独立补丁
- 每个补丁独立失败，易于诊断
