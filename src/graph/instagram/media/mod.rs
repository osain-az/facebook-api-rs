//! This mode Represents Instagram api for  Photo, Video, Story, Album, or Instagram TV media. Reels are not supported.
//! It allow´ you to get media details ( comments, like, etc).
//! for details check <https://developers.facebook.com/docs/instagram-api/reference/ig-media>.

use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use urlencoding::encode;
use std::{thread, time};
use crate::graph::client::Client;

#[derive(Deserialize, Clone, Serialize)]
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
    /// this method allow´s you to post on a give media container.
    // for details check <https://developers.facebook.com/docs/instagram-api/reference/ig-media/comments>
    pub async fn post_comments(
        self,
        comment_message:String,
    ) -> seed::fetch::Result<InstaMediaContainerId> {
        let self_data = self.clone();
         let  base_url = self.base_url.replace("EDGE", "comments");
        let comment = comment_message ;
        let message = encode(&comment);
        let url = base_url.to_string()
            + "?message="
            + (&message).as_ref()
            + "&access_token="
            + &self_data.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<InstaMediaContainerId>().await
    }

    pub async fn data(self)  -> seed::fetch::Result<MediaContainerData>  {
        let mut url = self.base_url.replace("EDGE", "?fields=");

        let mut fields_count = Fields::default().fields.len();
        for (count, field) in Fields::default().fields.into_iter().enumerate() {
            if count < fields_count - 1 {
                url = url+ &field + ",";
            } else {
                url= url+ &field; // remove the comma in the last filed
            }
        }
        url = url
            + "&access_token="
            + &self.access_token;


        let request = Request::new(url).method(Method::Get);
        fetch(request).await?.json::<MediaContainerData>().await
    }

 /// This method allows you to check the status for a given media, this is important to check before
 /// calling the publish_media method.
 /// for details check <https://developers.facebook.com/docs/instagram-api/reference/ig-container#reading>
    pub async fn status(self)
                        -> seed::fetch::Result<MediaContainerStatus> {
        let base_url = self.base_url.replace("EDGE", "?fields=status_code");
        let url = base_url
            + "&access_token="
            + &self.access_token;

        let request = Request::new(url).method(Method::Get);
        fetch(request).await?.json::<MediaContainerStatus>().await
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct  MediaContainerStatus {
    pub      status_code:String
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct MediaContainerData {
    media_type:String,
    media_url: String,
    owner:  Owner,
    timestamp: String,
    username: String,
    permalink: String,
    like_count: String,
    comments_count: String,
    caption : String
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstaMediaContainerId {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct Owner {
    id: String
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
            "ig_id",
            "comments_count",
            "follows_count",
            "like_count",
            "media_product_type",
            "media_type",
            "media_url",
            "owner",
            "permalink",
            "thumbnail_url",
            "timestamp " ,
            "username",
            "video_title",
        ];

        let fields = field_list.iter().map(|&field| field.into()).collect();
        Self { fields }
    }
}