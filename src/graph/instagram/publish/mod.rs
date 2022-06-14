//! This represent facebook (instagram) content API which hase endpoint for
//! posting and publishing of media (video and photo). POST /{ig-user-id}/media
//! — upload media and create media containers. POST /{ig-user-id}/media_publish
//! — publish the uploaded media using their media containers. For details of
//! the API endpoints and it requirements refer to facebook documentation.  <https://developers.facebook.com/docs/instagram-api/guides/content-publishing>
//!
//! Limitations
//! Can only be used to publish to business IG User accounts; Creator IG User
//! accounts are not supported. Accounts are limited to 25 API-published posts
//! within a 24 hour period. JPEG is the only image format supported. Extended
//! JPEG formats such as MPO and JPS are not supported. Stories are not
//! supported. Shopping tags are not supported.
//! Branded content tags are not supported.
//! Filters are not supported.
//! Multi-image posts are not supported.
//! If the caption contains a hashtag, it should be HTML URL-encoded as %23 in
//! the request. Publishing to Instagram TV is not supported.

use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

/// This struct present  possible data  for posting to instagram account,
/// only the url is required for posting.
#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstagramPostParams {
    pub url: String,
    pub caption: String,
    pub location_id: String,
    pub tag_users: Option<Vec<InstagramUser>>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct InstagramUser {
    pub username: String,
    pub x: f32,
    pub y: f32,
}

impl InstagramPostParams {
    /// This method let developer update the feed parameters by keeping tract of
    /// each  inputted values
    pub fn new(
        url: String,
        caption: String,
        location_id: String,
        tag_users: Option<Vec<InstagramUser>>,
    ) -> Self {
        InstagramPostParams {
            url,
            caption,
            location_id,
            tag_users,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstagramMediaContainerId {
    pub id: String,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct InstagramPostApi {
    pub access_token: String,
    pub base_url: String,
}

impl InstagramPostApi {
    pub fn new(access_token: String, base_url: String) -> InstagramPostApi {
        InstagramPostApi {
            access_token,
            base_url,
        }
    }

    /// Post video to instagram media container.
    ///
    /// A successful post will return a container ID which can be use
    /// in the publish_media method to publish the media.
    /// Note: the media will not be visible until the publish_media method is
    /// called with container id
    ///
    /// If the media container is not published after 24 hour, the media will be
    /// deleted.
    ///
    /// After posting the video and the media_id is returned in the response,
    /// the video will not be ready to be published immediately, therefore
    /// the developer must ensure that the publish_method is wait for 30 to 60
    /// seconds before sending the published request.
    ///
    /// Alternatively, to ensure that media is ready, you can call the "status"
    /// method in the media endpoint to get the status.
    ///
    /// # Limitation
    ///
    /// * Containers expire after 24 hours.
    /// * If the Page connected to the targeted Instagram Business account
    ///   requires Page Publishing Authorization (PPA), PPA must be completed or
    ///   the request will fail.
    /// I* f the Page connected to the targeted Instagram Business account
    /// requires two-factor authentication, the Facebook User must also have
    /// performed two-factor authentication or the request will fail.
    /// * Publishing to Instagram TV is not supported.
    ///
    /// # Video Specification
    ///
    /// * Container: MOV or MP4 (MPEG-4 Part 14), no edit lists, moov atom at
    ///   the front of the file.
    ///  *Audio codec: AAC, 48khz sample rate maximum, 1 or 2 channels (mono or
    /// stereo).  *Video codec: HEVC or H264, progressive scan, closed GOP,
    /// 4:2:0 chroma subsampling.
    ///  * Frame rate: 23-60 FPS.
    ///  *Picture size:
    ///  * Maximum columns (horizontal pixels): 1920
    ///  * Minimum aspect ratio [cols / rows]: 4 / 5
    ///  *Maximum aspect ratio [cols / rows]: 16 / 9
    ///  * Video bitrate: VBR, 5Mbps maximum
    ///  *Audio bitrate: 128kbps
    ///  * Duration: 60 seconds maximum, 3 seconds minimum
    ///  *File size: 100MB maximum
    /// [Facebook doc](https://developers.facebook.com/docs/instagram-api/reference/ig-user/media#creating)

    pub async fn post_video(
        self,
        post_params: InstagramPostParams,
        video_url: String,
        post_caption: String,
        location_page_id: Option<String>,
        tag_users: Option<Vec<InstagramUser>>,
    ) -> Result<InstagramMediaContainerId, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "media");
        let caption = encode(&post_caption);

        let mut url = base_url
            + "?media_type=VIDEO"
            + "&video_url="
            + &video_url
            + "&caption="
            + &*caption
            + "&access_token="
            + &self.access_token;

        if let Some(_location_page_id) = location_page_id {
            url = url + "location_id=" + &*_location_page_id
        };

        if let Some(users) = tag_users {
            let tags = format!("{:?}", users);
            url = url + "&user_tags=" + tags.as_str();
        };

        let resp =
            HttpConnection::post::<InstagramMediaContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// The post_media method is used to publish media and create media
    /// containers of video, image or carouse.
    ///
    /// media parameter is posted to the IG
    /// container, then a container ID is sent back that which can be used
    /// in the  publish_ media method  to published the media. Note: your
    /// media will not be visible until the publish_media method is called.
    /// If the publish_media is not called until 24 hour, the media will be
    /// deleted.
    ///
    /// After posting the media and the media_id is returned in the response,
    /// the video will not be ready to be published immediately, therefore
    /// the developer must ensure that the publish_method is wait for 30 to 60
    /// seconds before sending the published request. Alternatively, to
    /// ensure that media is ready, you can called the "status" method in the
    /// media endpoint to get the status.
    ///
    /// Limitation
    ///
    /// Video:
    ///      Container: MOV or MP4 (MPEG-4 Part 14), no edit lists, moov atom at
    /// the front of the file.      publAudio codec: AAC, 48khz sample rate
    /// maximum, 1 or 2 channels (mono or stereo).ish_video " method. This
    /// method is expecting a  InstaPostParams      Video codec: HEVC or
    /// H264, progressive scan, closed GOP, 4:2:0 chroma subsampling.
    ///      Duration: 60 seconds maximum, 3 seconds minimum.
    ///      File size: 100MB maximum.
    ///      Publishing to Instagram TV is not supported.
    ///
    /// Photo:
    ///      Formats: JPEG .
    ///       Maximum file size: 8MiB
    ///       Aspect ratio: Must be within a 4:5 to 1.91:1 range
    ///       Minimum width: 320 (will be scaled up to the minimum if necessary)
    ///       Maximum width: 1440 (will be scaled down to the maximum if
    /// necessary)
    ///
    /// for more details check facebook reference <https://developers.facebook.com/docs/instagram-api/reference/ig-user/media#creating> or
    /// the general content reference    <https://developers.facebook.com/docs/instagram-api/guides/content-publishing>
    pub async fn post_image(
        self,
        image_url: String,
        post_caption: String,
        location_page_id: Option<String>,
        tag_users: Option<Vec<InstagramUser>>,
    ) -> Result<InstagramMediaContainerId, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "media");
        let caption = encode(&post_caption).to_string();
        let mut url = base_url
            + "?image_url="
            + &image_url
            + "&caption="
            + &*caption
            + "&access_token="
            + &self.access_token;

        if let Some(_location_page_id) = location_page_id {
            url = url + "location_id=" + &*_location_page_id
        };

        if let Some(users) = tag_users {
            let tags = format!("{:?}", users);
            url = url + "&user_tags=" + tags.as_str();
        };

        let resp =
            HttpConnection::post::<InstagramMediaContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    pub async fn post_carousels(
        self,
        image_url: String,
        post_caption: String,
        media_container_ids: Vec<String>,
        location_page_id: Option<String>,
        tag_users: Option<Vec<InstagramUser>>,
    ) -> Result<InstagramMediaContainerId, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "media");
        let caption = encode(&post_caption);
        let children = format!("{:?}", media_container_ids);
        let mut url = base_url
            + "?image_url="
            + &image_url
            + "children="
            + &children
            + "&caption="
            + &*caption
            + "&access_token="
            + &self.access_token;

        if let Some(_location_page_id) = location_page_id {
            url = url + "location_id=" + &*_location_page_id
        };

        if let Some(users) = tag_users {
            let tags = format!("{:?}", users);
            url = url + "&user_tags=" + tags.as_str();
        };

        let resp =
            HttpConnection::post::<InstagramMediaContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// This  should be used when the container id of the feed is ready, this
    /// method will published the media that has been posted,
    /// It only accept the  container id of the posted media you want to
    /// published.
    ///
    /// After posting the media and the media_id is returned in the response,
    /// the video will not be ready to be published immediately, therefore
    /// the developer must ensure that the publish_method is wait for 30 to 60
    /// seconds before sending the published request. Alternatively, to
    /// ensure that media is ready, you can called the "status" method in the
    /// media endpoint to get the status.
    ///
    /// for more details check facebook reference <https://developers.facebook.com/docs/instagram-api/reference/ig-user/media_publish#creating> or
    /// the general content reference    <https://developers.facebook.com/docs/instagram-api/guides/content-publishing>
    pub async fn publish_media(
        self,
        insta_container_id: String,
    ) -> Result<InstagramMediaContainerId, ClientErr> {
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
