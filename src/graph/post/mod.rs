use seed::fetch::fetch;
use seed::prelude::{Method, Request};

//use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

/// Struct that will hold different data for the making a post request
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct PostApi {
    base_url: String,
    access_token: String,
}

/// Return response for posting feeds ( message or link)
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct FeedPostSuccess {
    pub id: String,
}

impl FeedPostSuccess {
    pub fn get_response_id(self) -> String {
        self.id
    }
}

pub struct PostNodes {}

/// Return response for posting feeds ( picture or video)
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct MediaPostSuccess {
    id: String,
    post_id: String,
}

impl PostApi {
    pub fn new(base_url: String, access_token: String) -> PostApi {
        PostApi {
            base_url,
            access_token,
        }
    }
    pub async fn feed_post(&self, message: &str) -> seed::fetch::Result<FeedPostSuccess> {
        let base_url = self.base_url.replace("EDGE", "feed");
        let url = base_url + "?message=" + message + "&access_token=" + &self.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<FeedPostSuccess>().await
    }
}
