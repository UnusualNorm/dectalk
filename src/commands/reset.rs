use crate::{Error, commands::Context};

#[poise::command(slash_command, ephemeral)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();
    ctx.data().remove_voice(author.id).await?;
    let voice = ctx.data().get_voice(author.id).await;
    ctx.say(format!("```rust\n{:?}\n```", voice)).await?;
    Ok(())
}
