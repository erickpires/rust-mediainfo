extern crate mediainfo;

use mediainfo::MediaInfo;

fn main() {
    let mut media_info = MediaInfo::new();
    println!("{}", media_info.available_parameters());
}
