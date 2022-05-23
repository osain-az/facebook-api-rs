//! The feed API of Facebook Page let you published and get data of facebok page
//! the API have different end points depending on the operation you want to
//! perform  on the page. For more information on different endpoint for get and
//! publish post check facebook document <https://developers.facebook.com/docs/graph-api/reference/v12.0/page/feed#publish>.

use crate::graph::pages::utils::{Fields, GetPostResponse};
use crate::prelude::errors::ClientErr;
use crate::prelude::{Data, HttpConnection};
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
    ///  # Arguments
    /// * ` message` - This is body of the message you want to post.
    /// * ` url_link ` -  The link you intend to attach to the  post.
    pub async fn post(&self, post_params: FeedPostFields) -> Result<FeedPostSuccess, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "feed");
        let mut url = base_url
            + "?message="
            + &post_params.message
            + "&access_token="
            + &self.page_access_token;

        if post_params.call_to_action.is_some() {
            // let type_ = "type:".to_owned() + &post_params.call_to_action.unwrap().name;
            let type_ = "type:".to_owned()
                + format!("{:?}", &post_params.call_to_action.as_ref().unwrap().type_).as_str();
            let link = "Link:".to_owned() + &post_params.call_to_action.unwrap().link;
            let value = "value:{".to_owned() + &link + "}";
            let built_url = "{".to_owned() + &type_ + "," + &value + "}";
            url = "&call_to_action=".to_owned() + &built_url;
        };
        if !post_params.place.is_empty() {
            if !post_params.tags.is_empty() {
                let tag_list = build_tags(post_params.tags);
                url = url + "&place=" + &post_params.place + "&tags=" + &tag_list;
            } else {
                url = url + "&place=" + &post_params.place
            }
        }

        let resp = HttpConnection::get::<FeedPostSuccess>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// This method  return data  of the page feed
    pub async fn get(self) -> Result<FeedData, ClientErr> {
        let mut base_url = self.base_url.replace("EDGE", "feed");
        base_url = "?fields=".to_string();
        let field_count = Fields::default().fields.len();
        for (count, field) in Fields::default().fields.into_iter().enumerate() {
            if count < field_count - 1 {
                base_url = base_url + &field + ",";
            } else {
                base_url = base_url + &field; // remove the comma in the last
                                              // filed
            }
        }
        let url = base_url + "&access_token=" + &self.page_access_token;

        let resp = HttpConnection::get::<FeedData>(url, "".to_string()).await?;
        Ok(resp)
    }
}

/// Return response for posting feeds ( message or link) to the page, the
/// response id is the combination of page_id and post_id.
#[derive(Deserialize, Serialize, Clone)]
pub struct FeedPostSuccess {
    pub id: String,
}

/// Return response for posting feeds ( picture or video).
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct MediaPostSuccess {
    id: String,
    post_id: String,
}

/// The struct gotten  from page feed.
/// The data depends of the feeds.
#[derive(Deserialize, Debug, Serialize)]
pub struct FeedData {
    data: Data<GetPostResponse>,
}

pub struct FeedPostFields {
    /// The URL of a link to attach to the post. At lest either link or message
    /// must be supplied.
    pub link: String,

    /// The main body of the post. The message can contain mentions of Facebook
    /// Pages, @[page-id].
    pub message: String,

    /// Comma-separated list of user IDs of people tagged in this post.
    /// You cannot specify this field without also specifying a place.
    pub tags: Vec<String>,

    /// Page ID of a location associated with this post. Note the page must have
    /// lacation enable
    pub place: String,

    /// Object that specifies a Call to Action button. This should be the action
    /// you want people to take when they see your post. Clicking on this
    /// button will take people to the link you specify.
    pub call_to_action: Option<CallToAction>,

    /// Add a Feeling or Activity to a Page Post
    pub feeling: Option<Feeling>,
}

/// You can enhance your link Page posts with call to action buttons.
/// The following call_to_action field can be added to new link Page Posts.
///
/// Object that specifies a Call to Action button. This should be the action you
/// want people to take when they see your post. Clicking on this button will
/// take people to the link you specify.
#[derive(Debug)]
pub struct CallToAction {
    /// This link should be the same as the link parameter of the Page Post
    link: String,

    /// Determines the call to action button text. One of allowed values.
    ///
    /// example "BOOK_NOW, BUY_NOOW
    type_: CallToActionType,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(warnings)]
pub enum CallToActionType {
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

    /// Call to action shows up as Learn More.
    LEARN_MORE,

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

/// Add a feeling or activity and an icon to a page post.
///
/// og_action_type_id and og_object_id are required when posting a feeling or
/// activity. ```og_icon_id``` is optional however if not used an icon will be
/// automatically supplied based on the og_object_id.
pub struct Feeling {
    /// An action, i.e., feeling, watching, etc.
    /// Each feeling is represented by an id. The id can be found  <https://developers.facebook.com/docs/graph-api/reference/v13.0/page/feed/feelings#actions>
    og_action_type_id: String,

    /// An icon perhaps representing the action type, i.e., a smiley face, a
    /// movie icon, etc. Each feeling is represented by an id. The id can be found  <https://developers.facebook.com/docs/graph-api/reference/v13.0/page/feed/feelings#actions>
    og_icon_id: String,

    /// An object_id can be any page_id or a predefined object. Feelings
    ///    Each feeling is represented by an id. The id can be found  <https://developers.facebook.com/docs/graph-api/reference/v13.0/page/feed/feelings#actions>
    og_object_id: String,
}

fn build_tags(list: Vec<String>) -> String {
    let mut url = "".to_string();
    for value in list {
        url = format!("{}{},", url, value)
    }
    url
}
