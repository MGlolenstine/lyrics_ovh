use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Lyrics {
    lyrics: String,
}

/// Fetch lyrics for the specified song.
///
/// ## Return
/// Returns lyrics as a string if they're found.
///
/// ## Error
/// Returns the reqwest error if any occur.
///
/// ## Example
///```rust
///#[tokio::main]
///pub async fn main(){
///    let lyrics = get_lyrics("Popular Monster", "Falling in reverse").await;
///    println!("Lyrics to the song Popular monster by Falling in reverse: \n {}", lyrics);
///}
///```
pub async fn get_lyrics(title: &str, author: &str) -> Result<String, Error> {
    let resp = reqwest::get(&format!("https://api.lyrics.ovh/v1/{}/{}", author, title))
        .await?
        .json::<Lyrics>()
        .await?;
    Ok(resp.lyrics)
}

pub async fn fetc_lyrics() {
    // Fetch lyrics like this: https://github.com/AllToMP3/alltomp3/blob/master/index.js#L72
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_lyrics() {
    let lyrics = get_lyrics("popular monster", "Falling in Reverse").await;
    assert!(lyrics.is_ok());
    let lyrics = lyrics.unwrap();
    assert!(lyrics.starts_with("Yeah"));
    assert!(lyrics.contains("And my doctor tries to tell me that I'm going through a phase"));
}
