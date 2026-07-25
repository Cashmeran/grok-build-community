//! Lightweight internationalization (i18n) for Grok Build Community Edition.
//!
//! # Design
//!
//! - Keys are the English strings themselves (self-documenting, natural fallback).
//! - Translations live in JSON files under `src/translations/`, embedded at compile
//!   time via `include_str!`.
//! - A `static` lookup table is loaded once on first use; the `/lang` command
//!   can switch it at runtime.
//!
//! # Usage
//!
//! ```ignore
//! use xai_grok_i18n::tr;
//!
//! let text: &'static str = tr!("Exit Grok Build");
//! // English → "Exit Grok Build"
//! // Chinese → "退出 Grok Build"
//! ```
//!
//! The macro works in const and non-const contexts and always returns `&'static str`.

use std::collections::HashMap;
use std::sync::OnceLock;

use parking_lot::RwLock;

// ---------------------------------------------------------------------------
// Built-in translation data (compile-time embedded)
// ---------------------------------------------------------------------------

const TRANSLATION_ZH_CN: &str = include_str!("translations/zh-CN.json");
const TRANSLATION_JA: &str = include_str!("translations/ja.json");
const TRANSLATION_KO: &str = include_str!("translations/ko.json");
const TRANSLATION_RU: &str = include_str!("translations/ru.json");
const TRANSLATION_FR: &str = include_str!("translations/fr.json");

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

static LANG: OnceLock<RwLock<String>> = OnceLock::new();
static TRANSLATIONS: OnceLock<RwLock<HashMap<&'static str, &'static str>>> = OnceLock::new();

fn lang_cell() -> &'static RwLock<String> {
    LANG.get_or_init(|| {
        let detected = detect_system_lang();
        let normalized = normalize_lang(&detected);
        tracing::info!(lang = %normalized, "i18n auto-detected system language");
        RwLock::new(normalized)
    })
}

fn translations_cell() -> &'static RwLock<HashMap<&'static str, &'static str>> {
    TRANSLATIONS.get_or_init(|| {
        let lang = lang_cell().read().clone();
        let map = load_translations(&lang);
        RwLock::new(map)
    })
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Auto-detect the system language and initialize translations.
///
/// Should be called once at startup, before any `tr!()` calls.
/// Checks (in order): saved preference in `~/.grok/lang`, environment variables
/// (`LANG`, `LC_ALL`), and platform-specific locale APIs. Falls back to `"en"`.
pub fn init() {
    let normalized = load_lang_preference().unwrap_or_else(|| {
        let detected = detect_system_lang();
        normalize_lang(&detected)
    });
    let map = load_translations(&normalized);
    *lang_cell().write() = normalized.clone();
    *translations_cell().write() = map;
    tracing::info!(lang = %normalized, "i18n initialized");
}

/// Return the currently active language code (e.g. `"en"`, `"zh-CN"`).
pub fn current_lang() -> String {
    lang_cell().read().clone()
}

/// Switch the active language at runtime and persist the preference.
///
/// Supported values: `"en"`, `"zh-CN"`. Unknown values fall back to `"en"`.
/// The new translations take effect immediately for all subsequent `tr!()` calls.
/// The choice is saved to `~/.grok/lang` for future sessions.
pub fn set_lang(lang: &str) {
    let normalized = normalize_lang(lang);
    let map = load_translations(&normalized);
    *lang_cell().write() = normalized;
    *translations_cell().write() = map;
    save_lang_preference(lang);
    tracing::info!(lang = %lang, "i18n language switched and saved");
}

/// Translate a string key into the active language.
///
/// Returns the translated `&'static str` if a mapping exists, otherwise returns
/// `key` unchanged (English fallback).
///
/// Prefer the [`tr`] macro for ergonomic usage.
pub fn translate(key: &str) -> &'static str {
    let guard = translations_cell().read();
    guard.get(key).copied().unwrap_or_else(|| {
        // Leak the key to make it 'static only when we need to return it.
        // This happens exactly once per untranslated key — acceptable in a
        // long-running TUI process.
        let leaked: &'static str = Box::leak(key.to_string().into_boxed_str());
        leaked
    })
}

/// Macro form of [`translate`].  Accepts a string literal and returns `&'static str`.
///
/// # Example
///
/// ```ignore
/// let s: &'static str = tr!("Hello, world!");
/// ```
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::translate($key)
    };
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn normalize_lang(raw: &str) -> String {
    let t = raw.trim().to_lowercase();
    match t.as_str() {
        "zh" | "zh-cn" | "zh_cn" | "zhcn" | "chinese" => "zh-CN".to_string(),
        "ja" | "ja-jp" | "japanese" => "ja".to_string(),
        "ko" | "ko-kr" | "korean" => "ko".to_string(),
        "ru" | "ru-ru" | "russian" => "ru".to_string(),
        "fr" | "fr-fr" | "french" => "fr".to_string(),
        _ => "en".to_string(),
    }
}

/// Detect the system's preferred language from environment variables.
///
/// Checks (in order):
/// 1. `LC_ALL` env var
/// 2. `LC_MESSAGES` env var
/// 3. `LANG` env var (Unix/macOS)
/// 4. `LANGUAGE` env var (GNU gettext)
/// 5. Falls back to `"en"` if nothing matches
///
/// Supported auto-detected locales: zh, ja, ko, ru, fr. All others → en.
fn detect_system_lang() -> String {
    for var in &["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
        if let Ok(val) = std::env::var(var) {
            let lang = val.split('.').next().unwrap_or(&val);
            if !lang.is_empty() && lang != "C" && lang != "POSIX" {
                return lang.to_string();
            }
        }
    }
    "en".to_string()
}

fn load_translations(lang: &str) -> HashMap<&'static str, &'static str> {
    match lang {
        "zh-CN" => parse_translations(TRANSLATION_ZH_CN),
        "ja" => parse_translations(TRANSLATION_JA),
        "ko" => parse_translations(TRANSLATION_KO),
        "ru" => parse_translations(TRANSLATION_RU),
        "fr" => parse_translations(TRANSLATION_FR),
        _ => HashMap::new(), // English = identity, empty map
    }
}

fn parse_translations(json: &str) -> HashMap<&'static str, &'static str> {
    let raw: HashMap<String, String> = serde_json::from_str(json).unwrap_or_else(|e| {
        tracing::warn!("Failed to parse translations: {e}");
        HashMap::new()
    });
    // Leak all strings to make them 'static — the translations live for the
    // entire process lifetime, so this is intentional and not a leak.
    raw.into_iter()
        .map(|(k, v)| {
            let k: &'static str = Box::leak(k.into_boxed_str());
            let v: &'static str = Box::leak(v.into_boxed_str());
            (k, v)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Persistence helpers
// ---------------------------------------------------------------------------

fn lang_pref_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".grok").join("lang"))
}
#[allow(dead_code)]
pub(crate) fn clear_lang_preference() {
    if let Some(p) = lang_pref_path() { let _ = std::fs::remove_file(&p); }
}

/// Try to load a saved language preference from `~/.grok/lang`.
fn load_lang_preference() -> Option<String> {
    let path = lang_pref_path()?;
    let content = std::fs::read_to_string(&path).ok()?;
    let lang = content.trim();
    if lang.is_empty() { None } else { Some(normalize_lang(lang)) }
}

/// Save the current language preference to `~/.grok/lang`.
fn save_lang_preference(lang: &str) {
    if let Some(path) = lang_pref_path() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, lang);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lang_falls_back_to_en() {
        // Default depends on system locale; on CI (LANG=C) it's "en".
        // We test the fallback path directly via normalize_lang.
        assert_eq!(normalize_lang("C"), "en");
        assert_eq!(normalize_lang("POSIX"), "en");
        assert_eq!(normalize_lang(""), "en");
    }

    #[test]
    fn english_is_identity() {
        assert_eq!(translate("Hello"), "Hello");
        assert_eq!(translate("File"), "File");
    }

    #[test]
    fn switch_to_chinese() {
        clear_lang_preference();
        set_lang("en");
        set_lang("zh-CN");
        assert_eq!(current_lang(), "zh-CN");
        // Reset for other tests — also clear persistence
        set_lang("en");
        clear_lang_preference();
    }

    #[test]
    fn normalize_variants() {
        assert_eq!(normalize_lang("zh"), "zh-CN");
        assert_eq!(normalize_lang("zh-cn"), "zh-CN");
        assert_eq!(normalize_lang("ZH_CN"), "zh-CN");
        assert_eq!(normalize_lang("en"), "en");
        assert_eq!(normalize_lang("fr"), "fr");
        assert_eq!(normalize_lang("de"), "en"); // unknown → en
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod translation_integration_tests;
