//! Representation of Instagram api for Photo, Video, Story, Album, or
//! Instagram TV media. Reels are not supported.
//!
//! It allow´ you to get media details ( comments, like, etc). This endpoint
//! does not allow creation of either of the media type.
//!
//! For more information check [Facebook doc](https://developers.facebook.com/docs/instagram-api/reference/ig-media).

use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize, Clone)]
pub struct InstagramMediaApi {
    access_token: String,
    base_url: String,
}

impl InstagramMediaApi {
    pub fn new(access_token: String, base_url: String) -> InstagramMediaApi {
        InstagramMediaApi {
            access_token,
            base_url,
        }
    }

    /// This method allow´s you to post a comment on a give media container.
    pub async fn post_comments(
        self,
        comment_message: String,
    ) -> Result<InstaMediaContainerId, ClientErr> {
        let self_data = self.clone();
        let base_url = self.base_url.replace("EDGE", "comments");
        let comment = comment_message;
        let message = encode(&comment);
        let url = base_url.to_string()
            + "?message="
            + (&message).as_ref()
            + "&access_token="
            + &self_data.access_token;

        let resp =
            HttpConnection::post::<InstaMediaContainerId, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// Get information regarding a given media.
    ///
    /// It returns the information about the media with response of
    /// [MediaContainerData](MediaContainerData)
    pub async fn media_data(self) -> Result<MediaContainerData, ClientErr> {
        let mut url = self.base_url.replace("EDGE", "?fields=");

        let mut fields_count = Fields::default().fields.len();
        for (count, field) in Fields::default().fields.into_iter().enumerate() {
            if count < fields_count - 1 {
                url = url + &field + ",";
            } else {
                url = url + &field; // remove the comma in the last filed
            }
        }
        url = url + "&access_token=" + &self.access_token;

        let resp = HttpConnection::get::<MediaContainerData>(url, "".to_string()).await?;
        Ok(resp)
    }
    // This method allows you to check the status for a given media.
    // pub async fn status(self) -> Result<MediaContainerStatus, ClientErr> {
    // let base_url = self.base_url.replace("EDGE", "?fields=status_code");
    // let url = base_url + "&access_token=" + &self.access_token;
    //
    // let resp = HttpConnection::get::<MediaContainerStatus>(url,
    // "".to_string()).await?; Ok(resp)
    // }
}
// #[derive(Deserialize, Debug, Clone)]
// pub struct MediaContainerStatus {
// pub status_code: String,
// }

/// ```
/// use facebook_api_rs::prelude::Owner;
/// pub struct MediaContainerData {
///     /// Media type. Can be CAROUSEL_ALBUM, IMAGE, or VIDEO.
///    pub media_type: String,
///    pub media_url: String,
///    pub owner: Owner,
///    pub timestamp: String,
///    pub username: String,
///    pub id: String,
///    pub permalink: String,
///     /// Count of likes on the media, including replies on comments.
///     /// Excludes likes on album child media and likes on promoted posts created
///     /// from the media.
///     ///
///     /// If the owner has hide this field then it will be set to None
///    pub like_count: Option<u32>,
///    pub thumbnail_url: String,
///    pub is_comment_enabled: String,
///   pub  comments_count: String,
///     /// Caption. Excludes album children. The @ symbol is excluded, unless the
///     /// app user can perform admin-equivalent tasks on the Facebook Page
///     /// connected to the Instagram account used to create the caption.
///    pub caption: String,
/// }
#[derive(Deserialize, Debug, Clone)]
pub struct MediaContainerData {
    /// Media type. Can be CAROUSEL_ALBUM, IMAGE, or VIDEO.
    pub media_type: String,
    pub media_url: String,
    pub owner: Owner,
    pub timestamp: String,
    pub username: String,
    pub id: String,
    pub permalink: String,
    /// Count of likes on the media, including replies on comments.
    /// Excludes likes on album child media and likes on promoted posts created
    /// from the media.
    ///
    /// If the owner has hide this field then it will be set to None
    pub like_count: Option<u32>,
    pub thumbnail_url: String,
    pub is_comment_enabled: String,
    pub comments_count: String,
    /// Caption. Excludes album children. The @ symbol is excluded, unless the
    /// app user can perform admin-equivalent tasks on the Facebook Page
    /// connected to the Instagram account used to create the caption.
    pub caption: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstaMediaContainerId {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Owner {
    id: String,
}

pub struct Fields {
    pub(crate) fields: Vec<String>,
}

impl Default for Fields {
    /// This parameters are used as fields which are passed in as a query
    /// parameters to the get post request and feeds request
    fn default() -> Self {
        let field_list = vec![
            "caption",
            "id",
            "comments_count",
            "follows_count",
            "like_count",
            "media_product_type",
            "media_type",
            "media_url",
            "owner",
            "permalink",
            "thumbnail_url",
            "timestamp ",
            "username",
            "video_title",
        ];

        let fields = field_list.iter().map(|&field| field.into()).collect();
        Self { fields }
    }
}
