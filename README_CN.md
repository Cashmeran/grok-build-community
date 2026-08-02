<!-- 璇█鍒囨崲 -->
<div align="right">
  <a href="README.md">English</a> | <a href="README_CN.md">涓枃</a>
</div>

# Grok Build 绀惧尯鐗?
[![CI](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml/badge.svg)](https://github.com/Cashmeran/grok-build-community/actions/workflows/ci.yml)

鍩轰簬 [SpaceXAI Grok Build](https://github.com/xai-org/grok-build) 鐨勭嫭绔嬬ぞ鍖虹淮鎶ゅ垎鏀€?
**瀹氫綅**锛欸rok Build 绀惧尯鐗?涔嬩簬 瀹樻柟 Grok Build锛屽鍚?[VSCodium](https://github.com/VSCodium/vscodium) 涔嬩簬 VS Code銆?
---

## 鍋氫簡浠€涔?
### 闅愮
- 鍒犻櫎 Mixpanel 鐢ㄦ埛琛屼负鍒嗘瀽
- 鍒犻櫎 Sentry 宕╂簝涓婃姤
- 鍒犻櫎 Google Cloud 浼氳瘽涓婁紶
- 鍒犻櫎 OpenTelemetry 鏁版嵁瀵煎嚭
- 閬ユ祴鏁版嵁鏀逛负 **鏈湴瀛樺偍**锛坄~/.grok/logs/`锛?- **闃绘**鑷姩鏇存柊

### 鎼滅储 鈥?4 绉嶅悗绔?鏍规嵁妯″瀷鑷姩閫傞厤锛?
| 鍚庣 | 閫傜敤妯″瀷 |
|------|---------|
| Responses API | Grok銆丟PT-4 |
| Messages API | DeepSeek銆丆laude |
| Chat Completions | OpenAI 鎼滅储妯″瀷 |
| DuckDuckGo | 鎵€鏈夋ā鍨嬶紙鍏嶈垂鍥為€€锛?|

### 鏂板宸ュ叿
| 宸ュ叿 | 璇存槑 |
|------|------|
| `glob` | 鏂囦欢妯″紡鍖归厤锛坄*`銆乣**`銆乣?`锛?|
| `calculator` | 鏁板琛ㄨ揪寮?+ 缁熻鍑芥暟 |
| `json_query` | JSON 鏌ヨ/杩囨护/鑱氬悎 |
| `csv_ops` | CSV 鏌ヨ/鎺掑簭/鍒嗙粍 |
| `text` | 姝ｅ垯鎻愬彇/鏇挎崲/缁熻 |
| `codec` | 缂栬В鐮?+ 鍝堝笇 |

### 鎻愮ず璇嶄紭鍖?浼樺寲鐨勭郴缁熸彁绀鸿瘝锛屽噺灏?AI 鑵旇皟锛岄檷浣庡够瑙夈€?
### 澶氳瑷€锛坴0.2.109+锛?鏀寔 6 绉嶈瑷€锛岃嚜鍔ㄦ娴嬬郴缁熻瑷€锛宍/lang` 瀹炴椂鍒囨崲锛?
| `/lang en` | English | `/lang zh-CN` | 涓枃 |
| `/lang ja` | 鏃ユ湰瑾?| `/lang ko` | 頃滉淡鞏?|
| `/lang ru` | 袪褍褋褋泻懈泄 | `/lang fr` | Fran莽ais |

278 鏉＄炕璇戣鐩栧叏閮ㄧ敤鎴风晫闈€?
---

## 瀹夎

### 涓€鏉″懡浠ゅ畨瑁?
**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.ps1 | iex
```

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.sh | bash
```

瀹夎鍚庤繍琛岋細
```
grokce
```

棰勭紪璇戜簩杩涘埗涔熷彂甯冨湪 [Releases](https://github.com/Cashmeran/grok-build-community/releases) 椤甸潰銆?
### 浠庢簮鐮佹瀯寤?```bash
cargo build -p xai-grok-pager-bin --release
```

澶氭ā鍨嬮厤缃細

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

## 鏂囨。

| 鏂囨。 | 鍐呭 |
|------|------|
| [CHANGELOG](docs/CHANGELOG.md) | 鐗堟湰鍘嗗彶 |
| [鏇存柊鏃ュ織](docs/CHANGELOG_CN.md) | 涓枃鏇存柊鏃ュ織 |
| [Maintenance Guide](docs/MAINTENANCE.md) | 寮€鍙戜笌涓婃父鍚屾 |
| [缁存姢鎸囧崡](docs/MAINTENANCE_CN.md) | 涓枃缁存姢鎸囧崡 |
| [Patches](patches/README.md) | 琛ヤ竵璇存槑 |

---

## 鍙備笌璐＄尞

娆㈣繋鎻愪氦 PR銆傝瑙?[docs/MAINTENANCE_CN.md](docs/MAINTENANCE_CN.md)銆?
## 璁稿彲璇?
Apache 2.0锛屼笌涓婃父涓€鑷淬€?
## 鍏嶈矗澹版槑

涓?SpaceXAI / xAI 鏃犲叧銆備娇鐢ㄤ綘鑷繁鐨?API 瀵嗛挜銆?
