use serde::{Deserialize, Serialize};

use super::PAUL_VOICE;

// https://github.com/dectalk/dectalk/blob/develop/src/Txt16bit/apndx_d.txt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
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

pub const MIN_VOICE: Voice = Voice {
    sx: 0,
    hs: 65,
    f4: 2000,
    f5: 2500,
    b4: 100,
    b5: 100,
    br: 0,
    lx: 0,
    sm: 0,
    ri: 0,
    nf: 0,
    la: 0,
    bf: 0,
    hr: 2,
    sr: 1,
    as_: 0,
    qu: 0,
    ap: 50,
    pr: 0,
    gv: 0,
    gh: 0,
    gf: 0,
    gn: 0,
    g1: 0,
    g2: 0,
    g3: 0,
    g4: 0,
    g5: 0,
};

pub const MAX_VOICE: Voice = Voice {
    sx: 1,
    hs: 145,
    f4: 4650,
    f5: 4950,
    b4: 2048,
    b5: 2048,
    br: 72,
    lx: 100,
    sm: 100,
    ri: 100,
    nf: 100,
    la: 100,
    bf: 40,
    hr: 100,
    sr: 100,
    as_: 100,
    qu: 100,
    ap: 350,
    pr: 250,
    gv: 86,
    gh: 86,
    gf: 86,
    gn: 86,
    g1: 86,
    g2: 86,
    g3: 86,
    g4: 86,
    g5: 86,
};

fn range_validate<Num: PartialOrd>(value: Num, min: Num, max: Num) -> bool {
    value >= min && value <= max
}

impl Voice {
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

impl Default for Voice {
    fn default() -> Self {
        PAUL_VOICE
    }
}
