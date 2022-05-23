use crate::prelude::video::{UploadPhase, VideoParams};
use reqwest::multipart::{Form, Part};
use std::borrow::BorrowMut;
use std::fs::File;

/// Creating form_data for reqwest Client
#[cfg(any(feature = "reqwest_async"))]
pub fn resumable_form_data_reqwest(
    upload_phase: UploadPhase,
    upload_session_id: String,
    start_offset: String,
    mut video_params: VideoParams,
    mut file: File,
) -> Form {
    // phase is expected to be of an enum of either , start, transfer, and end
    // depending on the  uplaoding stage
    use reqwest::Body;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    let mut current_upload_phase = "";

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let params = video_params.borrow_mut();
    let part = Part::bytes(buffer).file_name("vdeoe ");

    let formdata = match upload_phase {
        UploadPhase::start => {
            current_upload_phase = "start";
            println!("file zise {}", file.metadata().unwrap().clone().len());
            let form_data = Form::new()
                .text("file_size", file.metadata().unwrap().len().to_string())
                .text(" upload_phase", "start");

            form_data
        }

        UploadPhase::transfer => {
            current_upload_phase = "transfer";
            //  let form_data = Form::new()
            // .text("start_offset", start_offset.clone())
            // .part(
            // "video_file_chunk",
            // file_part(video_params.file_path.clone(), 0, 0),
            // )
            // .text("upload_session_id", upload_session_id.clone())
            // .text("upload_phase", current_upload_phase)
            // .text("access_token", self.page_access_token.clone());

            let form_data = Form::new();
            form_data
        }

        UploadPhase::finish => {
            current_upload_phase = "finish";
            let mut main_form_data = Form::new();
            let form_data = Form::new()
                .text("upload_session_id", upload_session_id.clone())
                .text("upload_phase", current_upload_phase);

            if !video_params.video_title.is_empty() {
                main_form_data =
                    Form::from(form_data).text("video_title", video_params.video_title.clone());
            }

            if !video_params.description.is_empty() {
                main_form_data = Form::from(main_form_data)
                    .text("description", video_params.description.clone());
            }

            if !video_params.thum.is_empty() {
                main_form_data = Form::from(main_form_data).text("thum", video_params.thum.clone());
            };
            main_form_data
        }
        // this method has not been implimented yet.
        // Todo:: added method for cancelling upload
        UploadPhase::cancel => {
            current_upload_phase = "cancel";

            let form_data = Form::new()
                .text("upload_session_id", upload_session_id.clone())
                .text("start_offset", start_offset.clone())
                .text("upload_session_id", upload_session_id.clone())
                .text("upload_phase", current_upload_phase);
            form_data
        }
        _ => {
            // this will never occure
            current_upload_phase = "";
            let form_data = Form::new();
            form_data
        }
    };
    formdata
}
