use crate::prelude::video::VideoParams;
use bytes::{Buf, BufMut, Bytes, BytesMut};
#[cfg(any(feature = "reqwest_async"))]
use reqwest::multipart::{Form, Part};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[cfg(any(feature = "reqwest_async"))]
pub fn file_part(file_path: String, session_id: String, page_token: String) -> Form {
    //let mut form_data = FormData::new().unwrap()

    let mut file = File::open(file_path).unwrap();
    let mut buffer = [0; 1048576];
    file.read_exact(&mut buffer).unwrap();
    let bytes = Bytes::from(buffer.to_vec());
    println!("page token :{}", page_token.clone());
    let part = reqwest::multipart::Part::stream(bytes);
    let form = reqwest::multipart::Form::new()
        .part("video_file_chunk", part)
        .text("?upload_phase", "transfer")
        .text("access_token", page_token)
        .text("start_offset", "0")
        .text("upload_session_id", session_id);
    form
}
