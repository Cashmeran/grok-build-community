//! `/resume` -- open session picker overlay to resume a previous session.

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};
use xai_grok_i18n::tr;

pub struct ResumeCommand;

impl SlashCommand for ResumeCommand {
    fn name(&self) -> &str {
        "resume"
    }

    fn description(&self) -> &str {
        tr!("Resume a previous session")
    }

    fn usage(&self) -> &str {
        "/resume"
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        CommandResult::Action(Action::ShowSessionPicker)
    }
}
