![](https://docs.rs/lyrics_ovh/badge.svg)

# Lyrics_ovh

This is a library that allows you to fetch lyrics to a song of your choosing.

This crate uses a [lyrics.ovh](https://lyrics.ovh) API to fetch lyrics.

The library only contains one async method.

## Example

```Rust
use lyrics_ovh::get_lyrics;

#[tokio::main]
pub async fn main(){
    let lyrics = get_lyrics("Popular Monster", "Falling in reverse").await;
    println!("Lyrics to the song Popular monster by Falling in reverse: \n {}", lyrics);
}
```
