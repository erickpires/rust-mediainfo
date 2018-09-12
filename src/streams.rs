use ffi::{MediaInfo, MediaInfoInfo, MediaInfoResult, MediaInfoError, MediaInfoStream};
use chrono::{UTC, DateTime, NaiveDateTime};

use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

macro_rules! stream_struct {
    ($struct_name: ident) => {
        pub struct $struct_name {
            pub stream_type: MediaInfoStream,
            pub index: usize,
            pub handler: Rc<RefCell<MediaInfo>>,
        }
    }
}

macro_rules! base_stream_implement {
    ($struct_name: ident) => {
        impl BaseStream for $struct_name {
            fn stream_type(&self) -> MediaInfoStream {
                self.stream_type
            }

            fn index(&self) -> usize {
                self.index
            }

            fn handler(&self) -> Option<&Rc<RefCell<MediaInfo>>> {
                Some(&self.handler)
            }
        }
    }
}

macro_rules! mediainfo_attr {
    ($meth_name: ident, $attr_name: tt) => (
        pub fn $meth_name(&self) -> MediaInfoResult<String> {
            match self.handler() {
                Some(rc) => rc.borrow_mut().get(self.stream_type(), self.index(), $attr_name, MediaInfoInfo::Text, MediaInfoInfo::Name),
                None => Err(MediaInfoError::NoDataOpenError)
            }
        }
    )
}

macro_rules! mediainfo_date {
    ($meth_name: ident, $attr_name: tt) => (
        pub fn $meth_name(&self) -> MediaInfoResult<DateTime<UTC>> {
            match self.handler() {
                Some(rc) => {
                    self.result_to_date(rc.borrow_mut().get(self.stream_type(), self.index(), $attr_name, MediaInfoInfo::Text, MediaInfoInfo::Name))
                },
                None => Err(MediaInfoError::NoDataOpenError),
            }
        }
    )
}

macro_rules! mediainfo_i64 {
    ($meth_name: ident, $attr_name: tt) => (
        pub fn $meth_name(&self) -> MediaInfoResult<i64> {
            match self.handler() {
                Some(rc) => {
                    self.result_to_i64(rc.borrow_mut().get(self.stream_type(), self.index(), $attr_name, MediaInfoInfo::Text, MediaInfoInfo::Name))
                },
                None => Err(MediaInfoError::NoDataOpenError),
            }
        }
    )
}

macro_rules! mediainfo_duration {
    ($meth_name: ident, $attr_name: tt) => (
        pub fn $meth_name(&self) -> MediaInfoResult<Duration> {
            match self.handler() {
                Some(rc) => {
                    self.result_to_duration(rc.borrow_mut().get(self.stream_type(), self.index(), $attr_name, MediaInfoInfo::Text, MediaInfoInfo::Name))
                },
                None => Err(MediaInfoError::NoDataOpenError),
            }
        }
    )
}

pub struct GeneralStream {
    pub stream_type: MediaInfoStream,
    pub handler: Option<Rc<RefCell<MediaInfo>>>,
}

pub trait BaseStream {
    fn stream_type(&self) -> MediaInfoStream;
    fn index(&self) -> usize;
    fn handler(&self) -> Option<&Rc<RefCell<MediaInfo>>>;

    fn result_to_duration(&self, result: MediaInfoResult<String>) -> MediaInfoResult<Duration> {
        match result?.parse::<u64>() {
            Ok(x) => Ok(Duration::from_millis(x)),
            Err(_) =>Err(MediaInfoError::NonNumericResultError),
        }
    }

    fn result_to_i64(&self, result: MediaInfoResult<String>) -> MediaInfoResult<i64> {
        match result?.parse::<i64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(MediaInfoError::NonNumericResultError),
        }
    }

    fn result_to_date(&self, result: MediaInfoResult<String>) -> MediaInfoResult<DateTime<UTC>> {
        match NaiveDateTime::parse_from_str(&result?, "UTC %Y-%m-%d %H:%M:%S") {
            Ok(x) => Ok(DateTime::<UTC>::from_utc(x, UTC)),
            Err(_) => Err(MediaInfoError::NonNumericResultError),
        }
    }
}

impl BaseStream for GeneralStream {
    fn stream_type(&self) -> MediaInfoStream {
        self.stream_type
    }

    fn index(&self) -> usize {
        0
    }

    fn handler(&self) -> Option<&Rc<RefCell<MediaInfo>>> {
        match self.handler.as_ref() {
            Some(x) => Some(x),
            None => None,
        }
    }
}

stream_struct!(VideoStream);
base_stream_implement!(VideoStream);
stream_struct!(AudioStream);
base_stream_implement!(AudioStream);
stream_struct!(TextStream);
base_stream_implement!(TextStream);
stream_struct!(OtherStream);
base_stream_implement!(OtherStream);
stream_struct!(ImageStream);
base_stream_implement!(ImageStream);
stream_struct!(MenuStream);
base_stream_implement!(MenuStream);

/* GeneralStream */
impl GeneralStream {
    mediainfo_attr!(codec_id, "CodecID");
    mediainfo_attr!(format, "Format");
    mediainfo_attr!(format_profile, "Format_Profile");
    mediainfo_attr!(format_info, "Format_Info");
    mediainfo_attr!(codec, "Codec");
    mediainfo_attr!(encoded_application_string, "Encoded_Application/String");
    mediainfo_attr!(encoded_application, "Encoded_Application");
    mediainfo_attr!(encoded_library, "Encoded_Library");
    mediainfo_attr!(artist, "Artist");
    mediainfo_attr!(performer, "Performer");
    mediainfo_attr!(title, "Title");
    mediainfo_attr!(copyright, "Copyright");
    mediainfo_attr!(genre, "Genre");
    mediainfo_attr!(album, "Album");
    mediainfo_attr!(year, "Year");

    mediainfo_duration!(duration, "Duration");

    mediainfo_i64!(overall_bit_rate, "OverallBitRate");
    mediainfo_i64!(headersize, "HeaderSize");
    mediainfo_i64!(datasize, "DataSize");
    mediainfo_i64!(footersize, "FooterSize");

    mediainfo_date!(mastered_date, "Mastered_Date");
    mediainfo_date!(last_modification_date, "File_Modified_Date");
    mediainfo_date!(encoded_date, "Encoded_Date");
    mediainfo_date!(tagged_date, "Tagged_Date");

    pub fn writing_application(&self) -> MediaInfoResult<String> {
       match self.encoded_application() {
            Ok(x) => Ok(x),
            Err(_) => self.encoded_application_string(),
        }
    }
}

/* VideoStream */
impl VideoStream {
    mediainfo_attr!(stream_id, "ID");
    mediainfo_attr!(stream_size, "StreamSize");
    mediainfo_attr!(bit_rate, "BitRate");
    mediainfo_attr!(nominal_bit_rate, "BitRate_Nominal");
    mediainfo_attr!(bit_rate_mode, "BitRate_Mode");

    pub fn cbr(&self) -> bool {
        match self.bit_rate_mode() {
            Ok(x) => x == "Constant",
            Err(_) => false
        }
    }

    pub fn vbr(&self) -> bool {
        !self.cbr()
    }

    mediainfo_attr!(scan_order, "ScanOrder");
    mediainfo_attr!(scan_type, "ScanType");

    pub fn interlaced(&self) -> bool {
        match self.scan_type() {
            Ok(x) => x == "Interlaced",
            Err(_) => false
        }
    }

    pub fn progressive(&self) -> bool {
        !self.interlaced()
    }

    mediainfo_attr!(format, "Format");
    mediainfo_attr!(format_info, "Format_Info");
    mediainfo_attr!(format_profile, "Format_Profile");
    mediainfo_attr!(format_version, "Format_Version");
    mediainfo_attr!(format_settings_cabac, "Format_Settings_CABAC");
    mediainfo_attr!(format_settings_reframes, "Format_Settings_ReFrames");
    mediainfo_attr!(format_settings_matrix, "Format_Settings_Matrix");
    mediainfo_attr!(format_settings_gop, "Format_Settings_GOP");
    mediainfo_attr!(format_commercial, "Format_Commercial");
    mediainfo_attr!(colorimetry, "Colorimetry");
    mediainfo_attr!(colorspace, "ColorSpace");
    mediainfo_attr!(colour_primaries, "colour_primaries");
    mediainfo_attr!(transfer_characteristics, "transfer_characteristics");
    mediainfo_attr!(matrix_coefficients, "matrix_coefficients");
    mediainfo_attr!(codec_id, "CodecID");
    mediainfo_attr!(codec_info, "CodecID/Info");
    mediainfo_attr!(codec, "Codec");
    mediainfo_attr!(frame_rate, "FrameRate");
    mediainfo_attr!(nominal_frame_rate, "FrameRate_Nominal");
    mediainfo_attr!(minimum_frame_rate, "FrameRate_Minimum");
    mediainfo_attr!(maximum_frame_rate, "FrameRate_Maximum");
    mediainfo_attr!(frame_rate_mode, "FrameRate_Mode");
    mediainfo_attr!(display_aspect_ratio, "DisplayAspectRatio");
    mediainfo_attr!(bits_pixel_frame, "Bits-(Pixel*Frame)");
    mediainfo_duration!(duration, "Duration");
    mediainfo_i64!(bitdepth, "BitDepth");
    mediainfo_i64!(resolution, "Resolution");
    mediainfo_i64!(width, "Width");
    mediainfo_i64!(height, "Height");

    pub fn frame_size(&self) -> MediaInfoResult<String> {
        let height = self.height()?;
        let width = self.width()?;

        Ok(format!("{}x{}", width, height))
    }

    mediainfo_date!(encoded_date, "Encoded_Date");
    mediainfo_date!(tagged_date, "Tagged_Date");
    mediainfo_date!(standard, "Standard");
}

/* AudioStream */
impl AudioStream {
    mediainfo_attr!(stream_id, "ID");
    mediainfo_duration!(duration, "Duration");
    mediainfo_attr!(sampling_count, "SamplingCount");
    mediainfo_i64!(sampling_rate, "SamplingRate");
    mediainfo_attr!(stream_size, "StreamSize");
    mediainfo_attr!(bit_rate, "BitRate");
    mediainfo_attr!(bit_rate_mode, "BitRate_Mode");
    mediainfo_attr!(interleave_duration, "Interleave_Duration");
    mediainfo_i64!(resolution, "Resolution");
    mediainfo_attr!(format, "Format");
    mediainfo_attr!(format_profile, "Format_Profile");
    mediainfo_attr!(format_version, "Format_Version");
    mediainfo_attr!(format_info, "Format/Info");
    mediainfo_attr!(format_settings_sbr, "Format_Settings_SBR");
    mediainfo_attr!(format_settings_endianness, "Format_Settings_Endianness");
    mediainfo_attr!(format_settings_sign, "Format_Settings_Sign");
    mediainfo_attr!(codec_id, "CodecID");
    mediainfo_attr!(codec_info, "CodecID/Info");
    mediainfo_attr!(codec, "Codec");
    mediainfo_attr!(codec_id_hint, "CodecID/Hint");
    mediainfo_attr!(channel_positions, "ChannelPositions");
    mediainfo_i64!(channels, "Channel(s)");

    pub fn stereo(&self) -> bool {
        match self.channels() {
            Ok(x) => x == 2,
            Err(_) => false
        }
    }

    pub fn mono(&self) -> bool {
        match self.channels() {
            Ok(x) => x == 1,
            Err(_) => false
        }
    }

    mediainfo_date!(encoded_date, "Encoded_Date");
    mediainfo_date!(tagged_date, "Tagged_Date");
}

impl ImageStream {
    mediainfo_attr!(resolution, "Resolution");
    mediainfo_attr!(format, "Format");
    mediainfo_i64!(width, "Width");
    mediainfo_i64!(height, "Height");

    pub fn frame_size(&self) -> MediaInfoResult<String> {
        let height = self.height()?;
        let width = self.width()?;

        Ok(format!("{}x{}", width, height))
    }
}

/* TextStream */
impl TextStream {
    mediainfo_attr!(stream_id, "ID");
    mediainfo_attr!(format, "Format");
    mediainfo_attr!(codec_id, "CodecID");
    mediainfo_attr!(codec_info, "CodecID/Info");
}

/* OtherStream */
impl OtherStream {
    mediainfo_attr!(stream_id, "ID");
    mediainfo_attr!(other_type, "Type");
    mediainfo_attr!(timecode, "TimeCode_FirstFrame");
}

/* MenuStream */
impl MenuStream {
    mediainfo_attr!(stream_id, "ID");
    mediainfo_date!(encoded_date, "Encoded_Date");
    mediainfo_date!(tagged_date, "Tagged_Date");
    mediainfo_i64!(delay, "Delay");
}
