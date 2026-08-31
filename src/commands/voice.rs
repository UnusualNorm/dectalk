use crate::{Error, commands::Context, dectalk::Voice};

#[poise::command(slash_command)]
pub async fn voice(
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

    let voice = ctx.data().get_voice(author.id).await;
    let voice = Voice {
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

    ctx.say(format!("```rust\n{:?}\n```", voice)).await?;
    ctx.data().set_voice(author.id, voice).await?;
    Ok(())
}
