use serenity::all::*;

use crate::{Error, commands::Context};

#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    required_permissions = "MUTE_MEMBERS"
)]
pub async fn muted(ctx: Context<'_>, user: Member, muted: Option<bool>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    ctx.say(format!(
        "`{}`",
        if let Some(muted) = muted {
            ctx.data().set_muted(guild_id, user.user.id, muted).await?;
            muted
        } else {
            ctx.data().get_muted(guild_id, user.user.id).await
        }
    ))
    .await?;
    Ok(())
}
