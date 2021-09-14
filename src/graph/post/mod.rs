use seed::fetch::fetch;
use seed::prelude::{Method, Request};

//use seed::{prelude::*, *};
use crate::graph::video::ContentCategory;
use serde::{Deserialize, Serialize};

/// Struct that will hold different data for the making a post request which are upadted from the client method
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct PostApi {
    base_url: String,
    access_token: String,
}

/// Return response for posting feeds ( message or link) to the page, the response id is tthe combination of page_post_id
///
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct FeedPostSuccess {
    pub id: String,
}

/// Return response for posting feeds ( picture or video)
///
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct MediaPostSuccess {
    id: String,
    post_id: String,
}

///This struct i sthe

/*pub struct VideoParams {
    video_title: String,
    file_url: String,
    description: String,
    sponsor_relationship: String,
    scheduled_publish_time: String,
    replace_video_id: String,
    thum: String,
    content_category: ContentCategory,
}*/

/// Return response for posting feeds ( picture or video) to a page or user account feed
///
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct VideoPostResponse {
    pub id: String,
    pub video_id: String,
    pub success: bool,
}

impl PostApi {
    pub fn new(base_url: String, access_token: String) -> PostApi {
        PostApi {
            base_url,
            access_token,
        }
    }
    /// Method used for posting to content to the account or page feed, this method can not post media like  video and photo
    ///
    pub async fn feed_post(&self, message: &str) -> seed::fetch::Result<FeedPostSuccess> {
        let base_url = self.base_url.replace("EDGE", "feed");
        let url = base_url + "?message=" + message + "&access_token=" + &self.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<FeedPostSuccess>().await
    }

    // video upload by  link
    /// this Method is used for posting video hosted online (video url ) to the  account or page feed.
    ///
    pub async fn post_by_link(&self, file_url: &str) -> seed::fetch::Result<VideoPostResponse> {
        let base_url = self.base_url.replace("EDGE", "videos");
        let url = base_url + "?file_url=" + file_url + "&access_token=" + &self.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<VideoPostResponse>().await
    }
}
