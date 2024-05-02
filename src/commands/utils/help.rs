use poise::command;
use crate::{BotError};
use crate::Context;

/// Get help
#[command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>
) -> Result<(), BotError> {
    poise::builtins::pretty_help(
        ctx,
        command.as_deref(),
        poise::builtins::PrettyHelpConfiguration {
            ..Default::default()
        }
    ).await?;


    Ok(())
}