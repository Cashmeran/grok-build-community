//! `/lang` — switch the display language.

use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

pub struct LangCommand;

impl SlashCommand for LangCommand {
    fn name(&self) -> &str {
        "lang"
    }

    fn aliases(&self) -> &[&str] {
        &["language"]
    }

    fn description(&self) -> &str {
        xai_grok_i18n::tr!("Switch language")
    }

    fn usage(&self) -> &str {
        xai_grok_i18n::tr!("/lang <code>")
    }

    fn takes_args(&self) -> bool {
        true
    }

    fn run(&self, _ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let code = args.trim();
        if code.is_empty() {
            let current = xai_grok_i18n::current_lang();
            return CommandResult::Message(format!(
                "Current language: {current}. Available: en, zh-CN"
            ));
        }

        xai_grok_i18n::set_lang(code);
        CommandResult::Message(format!("Language switched to: {}", xai_grok_i18n::current_lang()))
    }
}
