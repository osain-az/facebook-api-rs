//! Facebook graph api for video uplaoding video by file upload
//!
//! This mod contains struct and methods for uploading video to facebook API.
//!
//!
use seed::fetch::{fetch, FormData};
use seed::prelude::{Method, Request};

use seed::{prelude::*, *};
use serde_json::Value;
use web_sys::File;
//use gloo_file::File;
use seed::prelude::js_sys::Math::exp;
use serde::{Deserialize, Serialize};

/// Facebook video api accepts different paramters that could be passed posted while uploading the video.
/// this struck will have the possible pramaters that a user might need to pass along the video upload
///
#[derive(Deserialize, Serialize)]
pub struct VideoParams {
    ///
    pub video_title: String,
    pub description: String,
    pub thum: String,
    pub content_category: ContentCategory,
    pub file_name: String,
    pub title: String,
}

pub struct UploadFile {
    pub file: File,
}

/// Enum for the different categories that the uploaded video will belong to as difined on facebook  graph api documentation.
/// Choose any from the list, if  no data is supplied a default value of  "OTHER" is choosen.
///
#[derive(Deserialize, Serialize)]
pub struct PostResponse {
    id: String,
}

#[derive(Deserialize, Serialize)]
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
            video_title: "this video we post".to_string(),
            description: "This video is for testing purpose".to_string(),
            thum: "".to_string(),
            content_category: ContentCategory::OTHER,
            file_name: "file".to_string(),
            title: " tThis is the best video all said ".to_string(),
        }
    }
}

impl VideoParams {
    pub fn new() -> VideoParams {
        VideoParams::default()
    }
    pub fn update_video_params(mut self, video_params: VideoParams) -> Self {
        Self { ..video_params }
    }

    pub fn get_video_params(self) -> VideoParams {
        self
    }

    fn create_form_data(self, upload_phase: String, chunked_file: File) -> FormData {
        // better way should be a loop  and append the data to the form
        let mut form_data = FormData::new(); // init empty form data
        form_data.append_blob("source", &chunked_file); // appped  the  current chunked file   to the form

        if !self.video_title.is_empty() {
            form_data.append_str("video_title", &self.video_title);
        }

        if !self.description.is_empty() {
            form_data.append_str("description", &self.description);
        }

        if !self.thum.is_empty() {
            form_data.append_str("thum", &self.thum);
        };
        if !self.title.is_empty() {
            form_data.append_str("title", &self.title);
        };
        form_data
    }
}

#[derive(Deserialize, Default, Serialize)]

pub struct VideoApi {
    base_url: String,
    page_access_token: String,
    video_post_type: String,
    video_params: Option<VideoParams>,
}

impl VideoApi {
    pub fn new(base_url: String, page_access_token: String) -> VideoApi {
        VideoApi {
            base_url,
            page_access_token,
            video_post_type: "".to_string(),
            video_params: None,
        }
    }

    /// This method accepts video file in a struct  and VideoPrams struct  which will then decide either using resumable video upload or non_resumable method
    /// as defined by facebook  video upload api.

    pub fn init_video_upload(
        mut self,
        upload_file: UploadFile,
        video_params: VideoParams,
    ) -> VideoApi {
        let non_resumable_max_size_gb = 0.9; //Gb: facebook recommmended max 1 Gb for none resumabl upload
        let resumable_max_size_gb = 4.0; //Gb: facebook recommmended max max of 4  Gb for  resumabl uploading video
        let file = upload_file.file;
        //self.video_file   = file;
        let file_size_byte = file.size() as f64; // file size in byte
        let file_size_gb = file_size_byte / 10_f64.powf(9.0); // convert the file to Gb
        if file_size_gb < non_resumable_max_size_gb {
            self.video_post_type = "non_resumable".to_string();
            self.video_params = Some(video_params);
            self
        } else {
            // this will be for larger videos
            self.video_post_type = "non_resumable".to_string();
            self.video_params = None;
            self
        }
    }

    fn create_form_data(
        upload_phase: String,
        chunked_file: File,
        video_params: VideoParams,
    ) -> FormData {
        let mut form_data = FormData::new();
        form_data.append_blob("source", &chunked_file); // appped  the  current chunked file   to the form

        if !video_params.video_title.is_empty() {
            form_data.append_str("video_title", &video_params.video_title);
        }

        if !video_params.description.is_empty() {
            form_data.append_str("description", &video_params.description);
        }

        if !video_params.thum.is_empty() {
            form_data.append_str("thum", &video_params.thum);
        };

        form_data
    }
    ///This method
    pub async fn post_video(self, uploaded_file: UploadFile) -> seed::fetch::Result<PostResponse> {
        if self.video_post_type == "non_resumable" {
            VideoApi::non_resumable_video_upload(&self, uploaded_file).await
        } else {
            // this will be when uploading large file
            VideoApi::non_resumable_video_upload(&self, uploaded_file).await
        }
    }
    pub async fn non_resumable_video_upload(
        &self,
        uploaded_file: UploadFile,
    ) -> seed::fetch::Result<PostResponse> {
        let form_data =
            VideoApi::create_form_data("".to_string(), uploaded_file.file, Default::default());
        let base_url = self.base_url.replace("EDGE", "videos");
        let url = base_url + "?access_token=" + &self.page_access_token;
        let request = Request::new(url).method(Method::Post).form_data(form_data);
        fetch(request).await?.json::<PostResponse>().await
    }
}
