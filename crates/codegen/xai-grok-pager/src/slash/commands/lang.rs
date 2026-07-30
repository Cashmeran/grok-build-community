//! `/lang` — switch the display language.
//!
//! When invoked with no arguments, the prompt widget shows a dropdown
//! with available languages. Selecting one switches immediately.
//! Supported: en, zh-CN, ja, ko, ru, fr.

use crate::app::actions::Action;
use crate::slash::command::{AppCtx, ArgItem, CommandExecCtx, CommandResult, SlashCommand};

pub struct LangCommand;

impl SlashCommand for LangCommand {
    fn name(&self) -> &str {
        "lang"
    }

    fn aliases(&self) -> &[&str] {
        &["language"]
    }

    fn description(&self) -> &str {
        "Switch display language (saved across sessions)"
    }

    fn usage(&self) -> &str {
        "/lang [en|zh-CN|ja|ko|ru|fr]"
    }

    fn takes_args(&self) -> bool {
        true
    }

    fn args_required(&self) -> bool {
        false
    }

    fn arg_placeholder(&self) -> Option<&str> {
        Some("[language]")
    }

    fn suggest_args(&self, _ctx: &AppCtx, _args_query: &str) -> Option<Vec<ArgItem>> {
        let current = xai_grok_i18n::current_lang();
        let entries: &[(&str, &str)] = &[
            ("en", "English"),
            ("zh-CN", "中文"),
            ("ja", "日本語"),
            ("ko", "한국어"),
            ("ru", "Русский"),
            ("fr", "Français"),
        ];
        let items: Vec<ArgItem> = entries
            .iter()
            .map(|(code, name)| {
                let marker = if current == *code { ">" } else { " " };
                ArgItem {
                    display: format!("{marker}  {name}"),
                    match_text: code.to_string(),
                    insert_text: code.to_string(),
                    description: name.to_string(),
                }
            })
            .collect();
        Some(items)
    }

    fn run(&self, _ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let code = args.trim();
        if code.is_empty() {
            return CommandResult::Message(format!(
                "Current language: {}. Type /lang <code> to switch, or press Tab to select.",
                xai_grok_i18n::current_lang()
            ));
        }

        xai_grok_i18n::set_lang(code);
        CommandResult::Action(Action::RefreshLanguage)
    }
}
