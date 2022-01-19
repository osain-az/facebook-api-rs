//! The feed API of Facebook Page let you published and get data of facebok page
//! the API have different end points depending on the operation you want to
//! perform  on the page. For more information on different endpoint for get and
//! publish post check facebook document <https://developers.facebook.com/docs/graph-api/reference/v12.0/page/feed#publish>.

use crate::graph::pages::utils::{Fields, GetPostResponse};
use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};

/// The feed API of Facebook Page let you publish and get data of the page,
/// the API have different end points depending on the operation you want to
/// perform  on the page. For more information on different endpoint for get and
/// publish post check Facebook document https://developers.facebook.com/docs/graph-api/reference/v12.0/page/feed#publish.
pub struct FeedApi {
    base_url: String,
    page_access_token: String,
}

impl FeedApi {
    /// This is a static method used to create an instance of the feedApi
    /// Note: this method is called inside the Client.
    pub fn new(base_url: String, access_token: String) -> FeedApi {
        FeedApi {
            base_url,
            page_access_token: access_token,
        }
    }

    /// This Method is used for posting content to page feed,  you can
    /// publish to Pages by using this method to post either link or message
    /// this method can not post media like  video and photo.
    /// This method is expecting the post message you want to post on your feed.
    /// For more information check Facebook documentation  <https://developers.facebook.com/docs/graph-api/reference/page/feed/#publish>.
    pub async fn post(&self, message: &str) -> Result<FeedPostSuccess, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "feed");
        let url = base_url + "?message=" + message + "&access_token=" + &self.page_access_token;

        let resp = HttpConnection::get::<FeedPostSuccess>(url, "".to_string()).await?;
        Ok(resp)
    }
}

/// Return response for posting feeds ( message or link) to the page, the
/// response id is the combination of page_id and post_id.
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct FeedPostSuccess {
    pub id: String,
}

/// Return response for posting feeds ( picture or video).
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct MediaPostSuccess {
    id: String,
    post_id: String,
}

/// Todo: implement call to action method.
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
