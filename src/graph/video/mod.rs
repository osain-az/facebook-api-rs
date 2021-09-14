//! Facebook graph api for video uplaoding video by file upload
//!
//! This mod contains struct and methods for uploading video to facebook API.
//!
//!
use seed::fetch::fetch;
use seed::prelude::{Method, Request};

use seed::{prelude::*, *};

use seed::prelude::js_sys::Math::exp;
use serde::{Deserialize, Serialize};

/// Facebook video api accepts different paramters that could be passed posted while uploading the video.
/// this struck will have the possible pramaters that a user might need to pass along the video upload
pub struct VideoParams {
    video_title: String,
    file_url: String, // this is when uploading with link
    description: String,
    sponsor_relationship: String,
    scheduled_publish_time: String,
    replace_video_id: String,
    thum: String,
    content_category: ContentCategory,
    upload_phase: UploadPhase,
    upload_session_id: String,
    //uploaded_file: File,
}

/// enum used for tracking the process of uploading chunked files to facebook using resumable method,
/// each chunked upload shall contain one of this. The "start" enum is used in initializing the uploading.
///
enum UploadPhase {
    start,
    transfer,
    finish,
    cancel,
}

/// Enum for the different categories that the uploaded video will belong to as difined on facebook  graph api documentation.
/// Choose any from the list, if  no data is supplied a default value of  "OTHER" is choosen.
///
pub enum ContentCategory {
    EAUTY_FASHION,
    BUSINESS,
    CARS_TRUCKS,
    COMEDY,
    CUTE_ANIMALS,
    ENTERTAINMENT,
    FAMILY,
    FOOD_HEALTH,
    HOME,
    LIFESTYLE,
    MUSIC,
    NEWS,
    POLITICS,
    SCIENCE,
    SPORTS,
    TECHNOLOGY,
    VIDEO_GAMING,
    OTHER,
}
impl Default for VideoParams {
    /// this default method will create the struct will default
    fn default() -> Self {
        Self {
            video_title: "".to_string(),
            file_url: "".to_string(),
            description: "".to_string(),
            sponsor_relationship: "".to_string(),
            scheduled_publish_time: "".to_string(),
            replace_video_id: "".to_string(),
            thum: "".to_string(),
            content_category: ContentCategory::OTHER,
            upload_phase: UploadPhase::start,
            upload_session_id: "".to_string(),
        }
    }
}

impl VideoParams {
    fn new() -> VideoParams {
        VideoParams::default()
    }

    /*   pub fn init_video_upload(self, base_url: String) {
        let file_size_byte = self.uploaded_file.metadata().unwrap().len();
        let  file_size_gb =  file_size_byte/
        let  non_resumable_max_size_gb =  0.9; //Gb: facebook recommmended max 1 Gb for none resumabl upload
        if self.uploaded_file.metadata().unwrap().len()+ > exp(6) {}
    }*/
    /*
    pub async fn non_resumable_upload(&self, file: File) -> seed::fetch::Result<FeedPostSuccess> {
        let base_url = self.base_url.replace("EDGE", "videos");
        let url = base_url + "?file_url=" + file_url + "&access_token=" + &self.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<FeedPostSuccess>().await
    }*/
}
