use crate::dectalk::*;

// SLOP!!!
pub fn random(seed: u64) -> Voice {
    let mut rng = SplitMix64::new(seed);
    const MALE: [&Voice; 4] = [&PAUL_VOICE, &HARRY_VOICE, &FRANK_VOICE, &DENNIS_VOICE];
    const FEMALE: [&Voice; 4] = [&BETTY_VOICE, &URSULA_VOICE, &WENDY_VOICE, &RITA_VOICE];
    let presets = if rng.bool() { &MALE } else { &FEMALE };

    let a_index = rng.index(presets.len());
    let mut b_index = rng.index(presets.len());
    if b_index == a_index {
        b_index = (b_index + 1) % presets.len();
    }

    let a = presets[a_index];
    let b = presets[b_index];
    let t = rng.range_f32(0.15, 0.85);
    let mut v = Voice {
        sx: a.sx,
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

    v.hs = jitter_u8(&mut rng, v.hs, 4, 80, 120);
    if v.sx == 1 {
        v.f4 = jitter_u16(&mut rng, v.f4, 100, 3000, 3850);
        v.f5 = jitter_u16(&mut rng, v.f5, 100, 3450, 4350);
        v.ap = jitter_u16(&mut rng, v.ap, 8, 80, 170);
        v.pr = jitter_u8(&mut rng, v.pr, 8, 70, 150);
    } else {
        v.f4 = jitter_u16(&mut rng, v.f4, 100, 3850, 4600);
        v.f5 = 2500;
        v.ap = jitter_u16(&mut rng, v.ap, 10, 100, 260);
        v.pr = jitter_u8(&mut rng, v.pr, 10, 70, 190);
    }
    if v.sx == 1 {
        v.b4 = jitter_u16(&mut rng, v.b4, 30, 170, 330);
        v.b5 = jitter_u16(&mut rng, v.b5, 30, 200, 360);
    } else {
        v.b4 = jitter_u16(&mut rng, v.b4, 35, 190, 450);
        v.b5 = 2048;
    }
    v.br = jitter_u8(&mut rng, v.br, 6, 0, 60);
    v.lx = jitter_u8(&mut rng, v.lx, 8, 0, 90);
    v.sm = jitter_u8(&mut rng, v.sm, 8, 0, 100);
    v.ri = jitter_u8(&mut rng, v.ri, 8, 0, 100);
    v.nf = if rng.bool_probability(0.30) { 10 } else { 0 };
    v.la = jitter_u8(&mut rng, v.la, 2, 0, 8);
    v.bf = jitter_u8(&mut rng, v.bf, 3, 0, 20);
    v.hr = jitter_u8(&mut rng, v.hr, 3, 10, 30);
    v.sr = jitter_u8(&mut rng, v.sr, 4, 15, 40);
    v.as_ = jitter_u8(&mut rng, v.as_, 7, 25, 100);
    v.qu = jitter_u8(&mut rng, v.qu, 6, 0, 65);
    v.gv = jitter_u8(&mut rng, v.gv, 2, 58, 68);
    v.gh = jitter_u8(&mut rng, v.gh, 2, 65, 73);
    v.gf = jitter_u8(&mut rng, v.gf, 2, 65, 74);
    v.gn = jitter_u8(&mut rng, v.gn, 2, 69, 78);
    v.g1 = jitter_u8(&mut rng, v.g1, 2, 60, 77);
    v.g2 = jitter_u8(&mut rng, v.g2, 2, 56, 74);
    v.g3 = jitter_u8(&mut rng, v.g3, 2, 45, 58);
    v.g4 = jitter_u8(&mut rng, v.g4, 2, 50, 68);
    v.g5 = jitter_u8(&mut rng, v.g5, 2, 72, 86);
    v
}

#[inline]
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

#[inline]
fn lerp_u16(a: u16, b: u16, t: f32) -> u16 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u16
}

fn jitter_u8(rng: &mut SplitMix64, value: u8, amount: i32, min: u8, max: u8) -> u8 {
    let delta = rng.range_i32(-amount, amount);
    (value as i32 + delta).clamp(min as i32, max as i32) as u8
}

fn jitter_u16(rng: &mut SplitMix64, value: u16, amount: i32, min: u16, max: u16) -> u16 {
    let delta = rng.range_i32(-amount, amount);
    (value as i32 + delta).clamp(min as i32, max as i32) as u16
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);

        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    fn f32(&mut self) -> f32 {
        ((self.next_u64() >> 40) as u32) as f32 / 16_777_216.0
    }

    fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + self.f32() * (max - min)
    }

    fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        debug_assert!(min <= max);

        let width = (max - min + 1) as u64;
        min + (self.next_u64() % width) as i32
    }

    fn index(&mut self, len: usize) -> usize {
        (self.next_u64() % len as u64) as usize
    }

    fn bool(&mut self) -> bool {
        self.next_u64() & 1 != 0
    }

    fn bool_probability(&mut self, probability: f32) -> bool {
        self.f32() < probability
    }
}
