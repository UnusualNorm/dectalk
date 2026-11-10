use std::{
    collections::{HashMap, HashSet},
    env,
};

use commands::commands;
use dectalk::Voice;
use dotenv::dotenv;
use poise::serenity_prelude::*;
use serde_data::{SerdeData, SerdeDataError};
use songbird::{SerenityInit, input::Input};

mod commands;
mod dectalk;
mod serde_data;
mod utils;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

struct Data {
    voice_data: SerdeData<HashMap<u64, Voice>>,
    mute_data: SerdeData<HashMap<u64, HashSet<u64>>>,
    prefix_data: SerdeData<HashMap<u64, String>>,
    invert_data: SerdeData<HashSet<u64>>,
    tts_prefix: String,
    tts_peak: f32,
    tts_len: usize,
}

impl Data {
    async fn get_voice(&self, user_id: UserId) -> Voice {
        self.voice_data
            .get()
            .await
            .get(&user_id.get())
            .cloned()
            .unwrap_or(dectalk::random(user_id.get()))
    }

    async fn set_voice(&self, user_id: UserId, voice: Voice) -> Result<(), SerdeDataError> {
        self.voice_data.get_mut().await.insert(user_id.get(), voice);
        self.voice_data.save().await?;
        Ok(())
    }

    async fn remove_voice(&self, user_id: UserId) -> Result<(), SerdeDataError> {
        self.voice_data.get_mut().await.remove(&user_id.get());
        self.voice_data.save().await?;
        Ok(())
    }

    async fn get_muted(&self, guild_id: GuildId, user_id: UserId) -> bool {
        self.mute_data
            .get()
            .await
            .get(&guild_id.get())
            .map(|set| set.contains(&user_id.get()))
            .unwrap_or(false)
    }

    async fn set_muted(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        muted: bool,
    ) -> Result<(), SerdeDataError> {
        let mut mute_data = self.mute_data.get_mut().await;
        let set = mute_data.entry(guild_id.get()).or_insert_with(HashSet::new);
        if muted {
            set.insert(user_id.get());
        } else {
            set.remove(&user_id.get());
        }
        drop(mute_data);
        self.mute_data.save().await?;
        Ok(())
    }

    async fn get_prefix(&self, guild_id: GuildId) -> String {
        self.prefix_data
            .get()
            .await
            .get(&guild_id.get())
            .cloned()
            .unwrap_or_else(|| self.tts_prefix.clone())
    }

    async fn set_prefix(&self, guild_id: GuildId, prefix: String) -> Result<(), SerdeDataError> {
        self.prefix_data
            .get_mut()
            .await
            .insert(guild_id.get(), prefix);
        self.prefix_data.save().await?;
        Ok(())
    }

    async fn get_inverted(&self, user_id: UserId) -> bool {
        self.invert_data.get().await.contains(&user_id.get())
    }

    async fn set_inverted(&self, user_id: UserId, inverted: bool) -> Result<(), SerdeDataError> {
        let mut invert_data = self.invert_data.get_mut().await;
        if inverted {
            invert_data.insert(user_id.get());
        } else {
            invert_data.remove(&user_id.get());
        }
        drop(invert_data);
        self.invert_data.save().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut owners = HashSet::new();
    for owner in env::var("DISCORD_OWNERS")
        .expect("missing OWNERS")
        .split(',')
    {
        owners.insert(UserId::new(owner.parse().expect("invalid owner")));
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            owners,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    voice_data: SerdeData::new("voices").await.expect("invalid voices"),
                    mute_data: SerdeData::new("mutes").await.expect("invalid mutes"),
                    prefix_data: SerdeData::new("prefixes").await.expect("invalid prefixes"),
                    invert_data: SerdeData::new("inverts").await.expect("invalid inverts"),
                    tts_prefix: env::var("TTS_PREFIX").expect("missing TTS_PREFIX"),
                    tts_len: env::var("TTS_LEN")
                        .expect("missing TTS_LEN")
                        .parse::<usize>()
                        .expect("invalid TTS_LEN"),
                    tts_peak: env::var("TTS_PEAK")
                        .expect("missing TTS_PEAK")
                        .parse::<f32>()
                        .expect("invalid TTS_PEAK"),
                })
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Message { new_message, .. } => {
            message_event_handler(ctx, new_message, framework, data).await
        }
        FullEvent::VoiceStateUpdate { old, new } => {
            voice_state_update_event_handler(ctx, old, new, framework, data).await
        }
        _ => Ok(()),
    }
}

async fn message_event_handler(
    ctx: &Context,
    msg: &Message,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if msg.author.bot {
        return Ok(());
    }

    let Some(guild_id) = msg.guild_id else {
        return Ok(());
    };

    let mut text = msg.content.clone();
    let prefix = data.get_prefix(guild_id).await;
    match (
        data.get_inverted(msg.author.id).await,
        msg.content.starts_with(&prefix),
    ) {
        (true, true) => return Ok(()),
        (true, false) => (),
        (false, true) => text = text[prefix.len()..].to_string(),
        (false, false) => return Ok(()),
    }

    let Ok(channel) = msg.channel_id.to_channel(ctx).await else {
        return Ok(());
    };

    let Some(guild_channel) = channel.guild() else {
        return Ok(());
    };

    if guild_channel.kind != ChannelType::Voice {
        return Ok(());
    }

    let members = guild_channel.members(ctx)?;
    if members.len() == 0 {
        return Ok(());
    }

    if data.get_muted(guild_id, msg.author.id).await {
        msg.react(ctx, '❌').await?;
        return Ok(());
    }

    text = utils::replace_links(&text);
    text = utils::replace_emojis(&text);
    text = content_safe(&ctx, &text, &ContentSafeOptions::default(), &msg.mentions);
    text.retain(|character| character.is_ascii());
    text = text.trim().to_string();
    if text.len() == 0 {
        return Ok(());
    }

    if text.len() > framework.user_data.tts_len
        && !framework.options.owners.contains(&msg.author.id)
    {
        text = text[..framework.user_data.tts_len].to_string();
        msg.react(ctx, '⚠').await?;
    }

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    let handler_lock = manager.get_or_insert(guild_id);
    {
        let mut handler = handler_lock.lock().await;
        handler.deafen(true).await?;
        handler.join(msg.channel_id).await?
    }
    .await?;

    let voice = data.get_voice(msg.author.id).await;
    let mut bytes = match dectalk::say(&text, &voice, msg.id.get()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            msg.react(ctx, '❌').await?;
            return Err(Box::new(err));
        }
    };

    bytes = utils::normalize_wav(&bytes, framework.user_data.tts_peak)?;
    handler_lock.lock().await.play_input(Input::from(bytes));
    Ok(())
}

async fn voice_state_update_event_handler(
    ctx: &Context,
    old: &Option<VoiceState>,
    _new: &VoiceState,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    let old = match old {
        Some(old) => old,
        None => return Ok(()),
    };

    // let channel_id = match old.channel_id {
    //     Some(channel_id) => channel_id,
    //     None => return Ok(()),
    // };

    let guild_id = match old.guild_id {
        Some(guild_id) => guild_id,
        None => return Ok(()),
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => return Ok(()),
    };

    let mut handler = handler_lock.lock().await;

    let channel_id = match handler.current_channel() {
        Some(channel) => ChannelId::new(channel.0.get()),
        None => return Ok(()),
    };

    // if current_channel_id.0.get() != channel_id.get() {
    //     return Ok(());
    // }

    let channel = match channel_id.to_channel(ctx).await {
        Ok(channel) => channel,
        Err(_) => return Ok(()),
    };

    let guild_channel = match channel.guild() {
        Some(guild_channel) => guild_channel,
        None => return Ok(()),
    };

    let members = match guild_channel.members(ctx) {
        Ok(members) => members,
        Err(_) => return Ok(()),
    };

    if members.len() <= 1 {
        handler.leave().await?;
    }

    Ok(())
}
