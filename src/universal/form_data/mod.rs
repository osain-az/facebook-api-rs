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
use tokio_util::codec::{BytesCodec, FramedRead};
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

pub fn extract_bytes(file_path: String, start: u64, end: u64) -> Form {
    //let mut form_data = FormData::new().unwrap()

    //  let path = Path::new(&video_params.file_path);
    let mut file1 = File::open(file_path.clone()).unwrap();
    let mut file = File::open(file_path).unwrap();

    let new_file = BufReader::new(file1);
    let let_try = new_file.take(20000);
    let mut buffer = [0; 60];
    //  let stream = FramedRead::new(file, BytesCodec::new());

    file.read_exact(&mut buffer).unwrap();
    // file.read_to_end(&mut buffer).unwrap();
    //println!("bytes from back {:?}", new_file);

    println!("workign upto here ");
    let test = &buffer.as_mut_slice();
    let bytes = Bytes::from(buffer.to_vec());
    println!("workign upto here after ");

    //let mut reader = BufReader::new(&*test.chun).buffer();
    let part = Part::bytes(buffer.to_vec());
    let new_test = Part::stream(bytes).file_name("video file");
    println!("file size from back {:?}", new_test);

    let form_data = Form::new().part("video_file_chunk", part);
    println!("file size from back {:?}", form_data);

    form_data
    //    files: vec![("source".to_string(), video_params.file_path.to_string())],
}
