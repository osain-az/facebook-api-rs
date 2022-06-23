//! This represent facebook (instagram) content API which hase endpoint for
//! posting and publishing of media (video and photo).
//!
//! The major operations are:
//!
//! * Upload media and create media containers.
//! * Publish the uploaded media using the container id
//!
//! # Limitations
//! * Can only be used to publish to business IG User accounts; Creator IG User
//! * accounts are not supported. Accounts are limited to 25 API-published posts
//! within a 24 hour period.
//! *JPEG is the only image format supported. Extended JPEG formats such as MPO
//! and JPS are not supported. Stories are not supported.
//!
//! * Shopping tags are not supported.
//! * Branded content tags are not supported.
//! * Filters are not supported.
//! * Multi-image posts are not supported.
//! * Publishing to Instagram TV is not supported.
//!
//! For more information check [Facebook content doc](https://developers.facebook.com/docs/instagram-api/guides/content-publishing)
#![allow(dead_code, unused)]

use crate::prelude::errors::ClientErr;
use crate::prelude::structs::MediaType;
use crate::prelude::HttpConnection;
// use facebook_api_rs::prelude::search::PagesSearchAPI;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

//@Todo: remove this struct
#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstagramPostParams {
    pub url: String,
    pub caption: String,
    pub location_id: String,
    pub tag_users: Option<Vec<InstagramTagUser>>,
}

///  Instagram content api for uploading and publishing media.
///
/// # examples
///
/// # Uploading media
/// * For video upload use:
/// ```
/// use facebook_api_rs::prelude::{Client,UserToken};
/// use facebook_api_rs::prelude::Client;
///
/// // Check the method for doc and usage///
/// Client::new(UserToken::default(), "facebook_page_token".to_owned())
///     .instagram_content_publishing("instagram_id".to_owned()).upload_video()
/// ```
/// For image upload use:
/// ```
/// use facebook_api_rs::prelude::{Client,UserToken};
/// use facebook_api_rs::prelude::Client;
///
/// // Check the method for doc and usage.
///  Client::new(UserToken::default(), "facebook_page_token".to_owned())
///     .instagram_content_publishing("instagram_id".to_owned()).upload_image()
/// ```
/// For carousels upload use see below
///
/// # Uploading media for CAROUSES
///
/// You may publish up to 10 images, videos, or a mix of the two in a single
/// post (a carousel post). Publishing carousels is a three step process:
///
///  * carousels first process:
///
/// ```
///  use facebook_api_rs::prelude::{Client,UserToken};
///
/// // Check the method for doc and usage.
///  let carousels_id =  Client::new(UserToken::default(), "facebook_page_token".to_owned())
///     .instagram_content_publishing("instagram_id".to_owned()).upload_carousel_item("".to_string(), MediaType::Video, None, None).await?;
/// ```
///
///  * carousels secod process:
/// ```
///  use facebook_api_rs::prelude::{Client,UserToken};
///
/// // Check the method for doc and usage.
///  let carousels_id =  Client::new(UserToken::default(), "facebook_page_token".to_owned())
///     .instagram_content_publishing("instagram_id".to_owned()).carousels_container("".to_string(), vec![]).await?;
/// ```
///
///  * carousels third process:
///
///  The third process is thesame as the publishing method shown below.
///
///  # Publishing media
/// ```
///  use facebook_api_rs::prelude::{Client,UserToken};
/// use facebook_api_rs::prelude::Client;
///
/// // Check the method for doc and usage.
///  Client::new(UserToken::default(), "facebook_page_token".to_owned())
///     .instagram_content_publishing("instagram_id".to_owned()).publish_container()
/// ```
#[derive(Deserialize, Clone, Serialize)]
pub struct InstagramContentPublishingApi {
    pub access_token: String,
    pub base_url: String,
}

impl InstagramContentPublishingApi {
    pub fn new(access_token: String, base_url: String) -> InstagramContentPublishingApi {
        InstagramContentPublishingApi {
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
    /// # Video Specification:
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
    ///
    /// Video uploads are asynchronous, so receiving a container ID does not
    /// guarantee that the upload was successful. To verify that a video has
    /// been uploaded, request the status_code field on the IG Container. If
    /// its value is FINISHED, the video was uploaded successfully.
    ///
    /// [Facebook doc](https://developers.facebook.com/docs/instagram-api/reference/ig-user/media#creating)
    ///
    /// # Argument
    ///
    /// * `video_url` - A string of a hosted video.
    /// Path to the video. Facebbook cURL the video using the passed-in URL, so
    /// it must be on a public server
    ///
    /// * `post_caption` - A string of description of the post.
    /// A caption for the image, video, or carousel. Can include hashtags
    /// (example: #crazywildebeest) and usernames of Instagram users (example:
    /// @natgeo). @Mentioned Instagram users receive a notification when the
    /// container is published. Maximum 2200 characters, 30 hashtags, and 20 @
    /// tags.
    ///
    /// * `location_page_id` - The ID of a Page associated with a location that
    ///   you want to tag the image or video with.
    ///
    /// To get the id check [PagesSearchAPI](PagesSearchAPI) endpoint for more
    /// information.
    ///
    /// # Example
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken, InstagramPostParams, InstagramContainerId, ContainerStatusCodeList, InstagramMediaId};
    /// use facebook_api_rs::prelude::errors::ClientErr;
    /// use facebook_api_rs::prelude::search::PagesSearchAPI;
    /// let caption = "The best part of nature.  #Nature #NatureAtWork picture taken by @instagramUser".to_owned();
    /// // We dont want to tag any location
    /// let location_page_id = None;
    ///
    /// let instagram_client = Client::new(UserToken::default(), "facebook_page_token".to_owned())
    ///      .instagram_content_publishing("instagram_id".to_owned());
    ///
    /// //post video to a container and return the container id.
    /// let container_id : InstagramContainerId = instagram_client.clone()
    ///  .upload_video("video_url".to_string(), caption, location_page_id).await?;
    ///
    /// // since video uploading is asynchronous, you must check to see if the video upload was successful.
    /// // THis will take about 30 to 60 second to finish uploading. So is recommended to wait like 40 seconds before checking the status.  
    ///  let container_status = instagram_client.status(container_id.id.clone()).await?;
    ///  let status_code = container_status.status_code;
    ///
    ///  match container_status.status_code{
    ///   ContainerStatusCodeList::ERROR => {
    ///  // check the error message and handle error
    ///     let error_message = container_status.status.error;
    /// }
    ///   ContainerStatusCodeList::IN_PROGRESS =>{
    /// // wait for more seconds and check the status again
    /// }
    ///   ContainerStatusCodeList::PUBLISHED => {}
    /// ContainerStatusCodeList::EXPIRED => {
    ///  // container has expired, This happned after 24 hours of the container been published
    /// }
    ///   ContainerStatusCodeList::FINISHED => {
    /// // The container is ready to be published
    ///  let media_id : InstagramMediaId = instagram_client.clone().publish_container(container_id.id).await?;
    /// }
    /// }
    /// ```
    /// For more information check [Facebook doc](https://developers.facebook.com/docs/instagram-api/reference/ig-user/media#creating)
    pub async fn upload_video(
        self,
        video_url: String,
        post_caption: String,
        location_page_id: Option<String>,
    ) -> Result<InstagramContainerId, ClientErr> {
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

        let resp =
            HttpConnection::post::<InstagramContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// The post_image method is used to publish media and create media
    /// containers of image.
    ///
    /// Where the container ID is sent back which can be used
    /// in the publish_ media method to publish the media.
    ///
    /// Note: your media will not be visible until the publish_media method is
    /// called. If the publish_media is not called until 24 hour, the media
    /// will be deleted.
    ///
    /// # Photo Specification
    ///  - Formats: JPEG .
    ///  - Maximum file size: 8MiB
    ///  - Aspect ratio: Must be within a 4:5 to 1.91:1 range
    ///  - Minimum width: 320 (will be scaled up to the minimum if necessary)
    ///  - Maximum width: 1440 (will be scaled down to the maximum if
    /// necessary)
    ///
    ///
    /// # Argument
    ///
    /// * `image_url` - A string of a hosted video.
    /// Path to the video. Facebbook cURL the video using the passed-in URL, so
    /// it must be on a public server
    ///
    /// * `post_caption` - A string of description of the post.
    /// A caption for the image, video, or carousel. Can include hashtags
    /// (example: #crazywildebeest) and usernames of Instagram users (example:
    /// @natgeo). @Mentioned Instagram users receive a notification when the
    /// container is published. Maximum 2200 characters, 30 hashtags, and 20 @
    /// tags.
    ///
    /// * `location_page_id` - The ID of a Page associated with a location that
    ///   you want to tag the image or video with.
    ///
    /// To get the id check [PagesSearchAPI](PagesSearchAPI) endpoint for more
    /// information.
    ///
    /// * `users_tag` - A vector struct of instagram user
    ///   [InstagramUserMention](InstagramUserMention) that are
    ///  array of public usernames and x/y coordinates for any public Instagram
    /// users who you want to tag in the image.
    ///
    /// The array must contain a username, x, and y, property, such as
    /// [{username:'natgeo',x:0.5,y:1.0}]. x and y values must be float
    /// numbers that originate from the top-left of the image, with a range of
    /// 0.0–1.0. Tagged users receive a notification when you publish the
    /// media container.
    ///
    /// # Example
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken, InstagramPostParams, InstagramContainerId, InstagramTagUser, InstagramMediaId};
    /// use facebook_api_rs::prelude::errors::ClientErr;
    /// use facebook_api_rs::prelude::search::PagesSearchAPI;
    /// let caption = "The best part of nature.  #Nature #NatureAtWork picture taken by @instagramUser".to_owned();
    ///
    /// // We dont want to tag any location
    /// let location_page_id = None;
    /// let mut tag_users : Vec<InstagramTagUser> = Vec::new();
    ///  
    /// let user = InstagramTagUser{username: "username".to_string(), x: 0.4, y: 0.5};
    ///    tag_users.push(user);
    ///
    /// let instagram_client = Client::new(UserToken::default(), "facebook_page_token".to_owned())
    ///      .instagram_content_publishing("instagram_id".to_owned());
    ///
    ///    //Post the media to a container and return the container id
    /// let container_id : InstagramContainerId = instagram_client.clone()
    ///  .upload_image("image_url".to_string(), caption, location_page_id, Some(tag_users)).await? ;
    ///
    ///  //To publish the posted image, make a publish request with the container id.
    /// let media_id : InstagramMediaId = instagram_client.publish_container(container_id.id).await?;
    /// ```
    /// For more information check [Facebook doc](https://developers.facebook.com/docs/instagram-api/guides/content-publishing)
    pub async fn upload_image(
        self,
        image_url: String,
        post_caption: String,
        location_page_id: Option<String>,
        tag_users: Option<Vec<InstagramTagUser>>,
    ) -> Result<InstagramContainerId, ClientErr> {
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
            HttpConnection::post::<InstagramContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// Carousels are post with combination of image and videos.
    ///
    /// To create a carousel three steps are involved
    ///
    /// This is the first step
    ///
    /// Upload each individual media (video or image) to a Carousels container
    /// and use their containers' id to create a carousel.
    ///
    /// # Argument
    ///
    /// * `media_url` - A String of the media (video or image).
    /// * `media_type` - An enum of [MediaType](MediaType) which indicate either
    ///   video or image
    /// * `caption` - A string of description of the post.
    /// A caption for the image, video, or carousel. Can include hashtags
    /// (example: #crazywildebeest) and usernames of Instagram users (example:
    /// @natgeo). @Mentioned Instagram users receive a notification when the
    /// container is published. Maximum 2200 characters, 30 hashtags, and 20 @
    /// tags.
    ///
    /// * `location_page_id` - The ID of a Page associated with a location that
    ///   you want to tag the image or video with.
    ///
    /// To get the id check [PagesSearchAPI](PagesSearchAPI) endpoint for more
    /// information.
    ///
    /// * `users_tag` - A vector struct of instagram user
    ///   [InstagramUserMention](InstagramUserMention) that are
    ///  array of public usernames and x/y coordinates for any public Instagram
    /// users who you want to tag in the image.
    ///
    /// # example
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken, InstagramPostParams, InstagramContainerId, InstagramTagUser, InstagramMediaId};
    /// use facebook_api_rs::prelude::errors::ClientErr;
    /// use facebook_api_rs::prelude::search::PagesSearchAPI;
    /// use facebook_api_rs::prelude::structs::MediaType;
    ///
    /// let location_page_id = "xxxxxxxxxxx".to_owned(); //or None
    /// let mut tag_users : Vec<InstagramTagUser> = Vec::new();
    ///  
    /// let user = InstagramTagUser{username: "username".to_string(), x: 0.4, y: 0.5};
    ///    tag_users.push(user);
    ///
    ///  let media_url ="url".to_owned(); // video or image
    ///  let media_type = MediaType::Video; //if uploading video
    ///
    ///  // create a single client
    /// let instagram_client = Client::new(UserToken::default(), "facebook_page_token".to_owned())
    ///      .instagram_content_publishing("instagram_id".to_owned());
    ///
    ///  //Upload the media you intend to be used in Carousels post, a successful upload will return an id that can be used in the next step.
    ///  // Do this for all your item you want to include in Carousels.
    ///  let mut carousels_items : Vec<String>= Vec::new();
    ///
    /// let carousels_item_id : InstagramContainerId = instagram_client.clone()
    ///  .upload_carousel_item(media_url, media_type, Some(location_page_id), Some(tag_users)).await? ;
    ///   
    ///  carousels_items.push(carousels_item_id.id);
    ///
    ///  // Create the Carousels container with all the carousels_item_ids created above.
    /// let caption = "The best part of nature.  #Nature #NatureAtWork picture taken by @instagramUser".to_owned();
    ///
    /// let carousels_id : InstagramContainerId = instagram_client.carousels_container(caption, carousels_items).await?;
    ///
    /// // Finally, publish the Carousels container
    /// let media_id : InstagramMediaId = instagram_client.publish_container(carousels_id.id).await?;
    /// ```
    pub async fn upload_carousel_item(
        self,
        media_url: String,
        media_type: MediaType,
        location_page_id: Option<String>,
        tag_users: Option<Vec<InstagramTagUser>>,
    ) -> Result<InstagramContainerId, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "media");

        let mut url = base_url + "&is_carousel_item=true" + "&access_token=" + &self.access_token;

        let media_type = match media_type {
            MediaType::Video => url = url + "video_url=" + &media_url + "media_type=VIDEO",
            MediaType::Image => url = url + "image_url=" + &media_url,
        };

        if let Some(_location_page_id) = location_page_id {
            url = url + "location_id=" + &*_location_page_id
        };

        if let Some(users) = tag_users {
            let tags = format!("{:?}", users);
            url = url + "&user_tags=" + tags.as_str();
        };

        let resp =
            HttpConnection::post::<InstagramContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// Carousels are post with combination of image and videos.
    ///
    /// You may publish up to 10 images, videos, or a mix of the two in a
    /// single. The media you intend to create carousel should have already been
    /// uploaded with the first step.
    ///
    /// To create a carousel three steps are involved
    ///
    /// This is the second step
    ///
    /// # Argument
    ///
    /// * `post_caption` - A string of description of the post.
    /// A caption for the image, video, or carousel. Can include hashtags
    /// (example: #crazywildebeest) and usernames of Instagram users (example:
    /// @natgeo). @Mentioned Instagram users receive a notification when the
    /// container is published. Maximum 2200 characters, 30 hashtags, and 20 @
    /// tags.
    ///
    /// * `container_ids` - an array of String of previous uploaded medias
    ///   container id which was uploaded using the first step in the carousels
    ///   uploading methods
    ///
    /// # example
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken, InstagramPostParams, InstagramContainerId, InstagramTagUser, InstagramMediaId};
    /// use facebook_api_rs::prelude::errors::ClientErr;
    /// use facebook_api_rs::prelude::search::PagesSearchAPI;
    /// use facebook_api_rs::prelude::structs::MediaType;
    ///
    /// let location_page_id = "xxxxxxxxxxx".to_owned(); //or None
    /// let mut tag_users : Vec<InstagramTagUser> = Vec::new();
    ///  
    /// let user = InstagramTagUser{username: "username".to_string(), x: 0.4, y: 0.5};
    ///    tag_users.push(user);
    ///
    ///  let media_url ="url".to_owned(); // video or image
    ///  let media_type = MediaType::Video; //if uploading video
    ///
    ///  // create a single client
    /// let instagram_client = Client::new(UserToken::default(), "facebook_page_token".to_owned())
    ///      .instagram_content_publishing("instagram_id".to_owned());
    ///
    ///  //Upload the media you intend to be used in Carousels post, a successful upload will return an id that can be used in the next step.
    ///  // Do this for all your item you want to include in Carousels.
    ///  let mut carousels_items : Vec<String>= Vec::new();
    ///
    /// let carousels_item_id : InstagramContainerId = instagram_client.clone()
    ///  .upload_carousel_item(media_url, media_type, Some(location_page_id), Some(tag_users)).await? ;
    ///   
    ///  carousels_items.push(carousels_item_id.id);
    ///
    ///  // Create the Carousels container with all the carousels_item_ids created above.
    /// let caption = "The best part of nature.  #Nature #NatureAtWork picture taken by @instagramUser".to_owned();
    ///
    /// let carousels_id : InstagramContainerId = instagram_client.carousels_container(caption, carousels_items).await?;
    ///
    /// // Finally, publish the Carousels container
    /// let media_id : InstagramMediaId = instagram_client.publish_container(carousels_id.id).await?;
    /// ```
    pub async fn carousels_container(
        self,
        post_caption: String,
        container_ids: Vec<String>,
    ) -> Result<InstagramContainerId, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "media");
        let caption = encode(&post_caption);
        let children = format!("{:?}", container_ids);
        let mut url = base_url
            + "?media_type=CAROUSEL"
            + "children="
            + &children
            + "&caption="
            + &*caption
            + "&access_token="
            + &self.access_token;

        let resp =
            HttpConnection::post::<InstagramContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// This method will publish the container that has been posted.
    ///
    /// # Argument
    ///
    /// * `container_id` - A string of container id has been posted.
    ///
    ///  If the container was created more than 24 hours ago then the response
    /// will be [ClientErr](ClientErr) which will contain the error message.
    pub async fn publish_container(
        self,
        container_id: String,
    ) -> Result<InstagramMediaId, ClientErr> {
        let self_data = self.clone();

        let base_url = self_data.base_url.replace("EDGE", "media_publish");
        let url = base_url.to_string()
            + "?creation_id="
            + &container_id
            + "&access_token="
            + &self_data.access_token;

        let resp = HttpConnection::post(url, "".to_string()).await?;
        Ok(resp)
    }

    /// This method allows you to check the status for a given media.
    pub async fn status(self, container_id: String) -> Result<ContainerStatus, ClientErr> {
        // The user_id in the self.base_url is supposed to be replaced by the container
        // id. The easiest way is to form a new  url.
        let base_url = format!(
            "https://graph.facebook.com/{}?fields=status_code,status,id",
            container_id
        );

        let url = base_url + "&access_token=" + &self.access_token;

        let resp = HttpConnection::get::<ContainerStatus>(url, "".to_string()).await?;
        Ok(resp)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContainerStatus {
    pub id: String,
    pub status_code: ContainerStatusCodeList,
    pub status: ContainerErrorStatus,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContainerErrorStatus {
    pub error: ContainerError,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContainerError {
    pub message: String,
    pub code: u32,
    pub error_subcode: u32,
    pub is_transient: bool,
    pub error_user_title: String,
    pub error_user_msg: String,
    pub fbtrace_id: String,
}

/// The container's publishing status. The Possible values:
///
/// - EXPIRED — The container was not published within 24 hours and has expired.
/// - ERROR — The container failed to complete the publishing process.
/// - FINISHED — The container and its media object are ready to be published.
/// - IN_PROGRESS — The container is still in the publishing process.
/// - PUBLISHED — The container's media object has been published.
#[derive(Deserialize, Clone, Debug, Serialize)]
pub enum ContainerStatusCodeList {
    /// The container was not published within 24 hours and has expired.
    EXPIRED,
    /// The container failed to complete the publishing process.
    ERROR,
    /// The container and its media object are ready to be published.
    FINISHED,
    /// The container is still in the publishing process.
    IN_PROGRESS,
    /// The container's media object has been published.
    PUBLISHED,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct InstagramTagUser {
    pub username: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstagramContainerId {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstagramMediaId {
    pub id: String,
}
