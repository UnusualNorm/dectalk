use poise::CreateReply;
use serenity::all::*;

use crate::{Error, commands::Context, dectalk};

#[poise::command(slash_command)]
pub async fn test(ctx: Context<'_>, text: String) -> Result<(), Error> {
    let author = ctx.author();
    let voice = ctx.data().get_voice(author.id).await;
    let bytes = dectalk::say(&text, &voice, ctx.id()).await?;
    ctx.send(CreateReply::default().attachment(CreateAttachment::bytes(bytes, "test.wav")))
        .await?;
    Ok(())
}
