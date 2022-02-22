use crate::prelude::video::VideoParams;
use bytes::{Buf, BufMut, Bytes, BytesMut};
#[cfg(any(feature = "reqwest_async"))]
use reqwest::multipart::{Form, Part};
use reqwest::Body;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
//use tokio::fs::File;

#[cfg(any(feature = "reqwest_async"))]
pub fn create_form_data(video_params: VideoParams) -> Form {
    //let mut form_data = FormData::new().unwrap()
    let form_data = Form::new();
    //  let path = Path::new(&video_params.file_path);
    let mut file = File::open(video_params.file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    //let mut reader = BufReader::new(file).buffer();
    let part = Part::bytes(buffer).file_name("vdeoe ");
    form_data
        .text("video_title", video_params.video_title)
        .text("description", video_params.description)
        .text("thum", video_params.thum)
        .part("source", part)

    //    files: vec![("source".to_string(), video_params.file_path.to_string())],

}
