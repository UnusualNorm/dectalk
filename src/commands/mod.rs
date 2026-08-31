use poise::Command;

use crate::{Data, Error};

mod inverted;
mod muted;
mod prefix;
mod preset;
mod random;
mod reset;
mod test;
mod voice;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![
        inverted::inverted(),
        muted::muted(),
        prefix::prefix(),
        preset::preset(),
        random::random(),
        reset::reset(),
        test::test(),
        voice::voice(),
    ]
}
