extern crate mediainfo;

use mediainfo::MediaInfoWrapper;
use std::path::PathBuf;

fn main() {
    let mut media_info = MediaInfoWrapper::new();
    let sample_path = PathBuf::from("../samples");
    let extnames = ["mp3", "m4a", "flac"];

    for ext in extnames.iter() {
        let filename = sample_path.join(format!("sample.{}", ext));

        media_info.open(&filename).expect("It should open the file.");
        println!("Filename: {}", filename.to_str().as_ref().unwrap());
        println!("{}\n", media_info.codec().unwrap());

        media_info.close();
    }
}
