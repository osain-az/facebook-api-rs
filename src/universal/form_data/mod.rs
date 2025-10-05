use crate::prelude::video::VideoParams;

#[cfg(any(feature = "reqwest"))]
use reqwest::multipart::{Form, Part};
use std::io::Read;

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
