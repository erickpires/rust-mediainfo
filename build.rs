extern crate pkg_config;

fn main() {
    let lib_mediainfo = pkg_config::probe_library("libmediainfo");
    if lib_mediainfo.is_err() {
        panic!("Could not find MediaInfo via pkgconfig");
    } else {
        // panic!("{:?}", lib_mediainfo);
    }
}
