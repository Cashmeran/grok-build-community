//! `/feedback` -- send session feedback.

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};
use xai_grok_i18n::tr;

/// Send session feedback inline or enter feedback mode.
pub struct FeedbackCommand;

impl SlashCommand for FeedbackCommand {
    fn name(&self) -> &str {
        "feedback"
    }

    fn description(&self) -> &str {
        tr!("Send feedback about the current session")
    }

    fn usage(&self) -> &str {
        tr!("/feedback [text]")
    }

    fn takes_args(&self) -> bool {
        true
    }

    fn arg_placeholder(&self) -> Option<&str> {
        Some("[feedback text]")
    }

    fn run(&self, _ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let trimmed = args.trim();
        if trimmed.is_empty() {
            CommandResult::Action(Action::EnterFeedbackMode)
        } else {
            CommandResult::Action(Action::SendFeedback(trimmed.to_string()))
        }
    }
}
