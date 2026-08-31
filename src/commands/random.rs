use crate::{Error, commands::Context, dectalk};

#[poise::command(slash_command)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();
    let voice = dectalk::random(ctx.id());
    ctx.say(format!("```rust\n{:?}\n```", voice)).await?;
    ctx.data().set_voice(author.id, voice).await?;
    Ok(())
}
