//! `/timestamps` -- toggle timestamp display on messages.
//!
//! This command computes the new value itself and dispatches the typed
//! `Action::SetTimestamps(bool)`.

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};
use xai_grok_i18n::tr;

pub struct TimestampsCommand;

impl SlashCommand for TimestampsCommand {
    fn name(&self) -> &str {
        "timestamps"
    }

    fn description(&self) -> &str {
        tr!("Toggle message timestamps on/off")
    }

    fn usage(&self) -> &str {
        "/timestamps"
    }

    fn arg_placeholder(&self) -> Option<&str> {
        Some("on/off")
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        let new = !crate::appearance::cache::load_timestamps();
        CommandResult::Action(Action::SetTimestamps(new))
    }
}
