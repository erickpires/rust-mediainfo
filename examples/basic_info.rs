extern crate mediainfo;

use mediainfo::MediaInfo;

fn main() {
    let mut media_info = MediaInfo::new();

    let tracks = ["/home/erick/japan.mp3",
                  "/home/erick/japan.m4a",
                  "/home/erick/japan.flac",
                  "/home/erick/guitar.mp3"];

    for filename in tracks.iter() {
        media_info.open(filename);
        println!("Filename: {}", filename);

        println!("Title: {}", media_info.get_title().unwrap_or("".to_string()));
        println!("Artist: {}", media_info.get_performer().unwrap_or("".to_string()));
        println!("Album: {}", media_info.get_album().unwrap_or("".to_string()));
        println!("Duration: {:?}", media_info.get_duration_ms().unwrap_or(0));
        println!("Track: {}", media_info.get_track_name().unwrap_or("".to_string()));
        println!("Track #: {:?}\n", media_info.get_track_number().unwrap_or(0));

        media_info.close();
    }
}
