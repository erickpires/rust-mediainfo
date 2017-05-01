#![allow(dead_code)]
extern crate libc;

mod c_w_string;
pub mod ffi;

use ffi::MediaInfoStream;
use ffi::MediaInfoInfo;

pub type MediaInfo = ffi::MediaInfo;

impl MediaInfo {
    pub fn from_data(&mut self, data: &[u8]) -> Result<(), String>{
        let data_len = data.len();
        if data_len == 0 { return Err("Data length is 0".to_string()); }

        self.open_buffer_init(data_len as u64, 0);
        let continue_result = self.open_buffer_continue(data);
        let finalize_result = self.open_buffer_finalize();

        if continue_result & 0x01 == 0 || finalize_result == 0 {
            return Err("Could not read buffer".to_string());
        }

        Ok( () )
    }

    pub fn available_parameters(&mut self) -> String {
        self.option("Info_Parameters", "")
    }

    pub fn get_title(&mut self) -> String {
        self.get_with_default_options("Title")
    }

    pub fn get_performer(&mut self) -> String {
        self.get_with_default_options("Performer")
    }

    pub fn get_album(&mut self) -> String {
        self.get_with_default_options("Album")
    }

    pub fn get_genre(&mut self) -> String {
        self.get_with_default_options("Genre")
    }

    pub fn get_track_name(&mut self) -> String {
        self.get_with_default_options("Track")
    }

    pub fn get_duration_ms(&mut self) -> Option<u32> {
        let result_str = self.get_with_default_options("Duration");
        let result = result_str.parse::<u32>();

        match result {
            Ok(num) => Some(num),
            Err(_)  => None,
        }
    }

    pub fn get_track_number(&mut self) -> Option<u32> {
        let result_str = self.get_with_default_options("Track/Position");
        let result = result_str.parse::<u32>();

        match result {
            Ok(num) => Some(num),
            Err(_)  => None,
        }
    }

    pub fn get_with_default_options(&mut self, parameter: &str) -> String {
        self.get(MediaInfoStream::General, 0, parameter,
                 MediaInfoInfo::Text, MediaInfoInfo::Name)
    }
}
