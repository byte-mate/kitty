use poise::{serenity_prelude as serenity};
use poise::command;
use crate::{BotError};
use crate::Context;

/// You can give tasty cookies to your friends!
#[command(slash_command)]
pub async fn cookie(
    ctx: Context<'_>,
    # [description = "Selected user"] user: Option<serenity::User>
) -> Result<(), BotError> {
    let u = user.as_ref().unwrap_or_else(|| &ctx.author());

    if u.id == ctx.http().get_current_user().await.unwrap().id {
        ctx.reply("🍪 **|** Thank you for the cookie!").await?;
    } else if u.id == ctx.author().id {
        ctx.reply("🍪 **|** You ate the cookie!").await?;
    } else {
        ctx.reply(format!("🍪 **|** {} gave {} a cookie!", ctx.author().name, u.name)).await?;
    }

    Ok(())
}