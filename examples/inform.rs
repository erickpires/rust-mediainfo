extern crate mediainfo;

use mediainfo::ffi::MediaInfo;

fn main() {
    let mut media_info = MediaInfo::new();

    let tracks = ["/home/erick/japan.mp3",
                  "/home/erick/japan.m4a",
                  "/home/erick/japan.flac",
                  "/home/erick/guitar.mp3"];

    for filename in tracks.iter() {
        media_info.open(filename);
        println!("Filename: {}", filename);
        println!("{}\n", media_info.inform(0));

        media_info.close();
    }
}
