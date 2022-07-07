use crate::prelude::video::VideoParams;

#[cfg(any(feature = "reqwest"))]
use reqwest::multipart::{Form, Part};
use reqwest::Body;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

#[cfg(any(feature = "reqwest"))]
pub fn create_form_data(video_params: VideoParams, buffer: Vec<u8>) -> Form {
    let form_data = Form::new();

    let part = Part::bytes(buffer).file_name("video ");

    form_data
        .text("description", video_params.description)
        //.text("thumb", video_params.thumb)
        .part("source", part)
}

pub fn form_data_with_bytes(buffer: Vec<u8>, video_params: VideoParams) -> Form {
    let part = Part::bytes(buffer).file_name("video ");

    reqwest::multipart::Form::new()
       // .text("thum", video_params.thum)
        .text("description", video_params.description)
        .part("source", part)
}
