use serde::Deserialize;

use crate::CLIENT;

#[derive(Deserialize, Debug)]
struct GerLrcResult {
    data: GetLrcData,
}

#[derive(Deserialize, Debug)]
struct GetLrcData {
    lrclist: Vec<GetLrc>,
}

#[derive(Deserialize, Debug)]
struct GetLrc {
    #[serde(rename = "lineLyric")]
    line_lyric: String,
    time: String,
}

fn format_seconds_to_timestamp(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor() as i64;
    let remaining_seconds = seconds % 60.0;
    format!("{:02}:{:05.2}", minutes, remaining_seconds)
}

pub(crate) async fn get_kuwo_lyric(song_id: &str) -> Result<String, anyhow::Error> {
    let result = CLIENT
        .get(format!(
            "https://m.kuwo.cn/newh5/singles/songinfoandlrc?musicId={}",
            song_id.replace("MUSIC_", "")
        ))
        .send()
        .await?
        .json::<GerLrcResult>()
        .await?;

    let lyrics = result
        .data
        .lrclist
        .into_iter()
        .filter_map(|lrc| {
            let time: f64 = match lrc.time.parse() {
                Ok(t) => t,
                Err(_) => return None,
            };
            Some(format!(
                "[{}]{}",
                format_seconds_to_timestamp(time),
                lrc.line_lyric
            ))
        })
        .collect::<Vec<String>>()
        .join("\n");
    Ok(lyrics)
}
