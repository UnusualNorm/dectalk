use serenity::all::*;

use crate::{Error, commands::Context};

#[poise::command(slash_command, ephemeral, required_permissions = "MANAGE_GUILD")]
pub async fn inverted(ctx: Context<'_>, inverted: Option<bool>) -> Result<(), Error> {
    ctx.say(format!(
        "`{}`",
        if let Some(inverted) = inverted {
            ctx.data().set_inverted(ctx.author().id, inverted).await?;
            inverted
        } else {
            ctx.data().get_inverted(ctx.author().id).await
        }
    ))
    .await?;
    Ok(())
}
