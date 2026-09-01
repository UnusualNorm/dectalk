use std::future;

use futures::{Stream, StreamExt, stream};

use crate::{
    Error,
    commands::Context,
    dectalk::{
        BETTY_VOICE, DENNIS_VOICE, FRANK_VOICE, HARRY_VOICE, KIT_VOICE, PAUL_VOICE, RITA_VOICE,
        URSULA_VOICE, WENDY_VOICE,
    },
};

async fn autocomplete_preset<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    stream::iter(&[
        "Paul", "Harry", "Frank", "Dennis", "Betty", "Ursula", "Wendy", "Rita", "Kit",
    ])
    .filter(move |name| future::ready(name.starts_with(partial)))
    .map(|name| name.to_string())
}

#[poise::command(slash_command)]
pub async fn preset(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_preset"] voice: String,
) -> Result<(), Error> {
    let author = ctx.author();

    let voice = match voice.as_str() {
        "Paul" => PAUL_VOICE,
        "Harry" => HARRY_VOICE,
        "Frank" => FRANK_VOICE,
        "Dennis" => DENNIS_VOICE,
        "Betty" => BETTY_VOICE,
        "Ursula" => URSULA_VOICE,
        "Wendy" => WENDY_VOICE,
        "Rita" => RITA_VOICE,
        "Kit" => KIT_VOICE,
        _ => {
            ctx.say("Invalid voice preset!").await?;
            return Ok(());
        }
    };

    ctx.say(format!("```rust\n{:?}\n```", voice)).await?;
    ctx.data().set_voice(author.id, voice).await?;
    Ok(())
}
