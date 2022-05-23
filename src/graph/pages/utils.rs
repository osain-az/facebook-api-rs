//! This utils will contain different methods and struct that are shared within the pages  mod
//!

use crate::prelude::video::{UploadPhase, VideoParams};
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
use web_sys::{Blob, File, FormData};


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
#[derive(Deserialize, Debug, Serialize)]
pub struct GetPostResponse {
    pub id: String,
    pub message: String,
    pub from: From,
    pub permalink_url: String,
}
/// expected fields gotten from the get post request
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct ResponseStatus {
    pub id: String,
}

#[derive(Deserialize, Debug, Serialize)]
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



