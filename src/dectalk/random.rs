use std::ops::RangeInclusive;

use rand::prelude::*;

use crate::dectalk::*;

use super::{MAX_VOICE, MIN_VOICE};

const LERP_RANGE: RangeInclusive<f32> = 0.15..=0.85;
const LERP_VOICES: &[Voice] = &[
    PAUL_VOICE,
    HARRY_VOICE,
    FRANK_VOICE,
    DENNIS_VOICE,
    BETTY_VOICE,
    URSULA_VOICE,
    WENDY_VOICE,
    RITA_VOICE,
    KIT_VOICE,
];

const JITTER_VOICE: Voice = Voice {
    sx: 1,
    hs: 4,
    f4: 100,
    f5: 100,
    b4: 35,
    b5: 35,
    br: 6,
    lx: 8,
    sm: 8,
    ri: 8,
    nf: 0,
    la: 2,
    bf: 3,
    hr: 3,
    sr: 4,
    as_: 7,
    qu: 6,
    ap: 10,
    pr: 10,
    gv: 2,
    gh: 2,
    gf: 2,
    gn: 2,
    g1: 2,
    g2: 2,
    g3: 2,
    g4: 2,
    g5: 2,
};

#[inline]
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

#[inline]
fn lerp_u16(a: u16, b: u16, t: f32) -> u16 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u16
}

#[inline]
fn jitter_u8(rng: &mut impl Rng, value: u8, amount: i32, min: u8, max: u8) -> u8 {
    (value as i32 + rng.random_range(-amount..=amount)).clamp(min as i32, max as i32) as u8
}

#[inline]
fn jitter_u16(rng: &mut impl Rng, value: u16, amount: i32, min: u16, max: u16) -> u16 {
    (value as i32 + rng.random_range(-amount..=amount)).clamp(min as i32, max as i32) as u16
}

pub fn random(seed: u64) -> Voice {
    let mut rng = SmallRng::seed_from_u64(seed);
    let ai = rng.random_range(0..LERP_VOICES.len());
    let bi = rng.random_range(0..LERP_VOICES.len() - 1);
    let a = &LERP_VOICES[ai];
    let b = &LERP_VOICES[if bi >= ai { bi + 1 } else { bi }];
    let t = rng.random_range(LERP_RANGE);
    let mut voice = Voice {
        sx: lerp_u8(a.sx, b.sx, t),
        hs: lerp_u8(a.hs, b.hs, t),
        f4: lerp_u16(a.f4, b.f4, t),
        f5: lerp_u16(a.f5, b.f5, t),
        b4: lerp_u16(a.b4, b.b4, t),
        b5: lerp_u16(a.b5, b.b5, t),
        br: lerp_u8(a.br, b.br, t),
        lx: lerp_u8(a.lx, b.lx, t),
        sm: lerp_u8(a.sm, b.sm, t),
        ri: lerp_u8(a.ri, b.ri, t),
        nf: lerp_u8(a.nf, b.nf, t),
        la: lerp_u8(a.la, b.la, t),
        bf: lerp_u8(a.bf, b.bf, t),
        hr: lerp_u8(a.hr, b.hr, t),
        sr: lerp_u8(a.sr, b.sr, t),
        as_: lerp_u8(a.as_, b.as_, t),
        qu: lerp_u8(a.qu, b.qu, t),
        ap: lerp_u16(a.ap, b.ap, t),
        pr: lerp_u8(a.pr, b.pr, t),
        gv: lerp_u8(a.gv, b.gv, t),
        gh: lerp_u8(a.gh, b.gh, t),
        gf: lerp_u8(a.gf, b.gf, t),
        gn: lerp_u8(a.gn, b.gn, t),
        g1: lerp_u8(a.g1, b.g1, t),
        g2: lerp_u8(a.g2, b.g2, t),
        g3: lerp_u8(a.g3, b.g3, t),
        g4: lerp_u8(a.g4, b.g4, t),
        g5: lerp_u8(a.g5, b.g5, t),
    };

    voice.sx = jitter_u8(
        &mut rng,
        voice.sx,
        JITTER_VOICE.sx as i32,
        MIN_VOICE.sx,
        MAX_VOICE.sx,
    );
    voice.hs = jitter_u8(
        &mut rng,
        voice.hs,
        JITTER_VOICE.hs as i32,
        MIN_VOICE.hs,
        MAX_VOICE.hs,
    );
    voice.f4 = jitter_u16(
        &mut rng,
        voice.f4,
        JITTER_VOICE.f4 as i32,
        MIN_VOICE.f4,
        MAX_VOICE.f4,
    );
    voice.f5 = jitter_u16(
        &mut rng,
        voice.f5,
        JITTER_VOICE.f5 as i32,
        MIN_VOICE.f5,
        MAX_VOICE.f5,
    );
    voice.b4 = jitter_u16(
        &mut rng,
        voice.b4,
        JITTER_VOICE.b4 as i32,
        MIN_VOICE.b4,
        MAX_VOICE.b4,
    );
    voice.b5 = jitter_u16(
        &mut rng,
        voice.b5,
        JITTER_VOICE.b5 as i32,
        MIN_VOICE.b5,
        MAX_VOICE.b5,
    );
    voice.br = jitter_u8(
        &mut rng,
        voice.br,
        JITTER_VOICE.br as i32,
        MIN_VOICE.br,
        MAX_VOICE.br,
    );
    voice.lx = jitter_u8(
        &mut rng,
        voice.lx,
        JITTER_VOICE.lx as i32,
        MIN_VOICE.lx,
        MAX_VOICE.lx,
    );
    voice.sm = jitter_u8(
        &mut rng,
        voice.sm,
        JITTER_VOICE.sm as i32,
        MIN_VOICE.sm,
        MAX_VOICE.sm,
    );
    voice.ri = jitter_u8(
        &mut rng,
        voice.ri,
        JITTER_VOICE.ri as i32,
        MIN_VOICE.ri,
        MAX_VOICE.ri,
    );
    voice.nf = jitter_u8(
        &mut rng,
        voice.nf,
        JITTER_VOICE.nf as i32,
        MIN_VOICE.nf,
        MAX_VOICE.nf,
    );
    voice.la = jitter_u8(
        &mut rng,
        voice.la,
        JITTER_VOICE.la as i32,
        MIN_VOICE.la,
        MAX_VOICE.la,
    );
    voice.bf = jitter_u8(
        &mut rng,
        voice.bf,
        JITTER_VOICE.bf as i32,
        MIN_VOICE.bf,
        MAX_VOICE.bf,
    );
    voice.hr = jitter_u8(
        &mut rng,
        voice.hr,
        JITTER_VOICE.hr as i32,
        MIN_VOICE.hr,
        MAX_VOICE.hr,
    );
    voice.sr = jitter_u8(
        &mut rng,
        voice.sr,
        JITTER_VOICE.sr as i32,
        MIN_VOICE.sr,
        MAX_VOICE.sr,
    );
    voice.as_ = jitter_u8(
        &mut rng,
        voice.as_,
        JITTER_VOICE.as_ as i32,
        MIN_VOICE.as_,
        MAX_VOICE.as_,
    );
    voice.qu = jitter_u8(
        &mut rng,
        voice.qu,
        JITTER_VOICE.qu as i32,
        MIN_VOICE.qu,
        MAX_VOICE.qu,
    );
    voice.ap = jitter_u16(
        &mut rng,
        voice.ap,
        JITTER_VOICE.ap as i32,
        MIN_VOICE.ap,
        MAX_VOICE.ap,
    );
    voice.pr = jitter_u8(
        &mut rng,
        voice.pr,
        JITTER_VOICE.pr as i32,
        MIN_VOICE.pr,
        MAX_VOICE.pr,
    );
    voice.gv = jitter_u8(
        &mut rng,
        voice.gv,
        JITTER_VOICE.gv as i32,
        MIN_VOICE.gv,
        MAX_VOICE.gv,
    );
    voice.gh = jitter_u8(
        &mut rng,
        voice.gh,
        JITTER_VOICE.gh as i32,
        MIN_VOICE.gh,
        MAX_VOICE.gh,
    );
    voice.gf = jitter_u8(
        &mut rng,
        voice.gf,
        JITTER_VOICE.gf as i32,
        MIN_VOICE.gf,
        MAX_VOICE.gf,
    );
    voice.gn = jitter_u8(
        &mut rng,
        voice.gn,
        JITTER_VOICE.gn as i32,
        MIN_VOICE.gn,
        MAX_VOICE.gn,
    );
    voice.g1 = jitter_u8(
        &mut rng,
        voice.g1,
        JITTER_VOICE.g1 as i32,
        MIN_VOICE.g1,
        MAX_VOICE.g1,
    );
    voice.g2 = jitter_u8(
        &mut rng,
        voice.g2,
        JITTER_VOICE.g2 as i32,
        MIN_VOICE.g2,
        MAX_VOICE.g2,
    );
    voice.g3 = jitter_u8(
        &mut rng,
        voice.g3,
        JITTER_VOICE.g3 as i32,
        MIN_VOICE.g3,
        MAX_VOICE.g3,
    );
    voice.g4 = jitter_u8(
        &mut rng,
        voice.g4,
        JITTER_VOICE.g4 as i32,
        MIN_VOICE.g4,
        MAX_VOICE.g4,
    );
    voice.g5 = jitter_u8(
        &mut rng,
        voice.g5,
        JITTER_VOICE.g5 as i32,
        MIN_VOICE.g5,
        MAX_VOICE.g5,
    );

    voice
}
