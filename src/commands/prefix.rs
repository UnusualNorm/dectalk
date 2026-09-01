use crate::{Error, commands::Context};

#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    required_permissions = "MANAGE_GUILD"
)]
pub async fn prefix(ctx: Context<'_>, prefix: Option<String>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        return Ok(());
    };

    ctx.say(format!(
        "`{}`",
        if let Some(prefix) = prefix {
            ctx.data().set_prefix(guild_id, prefix.clone()).await?;
            prefix
        } else {
            ctx.data().get_prefix(guild_id).await
        }
    ))
    .await?;
    Ok(())
}
