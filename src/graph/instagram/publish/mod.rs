//! This represent facebook (instagram) content API which hase endpoint for posting and publishing of media (video and photo).
//! POST /{ig-user-id}/media — upload media and create media containers.
//! POST /{ig-user-id}/media_publish — publish uploaded media using their media containers.
//! For details of the API endpoints and it requirements refer to facebook documentation.
//!  <https://developers.facebook.com/docs/instagram-api/guides/content-publishing>
//!
//! Limitations
//! Can only be used to publish to business IG User accounts; Creator IG User accounts are not supported.
//! Accounts are limited to 25 API-published posts within a 24 hour period.
//! JPEG is the only image format supported. Extended JPEG formats such as MPO and JPS are not supported.
//! Stories are not supported.
//! Shopping tags are not supported.
//! Branded content tags are not supported.
//! Filters are not supported.
//! Multi-image posts are not supported.
//! If the caption contains a hashtag, it should be HTML URL-encoded as %23 in the request.
//! Publishing to Instagram TV is not supported.
//!

use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

/// This struct present  possible data  for posting to instagram account,
/// only the url is required for posting.
#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstaPostParams {
    pub url: String,
    pub caption: String,
    pub location_id: String,
    pub tag_users: Option<Vec<User>>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct User {
    username: String,
    x: f32,
    y: f32,
}

impl InstaPostParams {
    /// This method let developer update the feed parameters by keeping tract of
    /// each  inputted values
    pub fn new(url: String, caption: String, location_id: String, tag_users: Option<Vec<User>>) -> Self {
        InstaPostParams { url, caption, location_id, tag_users }
    }
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstaMediaContainerId {
    pub id: String,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct InstagramPostApi {
    access_token: String,
    base_url: String,
}

impl InstagramPostApi {
    pub fn new(access_token: String, base_url: String) -> InstagramPostApi {
        InstagramPostApi {
            access_token,
            base_url,
        }
    }

    /// The post_media method is used to upload media and create media containers of video or photo.
    /// media parameter is posted  to the IG container, then a
    /// container ID is sent back that which can be used in the  publish_ media method  to published the media.
    /// Note: your media will not be visible until the publish_media method is called.
    /// If the publish_media is not called until 24 hour, the media will be deleted.
    ///
    /// After posting the media and the media_id is returned in the response, the video will not be ready to be published
    /// immediately, therefore the developer must ensure that the publish_method is wait for 30 to 60 seconds before sending the published request.
    /// Alternatively, to ensure that media is ready, you can called the "status" method in the media endpoint to get the status.
    ///
    /// Limitation
    /// Video:
    ///      Container: MOV or MP4 (MPEG-4 Part 14), no edit lists, moov atom at the front of the file.
    ///      publAudio codec: AAC, 48khz sample rate maximum, 1 or 2 channels (mono or stereo).ish_video " method. This method is expecting a  InstaPostParams
    ///      Video codec: HEVC or H264, progressive scan, closed GOP, 4:2:0 chroma subsampling.
    ///      Duration: 60 seconds maximum, 3 seconds minimum.
    ///      File size: 100MB maximum.
    ///      Publishing to Instagram TV is not supported.
    /// Photo:
    ///      Formats: JPEG .
    ///       Maximum file size: 8MiB
    ///       Aspect ratio: Must be within a 4:5 to 1.91:1 range
    ///       Minimum width: 320 (will be scaled up to the minimum if necessary)
    ///       Maximum width: 1440 (will be scaled down to the maximum if necessary)
    ///
    /// for more details check facebook reference <https://developers.facebook.com/docs/instagram-api/reference/ig-user/media#creating> or
    /// the general content reference    <https://developers.facebook.com/docs/instagram-api/guides/content-publishing>

    pub async fn post_media(
        self,
        post_params: InstaPostParams,
        media_type: String,
    ) -> Result<InstaMediaContainerId, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "media");
        let mut url: String;
        let caption = encode(&post_params.caption);
        if media_type == "video" {
            url = base_url
                + "?media_type=VIDEO"
                + "&video_url="
                + &post_params.url
                + "&access_token="
                + &self.access_token;
        } else {
            url =
                base_url + "?image_url=" + &post_params.url + "&access_token=" + &self.access_token;
        }

        if !post_params.location_id.is_empty() {
            url = url + "location_id=" + &post_params.location_id
        };
        if !post_params.caption.is_empty() {
            url = url + "&caption=" + &caption.to_string()
        };

        let resp =
            HttpConnection::post::<InstaMediaContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// This  should be used when the container id of the feed is ready, this
    /// method will published the media that has been posted,
    /// It only accept the  container id of the posted media you want to published.
    ///
    /// After posting the media and the media_id is returned in the response, the video will not be ready to be published
    /// immediately, therefore the developer must ensure that the publish_method is wait for 30 to 60 seconds before sending the published request.
    /// Alternatively, to ensure that media is ready, you can called the "status" method in the media endpoint to get the status.
    ///
    /// for more details check facebook reference <https://developers.facebook.com/docs/instagram-api/reference/ig-user/media_publish#creating> or
    /// the general content reference    <https://developers.facebook.com/docs/instagram-api/guides/content-publishing>
    pub async fn publish_media(
        self,
        insta_container_id: String,
    ) -> Result<InstaMediaContainerId, ClientErr> {
        let self_data = self.clone();

        let base_url = self_data.base_url.replace("EDGE", "media_publish");
        let url = base_url.to_string()
            + "?creation_id="
            + &insta_container_id
            + "&access_token="
            + &self_data.access_token;

        let resp = HttpConnection::post(url, "".to_string()).await?;
        Ok(resp)
    }
}
