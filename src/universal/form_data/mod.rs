use crate::prelude::video::VideoParams;

#[cfg(any(feature = "reqwest_async"))]
use reqwest::multipart::{Form, Part};
use reqwest::Body;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

#[cfg(any(feature = "reqwest_async"))]
pub fn create_form_data(video_params: VideoParams, buffer: Vec<u8>) -> Form {
    let form_data = Form::new();

    let part = Part::bytes(buffer).file_name("vdeoe ");
    form_data
        .text("video_title", video_params.video_title)
        .text("description", video_params.description)
        .text("thum", video_params.thum)
        .part("source", part)
}

pub fn form_data_with_bytes(buffer: Vec<u8>, video_params: VideoParams) -> Form {
    let part = Part::bytes(buffer).file_name("vdeoe ");

    reqwest::multipart::Form::new()
        .text("thum", video_params.thum)
        .text("description", video_params.description)
        .part("source", part)
}
