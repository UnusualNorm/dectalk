use std::{collections::HashSet, env, sync::Arc};

use dectalk::tts;
use dotenv::dotenv;
use futures::{Stream, StreamExt, future, stream};
use mute_manager::MuteManager;
use poise::serenity_prelude as serenity;
use prefix_manager::PrefixManager;
use songbird::{SerenityInit, input::Input};
use tokio::{fs, sync::Mutex};
use voice_manager::VoiceManager;

mod dectalk;
mod mute_manager;
mod prefix_manager;
mod utils;
mod voice_manager;

struct Data {
    voice_manager: Arc<Mutex<VoiceManager>>,
    mute_manager: Arc<Mutex<MuteManager>>,
    prefix_manager: Arc<Mutex<PrefixManager>>,
    tts_peak: f32,
    tts_len: usize,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral)]
async fn voice(
    ctx: Context<'_>,
    #[description = "0-1 -- Sex 1 (male) or 0 (female)"] sx: Option<u8>,
    #[description = "65-145 % Head size"] hs: Option<u8>,
    #[description = "2000-4650 Hz Fourth formant frequency"] f4: Option<u16>,
    #[description = "2500-4950 Hz Fifth formant frequency"] f5: Option<u16>,
    #[description = "100-2048 Hz Fourth formant bandwidth"] b4: Option<u16>,
    #[description = "100-2048 Hz Fifth formant bandwidth"] b5: Option<u16>,
    #[description = "0-72 dB Breathiness"] br: Option<u8>,
    #[description = "0-100 % Lax breathiness"] lx: Option<u8>,
    #[description = "0-100 % Smoothness (high frequency attenuation)"] sm: Option<u8>,
    #[description = "0-100 % Richness"] ri: Option<u8>,
    #[description = "0-100 -- Number of fixed samplings of glottal pulse open phase"] nf: Option<
        u8,
    >,
    #[description = "0-100 % Laryngealization"] la: Option<u8>,
    #[description = "0-40 Hz Baseline fall"] bf: Option<u8>,
    #[description = "2-100 Hz Hat rise"] hr: Option<u8>,
    #[description = "1-100 Hz Stress rise"] sr: Option<u8>,
    #[description = "0-100 % Assertiveness"] as_: Option<u8>,
    #[description = "0-100 % Quickness"] qu: Option<u8>,
    #[description = "50-350 Hz Average pitch"] ap: Option<u16>,
    #[description = "0-250 % Pitch range"] pr: Option<u8>,
    #[description = "0-86 dB Gain of voicing source"] gv: Option<u8>,
    #[description = "0-86 dB Gain of aspiration source"] gh: Option<u8>,
    #[description = "0-86 dB Gain of frication source"] gf: Option<u8>,
    #[description = "0-86 dB Gain of nasalization"] gn: Option<u8>,
    // #[description = "0-86 dB Gain of first formant resonator"] g1: Option<u8>,
    // #[description = "0-86 dB Gain of second formant resonator"] g2: Option<u8>,
    // #[description = "0-86 dB Gain of third formant resonator"] g3: Option<u8>,
    // #[description = "0-86 dB Gain of fourth formant resonator"] g4: Option<u8>,
    // #[description = "0-86 dB Gain of fifth formant resonator (replaces lo)"] g5: Option<u8>,
) -> Result<(), Error> {
    let author = ctx.author();

    let mut voice_manager = ctx.data().voice_manager.lock().await;
    let voice = voice_manager.get(author.id.get());

    let voice = dectalk::DECtalkVoice {
        sx: sx.unwrap_or(voice.sx),
        hs: hs.unwrap_or(voice.hs),
        f4: f4.unwrap_or(voice.f4),
        f5: f5.unwrap_or(voice.f5),
        b4: b4.unwrap_or(voice.b4),
        b5: b5.unwrap_or(voice.b5),
        br: br.unwrap_or(voice.br),
        lx: lx.unwrap_or(voice.lx),
        sm: sm.unwrap_or(voice.sm),
        ri: ri.unwrap_or(voice.ri),
        nf: nf.unwrap_or(voice.nf),
        la: la.unwrap_or(voice.la),
        bf: bf.unwrap_or(voice.bf),
        hr: hr.unwrap_or(voice.hr),
        sr: sr.unwrap_or(voice.sr),
        as_: as_.unwrap_or(voice.as_),
        qu: qu.unwrap_or(voice.qu),
        ap: ap.unwrap_or(voice.ap),
        pr: pr.unwrap_or(voice.pr),
        gv: gv.unwrap_or(voice.gv),
        gh: gh.unwrap_or(voice.gh),
        gf: gf.unwrap_or(voice.gf),
        gn: gn.unwrap_or(voice.gn),
        // g1: g1.unwrap_or(voice.g1),
        // g2: g2.unwrap_or(voice.g2),
        // g3: g3.unwrap_or(voice.g3),
        // g4: g4.unwrap_or(voice.g4),
        // g5: g5.unwrap_or(voice.g5),
        g1: voice.g1,
        g2: voice.g2,
        g3: voice.g3,
        g4: voice.g4,
        g5: voice.g5,
    };

    if !voice.validate() {
        ctx.say("Invalid voice!").await?;
        return Ok(());
    }

    voice_manager.set(author.id.get(), &voice).await?;
    ctx.say(format!("```rust\n{:?}\n```", voice)).await?;
    Ok(())
}

async fn autocomplete_voice_preset<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    stream::iter(&[
        "Paul", "Harry", "Frank", "Dennis", "Betty", "Ursula", "Wendy", "Rita", "Kit",
    ])
    .filter(move |name| future::ready(name.starts_with(partial)))
    .map(|name| name.to_string())
}

#[poise::command(slash_command, ephemeral)]
async fn preset(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_voice_preset"] voice: String,
) -> Result<(), Error> {
    let author = ctx.author();

    let voice = match voice.as_str() {
        "Paul" => dectalk::PAUL_VOICE,
        "Harry" => dectalk::HARRY_VOICE,
        "Frank" => dectalk::FRANK_VOICE,
        "Dennis" => dectalk::DENNIS_VOICE,
        "Betty" => dectalk::BETTY_VOICE,
        "Ursula" => dectalk::URSULA_VOICE,
        "Wendy" => dectalk::WENDY_VOICE,
        "Rita" => dectalk::RITA_VOICE,
        "Kit" => dectalk::KIT_VOICE,
        _ => {
            ctx.say("Invalid voice preset!").await?;
            return Ok(());
        }
    };

    let mut voice_manager = ctx.data().voice_manager.lock().await;
    voice_manager.set(author.id.get(), &voice).await?;

    ctx.say(format!("```rust\n{:?}\n```", voice)).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();

    let mut voice_manager = ctx.data().voice_manager.lock().await;
    voice_manager.remove(author.id.get()).await?;

    let voice = voice_manager.get(author.id.get());
    ctx.say(format!("```rust\n{:?}\n```", voice)).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn test(ctx: Context<'_>, text: String) -> Result<(), Error> {
    let author = ctx.author();

    let voice_manager = ctx.data().voice_manager.lock().await;
    let voice = voice_manager.get(author.id.get());

    let tts_path = tts(&text, voice).await?;
    let tts_bytes = fs::read(&tts_path).await?;
    fs::remove_file(&tts_path).await?;

    ctx.send(
        poise::CreateReply::default()
            .attachment(serenity::CreateAttachment::bytes(tts_bytes, "tts.wav")),
    )
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    required_permissions = "MUTE_MEMBERS"
)]
async fn muted(ctx: Context<'_>, user: serenity::Member, muted: Option<bool>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => return Ok(()),
    };
    let user_id = user.user.id;

    let mut mute_manager = ctx.data().mute_manager.lock().await;

    let muted = if let Some(muted) = muted {
        mute_manager
            .set(guild_id.get(), user_id.get(), muted)
            .await?;
        muted
    } else {
        mute_manager.get(guild_id.get(), user_id.get())
    };

    ctx.say(format!("Muted: `{}`", muted)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    required_permissions = "MANAGE_GUILD"
)]
async fn prefix(ctx: Context<'_>, prefix: Option<String>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => return Ok(()),
    };

    let mut prefix_manager = ctx.data().prefix_manager.lock().await;
    let prefix = if let Some(prefix) = prefix {
        prefix_manager.set(guild_id.get(), &prefix).await?;
        prefix
    } else {
        prefix_manager.get(guild_id.get()).to_string()
    };

    ctx.say(format!("Prefix: `{}`", prefix)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut owners = HashSet::new();
    for owner in env::var("DISCORD_OWNERS")
        .expect("missing OWNERS")
        .split(',')
    {
        owners.insert(serenity::UserId::new(owner.parse().expect("invalid owner")));
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![preset(), voice(), reset(), test(), muted(), prefix()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            owners,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let mut voice_manager = VoiceManager::new();
                if voice_manager.can_load().await {
                    voice_manager.load().await?;
                }

                let mut mute_manager = MuteManager::new();
                if mute_manager.can_load().await {
                    mute_manager.load().await?;
                }

                let mut prefix_manager = PrefixManager::new(
                    env::var("TTS_PREFIX").expect("missing TTS_PREFIX").as_str(),
                );
                if prefix_manager.can_load().await {
                    prefix_manager.load().await?;
                }

                Ok(Data {
                    voice_manager: Arc::new(Mutex::new(voice_manager)),
                    mute_manager: Arc::new(Mutex::new(mute_manager)),
                    prefix_manager: Arc::new(Mutex::new(prefix_manager)),
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

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Message { new_message, .. } => {
            message_event_handler(ctx, new_message, framework, data).await
        }
        serenity::FullEvent::VoiceStateUpdate { old, new } => {
            voice_state_update_event_handler(ctx, old, new, framework, data).await
        }
        _ => Ok(()),
    }
}

async fn message_event_handler(
    ctx: &serenity::Context,
    msg: &serenity::Message,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    // Check if the message is from a bot
    if msg.author.bot {
        return Ok(());
    }

    // Check if the message is in a guild
    let guild_id = match msg.guild_id {
        Some(guild_id) => guild_id,
        None => return Ok(()),
    };

    // Check if the message starts with the prefix
    let prefix_manager = data.prefix_manager.lock().await;
    let prefix = prefix_manager.get(guild_id.get()).to_string();
    drop(prefix_manager);

    if !msg.content.starts_with(&prefix) {
        return Ok(());
    }
    let mut text = msg.content[prefix.len()..].to_string();

    // Check if the user is muted
    let mute_manager = data.mute_manager.lock().await;
    let muted = mute_manager.get(guild_id.get(), msg.author.id.get());
    drop(mute_manager);

    if muted {
        msg.react(ctx, '❌').await?;
        return Ok(());
    }

    // Process the message
    text = utils::replace_links(&text);
    text = utils::replace_discord_emojis(&text);
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

    // Check if the channel is a voice channel
    let channel = match msg.channel_id.to_channel(ctx).await {
        Ok(channel) => channel,
        Err(_) => return Ok(()),
    };

    let guild_channel = match channel.guild() {
        Some(guild_channel) => guild_channel,
        None => return Ok(()),
    };

    if guild_channel.kind != serenity::ChannelType::Voice {
        return Ok(());
    }

    // Check if anyone is in the voice channel
    let members = guild_channel.members(ctx)?;
    if members.len() == 0 {
        return Ok(());
    }

    // Check if we're connected to the voice channel
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => manager.join(guild_id, msg.channel_id).await?,
    };

    let mut handler = handler_lock.lock().await;

    if handler.current_channel().map(|c| c.0.get()).unwrap_or(0) != msg.channel_id.get() {
        handler.join(msg.channel_id).await?;
    }

    // Check if we're deafened
    if !handler.is_deaf() {
        handler.deafen(true).await?;
    }

    // Generate the TTS
    let voice_manager = data.voice_manager.lock().await;
    let voice = voice_manager.get(msg.author.id.get()).clone();
    drop(voice_manager);

    let tts_path = tts(&text, &voice).await?;
    let mut tts_bytes = fs::read(&tts_path).await?;
    fs::remove_file(&tts_path).await?;

    // Normalize the TTS
    tts_bytes = utils::normalize_wav(&tts_bytes, framework.user_data.tts_peak)?;

    // Play the TTS
    handler.play_input(Input::from(tts_bytes));
    Ok(())
}

async fn voice_state_update_event_handler(
    ctx: &serenity::Context,
    old: &Option<serenity::VoiceState>,
    _new: &serenity::VoiceState,
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
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => return Ok(()),
    };

    let mut handler = handler_lock.lock().await;

    let channel_id = match handler.current_channel() {
        Some(channel) => serenity::ChannelId::new(channel.0.get()),
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
