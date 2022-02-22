
use web_sys::{Blob, File, FormData};
use crate::prelude::video::{UploadPhase, VideoParams};
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
pub fn form_data_seed(file: File, video_params: VideoParams) -> FormData {
    let mut form_data = FormData::new().unwrap();

    form_data.append_with_blob("source", &file); // appped  the  current chunked file   to the form

    if !video_params.video_title.is_empty() {
        form_data.append_with_str("video_title", &video_params.video_title);
    }

    if !video_params.description.is_empty() {
        form_data.append_with_str("description", &video_params.description);
    }

    if !video_params.thum.is_empty() {
        form_data.append_with_str("thum", &video_params.thum);
    };

    form_data
}

#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
pub fn resumable_form_data_seed(
    upload_phase: UploadPhase,
    current_blob_file: Blob,
    uploaded_file: File,
    upload_session_id: String,
    start_offset: String,
    video_params: VideoParams,
) -> FormData {
    // phase is expected to be of an enum of either , start, transfer, and end
    // depending on the  uplaoding stage
    let mut current_upload_phase = "";
    let mut form_data = FormData::new().unwrap();
    use crate::prelude::FileResult;

    match upload_phase {
        UploadPhase::start => {
            current_upload_phase = "start";
            form_data.append_with_str(
                "file_size",
                FileResult::file_analyze(uploaded_file)
                    .file_size_byte_string()
                    .as_str(),
            ); // add the video size
        }
        UploadPhase::transfer => {
            current_upload_phase = "transfer";
            form_data.append_with_str("start_offset", &start_offset);
            form_data.append_with_blob("video_file_chunk", &current_blob_file);
        }

        UploadPhase::finish => {
            current_upload_phase = "finish";

            form_data.append_with_str("upload_session_id", &upload_session_id);
            if !video_params.video_title.is_empty() {
                form_data.append_with_str("video_title", &video_params.video_title);
            }

            if !video_params.description.is_empty() {
                form_data.append_with_str("description", &video_params.description);
            }

            if !video_params.thum.is_empty() {
                form_data.append_with_str("thum", &video_params.thum);
            };
        }
        // this method has not been implimented yet.
        UploadPhase::cancel => {
            form_data.append_with_str("upload_session_id", &upload_session_id);
            form_data.append_with_str("start_offset", &start_offset);
        }
    }

    form_data.append_with_str("upload_session_id", &upload_session_id);
    form_data.append_with_str("upload_phase", current_upload_phase);
    form_data
}