use std::{io::Cursor, sync::LazyLock};

use regex::Regex;

pub fn normalize_wav(input_buf: &[u8], target_peak: f32) -> Result<Vec<u8>, hound::Error> {
    // Open the input WAV buffer
    let mut reader = hound::WavReader::new(Cursor::new(input_buf))?;
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples::<i16>().filter_map(Result::ok).collect();

    // Find the peak amplitude
    let max_sample = samples
        .iter()
        .map(|&s| s.unsigned_abs() as f32) // Use `unsigned_abs()` (Rust 1.67+)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0); // Prevent division by zero

    // Compute the normalization factor
    let norm_factor = (target_peak * i16::MAX as f32) / max_sample;

    // Apply normalization
    let normalized_samples: Vec<i16> = samples
        .iter()
        .map(|&s| {
            let scaled = (s as f32 * norm_factor).round(); // Round to nearest integer
            scaled.clamp(i16::MIN as f32, i16::MAX as f32) as i16
        })
        .collect();

    // Write to the output WAV buffer
    let mut buf = Vec::new();
    let mut writer = hound::WavWriter::new(Cursor::new(&mut buf), spec)?;
    for sample in normalized_samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;

    Ok(buf)
}

static URL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?://[^\s/$.?#].[^\s]*").expect("invalid regex"));
static EMOJI_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<a?:(\w+):\d+>").expect("invalid regex"));

pub fn replace_links(text: &str) -> String {
    URL_REGEX.replace_all(text, "").to_string()
}

pub fn replace_discord_emojis(text: &str) -> String {
    EMOJI_REGEX.replace_all(text, "$1").to_string()
}
