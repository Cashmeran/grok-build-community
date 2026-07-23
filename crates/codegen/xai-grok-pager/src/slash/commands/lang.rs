//! `/lang` — switch the display language.
//!
//! When invoked with no arguments, the prompt widget shows a dropdown
//! with available languages. Selecting one switches immediately.
//! You can also type `/lang en` or `/lang zh-CN` directly.

use crate::slash::command::{AppCtx, ArgItem, CommandExecCtx, CommandResult, SlashCommand};
use xai_grok_i18n::tr;

pub struct LangCommand;

impl SlashCommand for LangCommand {
    fn name(&self) -> &str {
        "lang"
    }

    fn aliases(&self) -> &[&str] {
        &["language"]
    }

    fn description(&self) -> &str {
        tr!("Switch language")
    }

    fn usage(&self) -> &str {
        "/lang [en|zh-CN]"
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
        let items = vec![
            ArgItem {
                display: format!(
                    "{}  {}",
                    if current == "en" { ">" } else { " " },
                    tr!("English")
                ),
                match_text: "en".to_string(),
                insert_text: "en".to_string(),
                description: tr!("English").to_string(),
            },
            ArgItem {
                display: format!(
                    "{}  {}",
                    if current == "zh-CN" { ">" } else { " " },
                    tr!("中文")
                ),
                match_text: "zh-CN".to_string(),
                insert_text: "zh-CN".to_string(),
                description: tr!("中文").to_string(),
            },
        ];
        Some(items)
    }

    fn run(&self, _ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let code = args.trim();
        if code.is_empty() {
            return CommandResult::Message(format!(
                "Current language: {}. Type /lang en or /lang zh-CN to switch, or press Tab to select.",
                xai_grok_i18n::current_lang()
            ));
        }

        xai_grok_i18n::set_lang(code);
        CommandResult::Message(format!(
            "Language switched to: {}",
            xai_grok_i18n::current_lang()
        ))
    }
}
