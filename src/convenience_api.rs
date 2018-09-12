use ffi::{MediaInfo, MediaInfoResult, MediaInfoStream};
use streams::{GeneralStream, VideoStream, AudioStream, ImageStream, MenuStream, OtherStream, TextStream};

use chrono::{UTC, DateTime};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::time::Duration;

pub struct MediaInfoWrapper {
    general_stream: GeneralStream,
    handle: Rc<RefCell<MediaInfo>>,
    video_streams: Option<Vec<VideoStream>>,
    audio_streams: Option<Vec<AudioStream>>,
    image_streams: Option<Vec<ImageStream>>,
    menu_streams: Option<Vec<MenuStream>>,
    text_streams: Option<Vec<TextStream>>,
    other_streams: Option<Vec<OtherStream>>,
}

impl Default for MediaInfoWrapper {
    fn default() -> Self {
        MediaInfoWrapper {
            general_stream: GeneralStream {
                stream_type: MediaInfoStream::General,
                handler: None,
            },
            video_streams: None,
            audio_streams: None,
            image_streams: None,
            menu_streams: None,
            text_streams: None,
            other_streams: None,
            handle: Rc::new(RefCell::new(MediaInfo::new())),
        }
    }
}

impl MediaInfoWrapper {
    pub fn new() -> MediaInfoWrapper {
        Default::default()
    }

    pub fn open(&mut self, path: &Path) -> MediaInfoResult<usize> {
        let result = self.handle.borrow_mut().open(path);

        match result {
            Ok(r) => {
                self.wrap_streams();
                Ok(r)
            },
            Err(r) => Err(r),
        }
    }

    pub fn open_data(&mut self, data: &[u8]) -> Result<(), String>{
        let data_len = data.len();
        if data_len == 0 { return Err("Data length is 0".to_string()); }

        self.handle.borrow_mut().open_buffer_init(data_len as u64, 0);
        let continue_result = self.handle.borrow_mut().open_buffer_continue(data);
        let finalize_result = self.handle.borrow_mut().open_buffer_finalize();

        if continue_result & 0x01 == 0 || finalize_result == 0 {
            return Err("Could not read buffer".to_string());
        }

        self.wrap_streams();

        Ok(())
    }

    pub fn close(&mut self) {
        self.general_stream.handler = None;
        self.video_streams = None;
        self.audio_streams = None;
        self.image_streams = None;
        self.menu_streams = None;
        self.text_streams = None;
        self.other_streams = None;
        self.handle.borrow_mut().close();
    }

    fn wrap_streams(&mut self) {
        self.general_stream.handler = Some(Rc::clone(&self.handle));

        for stype in MediaInfoStream::variants() {
            match stype {
                MediaInfoStream::Video => {
                    let mut streams = Vec::new();
                    for i in 0..self.handle.borrow_mut().count_get(stype) {
                        streams.push(VideoStream {
                            stream_type: stype,
                            index: i,
                            handler: Rc::clone(&self.handle),
                        });
                    };
                    self.video_streams = Some(streams);
                },
                MediaInfoStream::Audio => {
                    let mut streams = Vec::new();
                    for i in 0..self.handle.borrow_mut().count_get(stype) {
                        streams.push(AudioStream {
                            stream_type: stype,
                            index: i,
                            handler: Rc::clone(&self.handle),
                        });
                    };
                    self.audio_streams = Some(streams);
                },
                MediaInfoStream::Text => {
                    let mut streams = Vec::new();
                    for i in 0..self.handle.borrow_mut().count_get(stype) {
                        streams.push(TextStream {
                            stream_type: stype,
                            index: i,
                            handler: Rc::clone(&self.handle),
                        });
                    };
                    self.text_streams = Some(streams);
                },
                MediaInfoStream::Other => {
                    let mut streams = Vec::new();
                    for i in 0..self.handle.borrow_mut().count_get(stype) {
                        streams.push(OtherStream {
                            stream_type: stype,
                            index: i,
                            handler: Rc::clone(&self.handle),
                        });
                    };
                    self.other_streams = Some(streams);
                },
                MediaInfoStream::Image => {
                    let mut streams = Vec::new();
                    for i in 0..self.handle.borrow_mut().count_get(stype) {
                        streams.push(ImageStream {
                            stream_type: stype,
                            index: i,
                            handler: Rc::clone(&self.handle),
                        });
                    };
                    self.image_streams = Some(streams);
                },
                MediaInfoStream::Menu => {
                    let mut streams = Vec::new();
                    for i in 0..self.handle.borrow_mut().count_get(stype) {
                        streams.push(MenuStream {
                            stream_type: stype,
                            index: i,
                            handler: Rc::clone(&self.handle),
                        });
                    };
                    self.menu_streams = Some(streams);
                },
                _ => continue,
            }
        }
    }

    pub fn video_streams(&self) -> Option<&Vec<VideoStream>> {
        self.video_streams.as_ref()
    }

    pub fn audio_streams(&self) -> Option<&Vec<AudioStream>> {
        self.audio_streams.as_ref()
    }

    pub fn text_streams(&self) -> Option<&Vec<TextStream>> {
        self.text_streams.as_ref()
    }

    pub fn image_streams(&self) -> Option<&Vec<ImageStream>> {
        self.image_streams.as_ref()
    }

    pub fn other_streams(&self) -> Option<&Vec<OtherStream>> {
        self.other_streams.as_ref()
    }

    pub fn menu_streams(&self) -> Option<&Vec<MenuStream>> {
        self.menu_streams.as_ref()
    }

    delegate! {
        target self.general_stream {
            pub fn codec_id(&self) -> MediaInfoResult<String>;
            pub fn duration(&self) -> MediaInfoResult<Duration>;
            pub fn format(&self) -> MediaInfoResult<String>;
            pub fn format_profile(&self) -> MediaInfoResult<String>;
            pub fn format_info(&self) -> MediaInfoResult<String>;
            pub fn codec(&self) -> MediaInfoResult<String>;
            pub fn overall_bit_rate(&self) -> MediaInfoResult<i64>;
            pub fn writing_application(&self) -> MediaInfoResult<String>;
            pub fn headersize(&self) -> MediaInfoResult<i64>;
            pub fn datasize(&self) -> MediaInfoResult<i64>;
            pub fn footersize(&self) -> MediaInfoResult<i64>;
            pub fn encoded_library(&self) -> MediaInfoResult<String>;
            pub fn mastered_date(&self) -> MediaInfoResult<DateTime<UTC>>;
            pub fn tagged_date(&self) -> MediaInfoResult<DateTime<UTC>>;
            pub fn encoded_date(&self) -> MediaInfoResult<DateTime<UTC>>;
            pub fn last_modification_date(&self) -> MediaInfoResult<DateTime<UTC>>;
            pub fn artist(&self) -> MediaInfoResult<String>;
            pub fn performer(&self) -> MediaInfoResult<String>;
            pub fn title(&self) -> MediaInfoResult<String>;
            pub fn genre(&self) -> MediaInfoResult<String>;
            pub fn album(&self) -> MediaInfoResult<String>;
            pub fn copyright(&self) -> MediaInfoResult<String>;
            pub fn year(&self) -> MediaInfoResult<String>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use chrono::NaiveDate;
    use std::fs;

    #[test]
    fn can_retrieve_general_information() {
        let sample_path = PathBuf::from("samples");
        let filename = sample_path.join("sample.mp4");
        let mut mw = MediaInfoWrapper::new();
        mw.open(filename.as_path()).unwrap();

        assert_eq!("mp42", mw.codec_id().unwrap());
        println!("CODECID");
        assert_eq!(Duration::from_millis(5568), mw.duration().unwrap());
        assert_eq!("MPEG-4", mw.format().unwrap());
        assert_eq!("Base Media / Version 2", mw.format_profile().unwrap());
        assert_eq!("MPEG-4", mw.codec().unwrap());
        assert_eq!(551194, mw.overall_bit_rate().unwrap());
        assert_eq!("HandBrake 0.9.4 2009112300", mw.writing_application().unwrap());
        assert_eq!(160, mw.headersize().unwrap());
        assert_eq!(379880, mw.datasize().unwrap());
        assert_eq!(3591, mw.footersize().unwrap());
        assert_eq!(DateTime::<UTC>::from_utc(NaiveDate::from_ymd(2010, 3, 20).and_hms(21, 29, 12), UTC), mw.tagged_date().unwrap());
        mw.close();
    }

    #[test]
    fn can_retrieve_information_from_buffer() {
        let sample_path = PathBuf::from("samples");
        let filename = sample_path.join("sample.mp4");
        let mut mw = MediaInfoWrapper::new();
        let contents = fs::read(filename).expect("File not found.");
        mw.open_data(contents.as_slice()).expect("Could not read from buffer.");

        assert_eq!("mp42", mw.codec_id().unwrap());
    }

    #[test]
    fn can_retrieve_video_stream_information() {
        let sample_path = PathBuf::from("samples");
        let filename = sample_path.join("sample.mp4");
        let mut mw = MediaInfoWrapper::new();
        mw.open(filename.as_path()).unwrap();

        let vstreams = match mw.video_streams() {
            Some(x) => x,
            None => panic!("It should have video streams."),
        };

        assert_eq!(vstreams.len(), 1);

        let vstream = match vstreams.first() {
            Some(x) => x,
            None => panic!("It should have video streams."),
        };

        assert_eq!("AVC", vstream.format().unwrap());
    }
}
