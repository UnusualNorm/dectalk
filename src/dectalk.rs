use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{fs, process::Command};
use uuid::Uuid;

// https://github.com/dectalk/dectalk/blob/develop/src/Txt16bit/apndx_d.txt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DECtalkVoice {
    pub sx: u8,  // 0    1    -- Sex 1 (male) or 0 (female)
    pub hs: u8,  // 65   145  %  Head size
    pub f4: u16, // 2000 4650 Hz Fourth formant frequency
    pub f5: u16, // 2500 4950 Hz Fifth formant frequency
    pub b4: u16, // 100  2048 Hz Fourth formant bandwidth
    pub b5: u16, // 100  2048 Hz Fifth formant bandwidth
    pub br: u8,  // 0    72   dB Breathiness
    pub lx: u8,  // 0    100  %  Lax breathiness
    pub sm: u8,  // 0    100  %  Smoothness (high frequency attenuation)
    pub ri: u8,  // 0    100  %  Richness
    pub nf: u8,  // 0    100  -- Number of fixed samplings of glottal pulse open phase
    pub la: u8,  // 0    100  %  Laryngealization
    pub bf: u8,  // 0    40   Hz Baseline fall
    pub hr: u8,  // 2    100  Hz Hat rise
    pub sr: u8,  // 1    100  Hz Stress rise
    pub as_: u8, // 0    100  %  Assertiveness
    pub qu: u8,  // 0    100  %  Quickness
    pub ap: u16, // 50   350  Hz Average pitch
    pub pr: u8,  // 0    250  %  Pitch range
    pub gv: u8,  // 0    86   dB Gain of voicing source
    pub gh: u8,  // 0    86   dB Gain of aspiration source
    pub gf: u8,  // 0    86   dB Gain of frication source
    pub gn: u8,  // 0    86   dB Gain of nasalization
    pub g1: u8,  // 0    86   dB Gain of first formant resonator
    pub g2: u8,  // 0    86   dB Gain of second formant resonator
    pub g3: u8,  // 0    86   dB Gain of third formant resonator
    pub g4: u8,  // 0    86   dB Gain of fourth formant resonator
    pub g5: u8,  // 0    86   dB Gain of fifth formant resonator (replaces lo)
}

pub const PAUL_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 1,
    hs: 100,
    f4: 3300,
    f5: 3650,
    b4: 260,
    b5: 330,
    br: 0,
    lx: 0,
    sm: 3,
    ri: 70,
    nf: 0,
    la: 0,
    bf: 18,
    hr: 18,
    sr: 32,
    as_: 100,
    qu: 40,
    ap: 122,
    pr: 100,
    gv: 65,
    gh: 70,
    gn: 74,
    gf: 70,
    g1: 68,
    g2: 60,
    g3: 48,
    g4: 64,
    g5: 86,
};

pub const HARRY_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 1,
    hs: 115,
    f4: 3300,
    f5: 3850,
    b4: 200,
    b5: 240,
    br: 0,
    lx: 0,
    sm: 12,
    ri: 86,
    nf: 10,
    la: 0,
    bf: 9,
    hr: 20,
    sr: 30,
    as_: 100,
    qu: 10,
    ap: 89,
    pr: 80,
    gv: 65,
    gh: 70,
    gn: 73,
    gf: 70,
    g1: 71,
    g2: 60,
    g3: 52,
    g4: 64,
    g5: 81,
};

pub const FRANK_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 1,
    hs: 90,
    f4: 3650,
    f5: 4200,
    b4: 280,
    b5: 300,
    br: 50,
    lx: 50,
    sm: 46,
    ri: 40,
    nf: 0,
    la: 5,
    bf: 9,
    hr: 20,
    sr: 22,
    as_: 65,
    qu: 0,
    ap: 155,
    pr: 90,
    gv: 63,
    gh: 68,
    gn: 75,
    gf: 68,
    g1: 63,
    g2: 58,
    g3: 56,
    g4: 66,
    g5: 86,
};

pub const DENNIS_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 1,
    hs: 105,
    f4: 3200,
    f5: 3600,
    b4: 240,
    b5: 280,
    br: 38,
    lx: 70,
    sm: 100,
    ri: 0,
    nf: 10,
    la: 0,
    bf: 9,
    hr: 20,
    sr: 22,
    as_: 100,
    qu: 50,
    ap: 110,
    pr: 135,
    gv: 63,
    gh: 68,
    gn: 76,
    gf: 68,
    g1: 75,
    g2: 60,
    g3: 52,
    g4: 61,
    g5: 84,
};

pub const BETTY_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 0,
    hs: 100,
    f4: 4450,
    f5: 2500,
    b4: 260,
    b5: 2048,
    br: 0,
    lx: 80,
    sm: 4,
    ri: 40,
    nf: 0,
    la: 0,
    bf: 0,
    hr: 14,
    sr: 20,
    as_: 35,
    qu: 55,
    ap: 208,
    pr: 140,
    gv: 65,
    gh: 70,
    gn: 72,
    gf: 72,
    g1: 69,
    g2: 65,
    g3: 50,
    g4: 56,
    g5: 81,
};

pub const URSULA_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 0,
    hs: 95,
    f4: 4500,
    f5: 2500,
    b4: 230,
    b5: 2048,
    br: 0,
    lx: 50,
    sm: 60,
    ri: 100,
    nf: 10,
    la: 0,
    bf: 8,
    hr: 20,
    sr: 32,
    as_: 100,
    qu: 30,
    ap: 240,
    pr: 135,
    gv: 65,
    gh: 70,
    gn: 74,
    gf: 70,
    g1: 67,
    g2: 65,
    g3: 51,
    g4: 58,
    g5: 80,
};

pub const WENDY_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 0,
    hs: 100,
    f4: 4500,
    f5: 2500,
    b4: 400,
    b5: 2048,
    br: 55,
    lx: 80,
    sm: 100,
    ri: 0,
    nf: 10,
    la: 0,
    bf: 0,
    hr: 20,
    sr: 22,
    as_: 50,
    qu: 10,
    ap: 200,
    pr: 175,
    gv: 51,
    gh: 68,
    gn: 75,
    gf: 70,
    g1: 69,
    g2: 62,
    g3: 53,
    g4: 55,
    g5: 83,
};

pub const RITA_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 0,
    hs: 95,
    f4: 4000,
    f5: 2500,
    b4: 250,
    b5: 2048,
    br: 46,
    lx: 0,
    sm: 24,
    ri: 20,
    nf: 0,
    la: 4,
    bf: 0,
    hr: 20,
    sr: 32,
    as_: 65,
    qu: 30,
    ap: 106,
    pr: 80,
    gv: 65,
    gh: 70,
    gn: 73,
    gf: 72,
    g1: 69,
    g2: 72,
    g3: 48,
    g4: 54,
    g5: 83,
};

pub const KIT_VOICE: DECtalkVoice = DECtalkVoice {
    sx: 0,
    hs: 80,
    f4: 2500,
    f5: 2500,
    b4: 2048,
    b5: 2048,
    br: 47,
    lx: 75,
    sm: 5,
    ri: 40,
    nf: 0,
    la: 0,
    bf: 0,
    hr: 20,
    sr: 22,
    as_: 65,
    qu: 50,
    ap: 306,
    pr: 210,
    gv: 65,
    gh: 70,
    gn: 71,
    gf: 72,
    g1: 69,
    g2: 69,
    g3: 52,
    g4: 50,
    g5: 73,
};

fn range_validate<Num: PartialOrd>(value: Num, min: Num, max: Num) -> bool {
    value >= min && value <= max
}

impl DECtalkVoice {
    pub fn validate(&self) -> bool {
        range_validate(self.sx, 0, 1)
            && range_validate(self.hs, 65, 145)
            && range_validate(self.f4, 2000, 4650)
            && range_validate(self.f5, 2500, 4950)
            && range_validate(self.b4, 100, 2048)
            && range_validate(self.b5, 100, 2048)
            && range_validate(self.br, 0, 72)
            && range_validate(self.lx, 0, 100)
            && range_validate(self.sm, 0, 100)
            && range_validate(self.ri, 0, 100)
            && range_validate(self.nf, 0, 100)
            && range_validate(self.la, 0, 100)
            && range_validate(self.bf, 0, 40)
            && range_validate(self.hr, 2, 100)
            && range_validate(self.sr, 1, 100)
            && range_validate(self.as_, 0, 100)
            && range_validate(self.qu, 0, 100)
            && range_validate(self.ap, 50, 350)
            && range_validate(self.pr, 0, 250)
            && range_validate(self.gv, 0, 86)
            && range_validate(self.gh, 0, 86)
            && range_validate(self.gf, 0, 86)
            && range_validate(self.gn, 0, 86)
            && range_validate(self.g1, 0, 86)
            && range_validate(self.g2, 0, 86)
            && range_validate(self.g3, 0, 86)
            && range_validate(self.g4, 0, 86)
            && range_validate(self.g5, 0, 86)
    }
}

impl Default for DECtalkVoice {
    fn default() -> Self {
        PAUL_VOICE
    }
}

#[derive(Error, Debug)]
pub enum DECtalkError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Say error: {0}")]
    SayError(String),
}

pub async fn tts(text: &str, voice: &DECtalkVoice) -> Result<String, DECtalkError> {
    let filename = format!("dectalk/{}.wav", Uuid::new_v4());

    let mut cmd = Command::new("dectalk/say");
    cmd.arg("-fo").arg(&filename);
    cmd.arg("-pre").arg(format!(
        "[:phoneme on][:nv]
        [:dv sx {}][:dv hs {}][:dv f4 {}][:dv f5 {}][:dv b4 {}][:dv b5 {}]
        [:dv br {}][:dv lx {}][:dv sm {}][:dv ri {}][:dv nf {}][:dv la {}]
        [:dv bf {}][:dv hr {}][:dv sr {}][:dv as {}][:dv qu {}][:dv ap {}][:dv pr {}]
        [:dv gv {}][:dv gh {}][:dv gf {}][:dv gn {}][:dv g1 {}][:dv g2 {}][:dv g3 {}][:dv g4 {}][:dv g5 {}]",
        voice.sx,
        voice.hs,
        voice.f4,
        voice.f5,
        voice.b4,
        voice.b5,
        voice.br,
        voice.lx,
        voice.sm,
        voice.ri,
        voice.nf,
        voice.la,
        voice.bf,
        voice.hr,
        voice.sr,
        voice.as_,
        voice.qu,
        voice.ap,
        voice.pr,
        voice.gv,
        voice.gh,
        voice.gf,
        voice.gn,
        voice.g1,
        voice.g2,
        voice.g3,
        voice.g4,
        voice.g5
    ));
    cmd.arg("-a").arg(text);

    let output = cmd.output().await?;
    if !output.status.success() {
        if fs::metadata(&filename).await.is_ok() {
            fs::remove_file(&filename).await?;
        }

        return Err(DECtalkError::SayError(
            String::from_utf8(output.stderr).expect("stderr is not valid UTF-8"),
        ));
    }

    Ok(filename)
}
