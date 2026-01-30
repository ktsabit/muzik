use lofty::file::TaggedFileExt;
use lofty::prelude::AudioFile;
use lofty::probe::Probe;
use lofty::tag::Accessor;
use std::path::Path;

pub struct AudioMetadata {
    pub filename: String,
    pub duration: std::time::Duration,
    pub bitrate: u32,
    pub sample_rate: u32,
    pub channels: Option<u8>,
    pub bit_depth: Option<u8>,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<u32>,
    pub track: Option<u32>,
    pub track_total: Option<u32>,
    pub has_album_art: bool,
    pub album_art_count: usize,
}

pub fn get_audio_info(path: &Path) -> Result<AudioMetadata, Box<dyn std::error::Error>> {
    let tagged_file = Probe::open(path)?.read()?;
    let properties = tagged_file.properties();

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut metadata = AudioMetadata {
        filename,
        duration: properties.duration(),
        bitrate: properties.audio_bitrate().unwrap_or(0),
        sample_rate: properties.sample_rate().unwrap_or(0),
        channels: properties.channels(),
        bit_depth: properties.bit_depth(),
        title: "Unknown Title".to_string(),
        artist: "Unknown Artist".to_string(),
        album: "Unknown Album".to_string(),
        year: None,
        track: None,
        track_total: None,
        has_album_art: false,
        album_art_count: 0,
    };

    if let Some(tag) = tagged_file.primary_tag() {
        metadata.title = tag.title().map(|s| s.to_string()).unwrap_or(metadata.title);
        metadata.artist = tag.artist().map(|s| s.to_string()).unwrap_or(metadata.artist);
        metadata.album = tag.album().map(|s| s.to_string()).unwrap_or(metadata.album);
        metadata.year = tag.year();
        metadata.track = tag.track();
        metadata.track_total = tag.track_total();

        let pics = tag.pictures();
        metadata.album_art_count = pics.len();
        metadata.has_album_art = !pics.is_empty();
    }

    Ok(metadata)
}

pub fn print_audio_info(metadata: &AudioMetadata) {
    println!("--- {} ---", metadata.filename);
    println!("Duration:    {:?}", metadata.duration);
    println!("Bitrate:     {} kbps", metadata.bitrate);
    println!("Sample Rate: {} Hz", metadata.sample_rate);
    if let Some(channels) = metadata.channels {
        println!("Channels:    {}", channels);
    }
    if let Some(depth) = metadata.bit_depth {
        println!("Bit Depth:   {} bit", depth);
    }
    println!("Title:       {}", metadata.title);
    println!("Artist:      {}", metadata.artist);
    println!("Album:       {}", metadata.album);
    if let Some(year) = metadata.year {
        println!("Year:        {}", year);
    }
    println!(
        "Track:       {}/{}",
        metadata.track.unwrap_or(0),
        metadata.track_total.unwrap_or(0)
    );
    if metadata.has_album_art {
        println!("Album Art:   Found ({} image(s))", metadata.album_art_count);
    }
    println!();
}