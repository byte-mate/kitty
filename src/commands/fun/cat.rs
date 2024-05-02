use poise::command;
use crate::{BotError};
use crate::Context;

/// Random cat image
#[command(slash_command)]
pub async fn cat(
    ctx: Context<'_>,
) -> Result<(), BotError> {
    let cat = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await?
        .json::<serde_json::Value>()
        .await?;
    let cat_url = cat[0]["url"].as_str().unwrap();

    let reply: poise::CreateReply = ctx.reply_builder(poise::CreateReply::default()).content(cat_url);


    ctx.send(reply).await?;

    Ok(())
}