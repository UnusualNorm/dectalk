use std::io;

use thiserror::Error;
use tokio::{fs, process::Command};

use super::Voice;

#[derive(Error, Debug)]
pub enum SayError {
    #[error("Invalid ASCII")]
    ASCIIError,
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Say error: {0}")]
    SayError(String),
}

pub async fn say(text: &str, voice: &Voice, id: u64) -> Result<Vec<u8>, SayError> {
    if !text.is_ascii() {
        return Err(SayError::ASCIIError);
    }

    let filename = format!("dectalk/{}.wav", id);
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

        return Err(SayError::SayError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let bytes = fs::read(&filename).await?;
    fs::remove_file(&filename).await?;
    Ok(bytes)
}
