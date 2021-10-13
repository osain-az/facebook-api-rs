use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
// rename it feed, and move it under page folder
/// This
/// ! https://developers.facebook.com/docs/graph-api/reference/v12.0/page/feed

#[derive(Deserialize, Debug, Serialize)]
pub struct Fields {
    fields: Vec<String>,
}

impl Default for Fields {
    fn default() -> Self {
        let field_list = vec![
            "from",
            "id",
            "message_tags",
            "story",
            "story_tags",
            "permalink_url",
            "message",
            "shares",
            "comments",
            "likes",
            "reactions",
        ];
        let fields = field_list.iter().map(|&field| field.into()).collect();
        Self { fields }
    }
}

// This will be in another files
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct GetPostResponse {
    pub id: String,
    pub message: String,
    pub from: From,
    pub permalink_url: String,
}

#[derive(Deserialize, Debug, Default, Serialize)]
pub struct From {
    pub id: String,
    pub name: String,
}

pub struct FeedApi {
    base_url: String,
    access_token: String,
}

impl FeedApi {
    pub fn new(base_url: String, access_token: String) -> FeedApi {
        FeedApi {
            base_url,
            access_token,
        }
    }

    pub async fn get(self) -> seed::fetch::Result<GetPostResponse> {
        let mut url = self.base_url.replace("EDGE", "?fields=");

        let field_count = Fields::default().fields.len();
        for (count, field) in Fields::default().fields.into_iter().enumerate() {
            if count < field_count - 1 {
                url = url + &field + ",";
            } else {
                url = url + &field; // remove the comma in the last filed
            }
        }

        let base_url = url + "&access_token=" + &self.access_token;
        let request = Request::new(base_url).method(Method::Get);
        fetch(request).await?.json::<GetPostResponse>().await
    }
}

//  https://developers.facebook.com/docs/graph-api/reference/post#updating

// move it under page module
/// Struct that will hold different data for the making a feed request which are
/// upadted from the client method
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct PostApi {
    base_url: String,
    access_token: String,
}

/// Return response for posting feeds ( message or link) to the page, the
/// response id is tthe combination of page_post_id
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct FeedPostSuccess {
    pub id: String,
}

/// Return response for posting feeds ( picture or video)
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct MediaPostSuccess {
    id: String,
    post_id: String,
}

/// This struct i sthe

pub struct PostFields {
    link: String,
    call_to_action: CallToAction,
    action: CallToActionList,
}
pub enum CallToActionList {
    // Determines the call to action button text. One of allowed values:
    BOOK_TRAVEL,
    /// . Call to action shows up as Book Now.
    BUY_NOW,
    /// Call to action shows up as Buy Now. Only used for desktop app ads for
    /// virtual goods.
    CALL_NOW,
    /// Call to action shows up as Call Now. Only used for local awareness ads.
    DOWNLOAD,
    ///  Call to action shows up as Download.
    GET_DIRECTIONS,
    /// Call to action shows up as Get Directions. Must specify coordinates on
    /// the link field. Only used for local awareness ads.
    GET_QUOTE,
    /// Call to action shows up as Get Quote for lead generation.
    INSTALL_APP,
    /// Call to action shows up as Install Now.
    INSTALL_MOBILE_APP, // Call to action shows up as Install Now. Only used for mobile app ads.

    LEARN_MORE, // Call to action shows up as Learn More.

    LIKE_PAGE, // Call to action shows up as Like Page. Only used for ads in Page Likes objective.

    LISTEN_MUSIC, // Call to action shows up as Listen Music.

    MESSAGE_PAGE, // Call to action shows up as Send Message. Only used for local awareness ads.

    NO_BUTTON, // No call to action shows up.

    OPEN_LINK, /* Call to action shows up as Open Link. Only used for ads in Website Clicks
                * objective. */

    PLAY_GAME, // Call to action shows up as Play Game. Only used for desktop app ads.

    SHOP_NOW,
    /// all to action shows up as Shop Now. Only used for ads in Website
    /// Conversions objective.
    SIGN_UP,
    /// Cl to action shows up as Sign Up.
    SUBSCRIBE, // Call to action shows up as Subscribe for lead generation.

    USE_APP,
    /// Cl to action shows up as Use App.
    USE_MOBIL, //_APP. Only used for mobile app ads.

    WATCH_MOR, //. Call to action shows up as Watch More.

    WATCH_VID, // O. Call to action shows up as Watch Video.
}
pub struct CallToAction {}

/// Return response for posting feeds ( picture or video) to a page or user
/// account feed
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

    /// Method used for posting to content to the account or page feed, this
    /// method can not feed media like  video and photo
    pub async fn feed_post(&self, message: &str) -> seed::fetch::Result<FeedPostSuccess> {
        let base_url = self.base_url.replace("EDGE", "feed");
        let url = base_url + "?message=" + message + "&access_token=" + &self.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<FeedPostSuccess>().await
    }

    // video upload by  link
    /// this Method is used for posting video hosted online (video url ) to the
    /// account or page feed.
    pub async fn post_by_link(&self, file_url: &str) -> seed::fetch::Result<VideoPostResponse> {
        let base_url = self.base_url.replace("EDGE", "videos");
        let url = base_url + "?file_url=" + file_url + "&access_token=" + &self.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<VideoPostResponse>().await
    }
}