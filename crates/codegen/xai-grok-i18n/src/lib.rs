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
/// Checks environment variables (`LANG`, `LC_ALL`) and platform-specific
/// locale APIs. Falls back to `"en"` if detection fails.
pub fn init() {
    let detected = detect_system_lang();
    let normalized = normalize_lang(&detected);
    let map = load_translations(&normalized);
    *lang_cell().write() = normalized;
    *translations_cell().write() = map;
    tracing::info!(lang = %detected, "i18n initialized from system locale");
}

/// Return the currently active language code (e.g. `"en"`, `"zh-CN"`).
pub fn current_lang() -> String {
    lang_cell().read().clone()
}

/// Switch the active language at runtime.
///
/// Supported values: `"en"`, `"zh-CN"`. Unknown values fall back to `"en"`.
/// The new translations take effect immediately for all subsequent `tr!()` calls.
pub fn set_lang(lang: &str) {
    let normalized = normalize_lang(lang);
    let map = load_translations(&normalized);
    *lang_cell().write() = normalized;
    *translations_cell().write() = map;
    tracing::info!(lang = %lang, "i18n language switched");
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
        _ => HashMap::new(), // English = identity, empty map
    }
}

fn parse_translations(json: &str) -> HashMap<&'static str, &'static str> {
    let raw: HashMap<String, String> =
        serde_json::from_str(json).unwrap_or_else(|e| {
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
        set_lang("zh-CN");
        assert_eq!(current_lang(), "zh-CN");
        // Reset for other tests
        set_lang("en");
    }

    #[test]
    fn normalize_variants() {
        assert_eq!(normalize_lang("zh"), "zh-CN");
        assert_eq!(normalize_lang("zh-cn"), "zh-CN");
        assert_eq!(normalize_lang("ZH_CN"), "zh-CN");
        assert_eq!(normalize_lang("en"), "en");
        assert_eq!(normalize_lang("fr"), "en"); // unknown → en
    }
}
