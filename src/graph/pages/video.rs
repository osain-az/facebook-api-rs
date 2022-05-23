//! The Facebook Video API allows you to publish Videos on Pages and Groups.
//! Publishing on Users is not supported.
//!
//! The process for publishing Videos involves choosing an upload protocol and
//! sending a POST request to the targeted Page or Group's  to end point /videos
//! edge. The API  supports  postting video by video_url, Resumable and
//! Non-Resumableupload protocols.
//! Facebook recommend that you use the Resumable Upload protocol
//! as it is more  versatile and can gracefully handle connection interruptions.
//! To post to either page or group pass either of the id.  (/event-id/videos,
//! /page-id/videos /group-id/videos
//!
//! For other information check on facebook documentation  for
//! video API  <https://developers.facebook.com/docs/video-api/guides/publishing>
//! For information on different opertaions on facebook page  check  <https://developers.facebook.com/docs/graph-api/reference/page/videos/#Creating>

use crate::prelude::errors::ClientErr;
use crate::prelude::utils::{ChunksUploadResponse, PostResponse, UploadingData};
use crate::prelude::HttpConnection;
#[cfg(any(feature = "reqwest_async"))]
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[cfg(any(feature = "web_sys_async"))]
use web_sys::{Blob, File, FormData};

#[cfg(any(feature = "reqwest_async"))]
use crate::prelude::media_upload::video_by_reqwest::VideoApi_reqwest;

#[cfg(any(feature = "web_sys_async"))]
use crate::prelude::media_upload::video_by_web_sys::VideoApi_seed;

/// Facebook video api accepts different parameters that could be passed to the
/// post request while uploading the video. this struck will have the possible
/// parameters that a user might need to pass along the video while publishing.
/// Note : video_title, file_name, and title will not appear in your feed. use
/// "description" to describe your video  which will appear at the top of the
/// post.
#[derive(Clone, Deserialize, Serialize)]
pub struct VideoParams {
    /// The video_title parameter will not be display on your post feed
    pub video_title: String,
    /// The description parameter is used to describe your video  which will
    /// appear at the top of the post
    pub description: String,

    /// Format: BMP, GIF, JPEG, PNG,TIFF
    /// File Size: 10MB or less.
    /// There are no image dimension requirements, but it should share the same
    /// aspect ratio as your video.
    pub thum: String,

    /// Enum for different categories that the uploaded video will belong to
    /// as defined on facebook  graph api documentation. Choose any from the
    /// list, if no data is supplied a default value of  "OTHER" is chosen.
    pub content_category: ContentCategory,

    pub title: String,
}

/// UsageType is an enum that is use to indication where  you are using the
/// method,  Client or Server
#[derive(Deserialize, Debug, Serialize)]
pub enum UsageType {
    /// Client is when you are using it for at frontend or wasm
    Client,
    /// Server is when you are using this method at the backend
    Server,
}
/// Response for successful uploading of video using non_resumable method

/// Enum for different categories that the uploaded video will belong to as
/// defined on facebook graph api documentation. Choose any from the list, if no
/// data is supplied a default value of "OTHER" is chosen.
#[derive(Deserialize, Serialize, Copy, Clone)]
#[allow(warnings)]
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

/// This struct is the response gotten when initializing the resumable uploading
/// method process.
#[derive(Deserialize, Debug, Clone)]
struct InitializeUploadResponse {
    pub video_id: String,
    pub end_offset: String,
    pub upload_session_id: String,
}

/// This struct is the response upon successful  upload of video using resumable
/// method. The struct is constructed  using different data gotten from
/// different responses while using the resumable  method. if the success
/// parameter in the struct is true then the video was uploaded successfully
/// Note: for video uploaded using the video_ur method, only the video_id
/// parameter will have a value other will be empty  since other parameter are
/// not used.
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct FinalResponeResumableUpload {
    // this struct will be data constructed from all the different uploads
    success: bool,
    upload_session_id: String, // will developer need this
    video_id: String,
}

// this will be used to update this method upon each round of chunked upload
impl FinalResponeResumableUpload {
    /// This method is used to update  the struct during a resumabl upload
    /// method since each chunk upload different data are sent back.
    pub fn update_params(
        mut self,
        video_id: String,
        upload_session_id: String,
    ) -> FinalResponeResumableUpload {
        self.upload_session_id = upload_session_id;
        self.video_id = video_id;
        self
    }

    /// This method is used to get the response data for the first request
    /// during a resumable upload
    pub fn new(
        video_id: String,
        upload_session_id: String,
        success: bool,
    ) -> FinalResponeResumableUpload {
        FinalResponeResumableUpload {
            video_id,
            upload_session_id,
            success,
        }
    }

    /// This method is used to get the response data for the final request
    /// during a resumable upload the response will either be true or false.
    pub fn update_success(mut self, success: bool) -> FinalResponeResumableUpload {
        self.success = success;
        self
    }

    /// This method will return the struct of all the paramters
    pub fn response(self) -> FinalResponeResumableUpload {
        self
    }
}

/// After complete uploading of the video through resumable, facebook will send
/// reponse which will be either true or false. true means, the video was
/// uplaoded successfull
#[derive(Deserialize, Serialize)]
struct ResumableUploadFinal {
    // response from facebook, true or false
    success: bool,
}

/// While using the resumable upload method, there are 4 enums which are used to
/// track the  progress/stage and status of the uploading processs which are
/// send along each upload. "start" is send when initialzing the upload process,
/// "transfer" is send when uploading is in progress, "finished is send when the
/// chunk files are finish uploading, and "cancel" is send when for you decide
/// to stop the uploading.
pub enum UploadPhase {
    start,
    transfer,
    finish,
    cancel,
}

impl Default for VideoParams {
    /// this default method will create the struct will default
    fn default() -> Self {
        Self {
            video_title: "".to_string(),
            description: "video feed".to_string(),
            thum: "".to_string(),
            content_category: ContentCategory::OTHER,
            title: " ".to_string(),
        }
    }
}

impl VideoParams {
    /// This method will update an existing data in your VideoPramas
    pub fn update_video_params(self, video_params: VideoParams) -> Self {
        Self { ..video_params }
    }

    /// This method will return the existing data in your VideoPramas
    pub fn video_params(self) -> VideoParams {
        self
    }

    pub fn new(
        video_title: String,
        description: String,
        thum: String,
        content_category: ContentCategory,
        title: String,
    ) -> Self {
        VideoParams {
            video_title,
            description,
            thum,
            content_category,
            title,
        }
    }
}

#[derive(Deserialize, Clone, Serialize)]
pub struct VideoApi {
    base_url: String,
    page_access_token: String,
}

impl VideoApi {
    pub fn new(base_url: String, page_access_token: String) -> VideoApi {
        VideoApi {
            base_url,
            page_access_token,
        }
    }
}

#[derive(Deserialize, Debug, Default, Serialize)]
struct FeedPostSuccess {
    id: String,
}

impl VideoApi {
    /// This method is used for uploading large video files, it does that by
    /// chunking the file and uplaoding them individually until is complete.
    /// The method takes two parameter( file, video parameter struct).
    /// the waiting time depend  on the video size uplaoded
    ///
    /// Note there is an issue with chunking method that only chunk smaller size
    /// so extra time than usuall is expect until the issue is fixed.
    ///
    /// for more infromation  check  https://developers.facebook.com/docs/video-api/guides/publishing
    #[cfg(any(feature = "web_sys_async"))]
    pub async fn resumable_post(
        &self,
        file: File,
        video_param: VideoParams,
    ) -> Result<FinalResponeResumableUpload, ClientErr> {
        let base_url = self.base_url.clone();
        let page_token = self.page_access_token.clone();

        VideoApi_seed::new(base_url, page_token)
            .resumable_post(file, video_param)
            .await
    }

    #[cfg(any(feature = "reqwest_async"))]
    pub async fn resumable_post(
        &self,
        file: File,
        video_param: VideoParams,
    ) -> Result<FinalResponeResumableUpload, ClientErr> {
        let base_url = self.base_url.clone();
        let page_token = self.page_access_token.clone();

        VideoApi_reqwest::new(base_url, page_token)
            .resumable_post(video_param, file)
            .await
    }

    /// facebook recommend that you upload files using the Resumable Upload
    /// method because it handles connection interruptions more efficiently
    /// and supports larger files. However, if you prefer to upload files
    /// using the Non-Resumable Upload method.
    ///
    /// This method is expecting a video file less than 1 gb, and a video
    /// parameter struct,  if the video file is within this range it post
    /// the video but if the video is not within the range , the post will
    /// not be made but a Fetcherror will be gerated.
    #[cfg(any(feature = "web_sys_async"))]
    pub async fn non_resumable_post(
        &self,
        video_params: VideoParams,
        file: File,
    ) -> Result<PostResponse, ClientErr> {
        let base_url = self.base_url.clone();
        let page_token = self.page_access_token.clone();
        VideoApi_seed::new(base_url, page_token)
            .non_resumable_post(video_params, file)
            .await
    }

    #[cfg(any(feature = "reqwest_async"))]
    pub async fn non_resumable_post(
        &self,
        video_params: VideoParams,
        mut file: File,
    ) -> Result<PostResponse, ClientErr> {
        let base_url = self.base_url.clone();
        let page_token = self.page_access_token.clone();

        VideoApi_reqwest::new(base_url, page_token)
            .non_resumable_post(video_params, file)
            .await
    }
}
impl VideoApi {
    /// This Method is used for posting media  by there url.
    ///
    /// useage
    ///
    /// .post_by_link(video_url, description, title)

    pub async fn post_by_link(
        &self,
        file_url: &str,
        description: &str,
        title: &str,
    ) -> Result<FinalResponeResumableUpload, ClientErr> {
        let url = self.base_url.replace("EDGE", "videos")
            + "?file_url="
            + file_url
            + "&access_token="
            + &self.page_access_token
            + "&title="
            + title
            + &"description="
            + description;

        let video_id = HttpConnection::post::<FeedPostSuccess, String>(url, "".to_string()).await?;
        if video_id.id.is_empty() {
            Err(ClientErr::FacebookError(
                "The video posting by url was not suceessfull ".to_string(),
            )) // try to generate a customer
        } else {
            Ok(FinalResponeResumableUpload::default().update_params(video_id.id, "".to_string()))
        }
    }
}
