use serenity::all::*;

use crate::{Error, commands::Context};

#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    required_permissions = "MANAGE_GUILD"
)]
pub async fn inverted(ctx: Context<'_>, inverted: Option<bool>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    ctx.say(format!(
        "`{}`",
        if let Some(inverted) = inverted {
            ctx.data().set_inverted(guild_id, inverted).await?;
            inverted
        } else {
            ctx.data().get_inverted(guild_id).await
        }
    ))
    .await?;
    Ok(())
}
