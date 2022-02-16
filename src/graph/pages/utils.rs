//! This utils will contain different methods and struct that are shared within the pages  mod
//!

use crate::prelude::video::{UploadPhase, VideoParams};
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
use web_sys::{Blob, File, FormData};

#[cfg(any(feature = "reqwest_async"))]
use reqwest::multipart::{Form, Part};

#[derive(Deserialize, Debug, Serialize)]
pub struct Fields {
    pub(crate) fields: Vec<String>,
}

impl Default for Fields {
    /// This parameters are used as fields which are passed in as a query
    /// parameters to the get post request and feeds request  
    fn default() -> Self {
        let field_list = vec![
            "from",
            "id",
            "message_tags",
            "story",
            "story_tags",
            "permalink_url",
            "message",
            "shares",
            "comments",
            "likes",
            "reactions",
        ];
        let fields = field_list.iter().map(|&field| field.into()).collect();
        Self { fields }
    }
}

/// expected fields gotten from the get post request
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct GetPostResponse {
    pub id: String,
    pub message: String,
    pub from: From,
    pub permalink_url: String,
}

#[derive(Deserialize, Debug, Default, Serialize)]
pub struct From {
    pub id: String,
    pub name: String,
}

pub struct PostByLinkParams {
    file_url: String,
}

#[derive(Clone, Deserialize, Debug, Serialize, Default)]
pub struct UploadingData {
    pub file_path: String,
    pub end_offset: u64,
    pub start_offset: u64,
    pub upload_phase: String,
    pub upload_session_id: String,
}

impl UploadingData {
    pub fn new(
        file_path: String,
        end_offset: u64,
        start_offset: u64,
        upload_phase: String,
        upload_session_id: String,
    ) -> Self {
        UploadingData {
            file_path,
            end_offset,
            start_offset,
            upload_phase,
            upload_session_id,
        }
    }
}

/// Response for successful uploading of video using non_resumable method
#[derive(Deserialize, Debug, Serialize)]
pub struct PostResponse {
    id: String,
}

/// During each chunk blob file uploaded, facebook will send a response back,
/// This struct is the response gotten for each video chunk  sent
#[derive(Deserialize, Clone, Serialize)]
pub struct ChunksUploadResponse {
    pub start_offset: String, // Value for second chunk
    pub end_offset: String,
}

impl ChunksUploadResponse {
    /// creating an instance of the chunking paramters
    pub fn new(start_offset: String, end_offset: String) -> ChunksUploadResponse {
        ChunksUploadResponse {
            start_offset,
            end_offset,
        }
    }

    pub fn end_offset(self) -> String {
        self.end_offset
    }

    pub fn start_offset(self) -> String {
        self.start_offset
    }
}

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

/// Creating form_data for reqwest Client
#[cfg(any(feature = "reqwest_async"))]
pub fn resumable_form_data_reqwest(
    upload_phase: UploadPhase,
    upload_session_id: String,
    start_offset: String,
    video_params: VideoParams,
) -> Form {
    // phase is expected to be of an enum of either , start, transfer, and end
    // depending on the  uplaoding stage
    use reqwest::Body;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    let mut current_upload_phase = "";

    //  let path = Path::new(&video_params.file_path);
    let mut file = File::open(video_params.file_path.clone()).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    //let mut reader = BufReader::new(file).buffer();
    let part = Part::bytes(buffer).file_name("vdeoe ");
    //println!("the file size {:?} ", &file.bytes());
    let formdata = match upload_phase {
        UploadPhase::start => {
            println!("this is the start");
            current_upload_phase = "start";
            println!("file zise {}", file.metadata().unwrap().clone().len());
            let form_data = Form::new()
                .text("file_size", file.metadata().unwrap().len().to_string())
                .text(" upload_phase", "start");

            form_data
        }

        UploadPhase::transfer => {
            current_upload_phase = "transfer";
            /*  let form_data = Form::new()
            .text("start_offset", start_offset.clone())
            .part(
                "video_file_chunk",
                file_part(video_params.file_path.clone(), 0, 0),
            )
            .text("upload_session_id", upload_session_id.clone())
            .text("upload_phase", current_upload_phase)
            .text("access_token", self.page_access_token.clone());*/

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
        //Todo:: added method for cancelling upload
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
            //this will never occure
            current_upload_phase = "";
            let form_data = Form::new();
            form_data
        }
    };
    formdata
}
