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

        println!("Title: {}", media_info.get_title().unwrap());
        println!("Artist: {}", media_info.get_performer().unwrap());
        println!("Album: {}", media_info.get_album().unwrap());
        println!("Duration: {:?}", media_info.get_duration_ms());
        println!("Track: {}", media_info.get_track_name().unwrap());
        println!("Track #: {:?}\n", media_info.get_track_number());

        media_info.close();
    }
}
