#![allow(dead_code)]
#![recursion_limit="1024"]
extern crate libc;
extern crate chrono;
#[macro_use]
extern crate delegate;

mod c_w_string;
mod ffi;
mod streams;
mod convenience_api;

pub type MediaInfoWrapper = convenience_api::MediaInfoWrapper;
pub type MediaInfo = ffi::MediaInfo;
pub type MediaInfoResult<T> = ffi::MediaInfoResult<T>;
pub type MediaInfoError = ffi::MediaInfoError;


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use ffi::MediaInfoStream;

    #[test]
    fn can_retrieve_stream_count() {
        let sample_path = PathBuf::from("samples");
        let filename = sample_path.join("sample.mp4");
        let mut m = MediaInfo::new();

        assert_eq!(true, m.open(filename.as_path()).is_ok());
        assert_eq!(m.count_get(MediaInfoStream::Video), 1);
        assert_eq!(m.count_get(MediaInfoStream::Audio), 1);
        assert_eq!(m.count_get(MediaInfoStream::General), 1);

        m.close();
    }
}
