//!  This mod will serves as method binder that gives access different end
//! poinst availiable on the facebook-api.rs.

use crate::graph::me::MeApi;
use crate::graph::pages::feed::FeedApi;
use crate::graph::pages::post::PostApi;
use crate::login::token::{TokenLiveType, UserToken};
use crate::prelude::search::PagesSearchAPI;
use crate::prelude::video::VideoApi;
use crate::prelude::{
    BatchApi, HashtagAPi, InstagramApi, InstagramContentPublishingApi, InstagramMediaApi,
};

/// Default Facebook Graph API version
///
/// According to Facebook's versioning policy, each API version is supported for at least 2 years.
/// v23.0 was released in 2024 and will be supported until at least 2026.
///
/// See: <https://developers.facebook.com/docs/graph-api/guides/versioning>
pub const DEFAULT_GRAPH_API_VERSION: &str = "v23.0";

/// Client Struct for making calls to Facebook Graph
#[derive(Debug)]
pub struct Client {
    graph: String,
    short_live_user_access_token: String,
    long_live_user_access_token: String,
    //  page_access_token: Option<String>,
    /// The access token token type is used to indicate which type of token is
    /// currently passed to the method. It is required to provide either
    /// page_access_token  or user_access_token. corresponding to the token
    /// passed it
    page_access_token: String,
}

/// Empty Client
impl Default for Client {
    fn default() -> Self {
        let graph = "https://graph.facebook.com/v23.0/NODE/EDGE".to_string();
        Self {
            graph,
            short_live_user_access_token: "".to_string(),
            long_live_user_access_token: "".to_string(),
            page_access_token: "".to_string(),
        }
    }
}

impl Client {
    pub fn add_access_token(mut self, access_token: String) -> Client {
        self.page_access_token = access_token;
        self
    }
    /// This method add access token to the client, the method is expecting two
    /// input ( access token and access token type ). Since the access token
    /// type could be user token or page token , use access_token_type  to
    /// indicate which token is being passed in.
    /// For user_token, set the "access_toke_type" in the method to be
    /// "user_token" while for page token set the "access_toke_type" to be
    /// "page_token" example   Client::new(Token,"access_toke_type".
    /// to_string())
    pub fn new(user_access_token: UserToken, page_token: String) -> Client {
        Client::default().add_access_tokens(user_access_token, page_token)
    }

    /// Create a new Client with a custom API version
    ///
    /// # Arguments
    /// * `user_access_token` - The user's access token
    /// * `page_token` - The page access token
    /// * `api_version` - The Facebook Graph API version (e.g., "v23.0", "v22.0")
    ///
    /// # Notes
    /// Facebook's versioning policy guarantees each version for at least 2 years.
    /// Using older versions may result in deprecated endpoints or reduced functionality.
    ///
    /// # Panics
    /// Panics if the API version format is invalid. The version must be in format "vXX.X".
    ///
    /// # Example
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken};
    ///
    /// let client = Client::new_with_version(
    ///     UserToken::default(),
    ///     "page_token".to_string(),
    ///     "v23.0".to_string()
    /// );
    /// ```
    pub fn new_with_version(
        user_access_token: UserToken,
        page_token: String,
        api_version: String,
    ) -> Client {
        // Import the validation function from config module
        use crate::prelude::is_valid_api_version;

        if !is_valid_api_version(&api_version) {
            panic!(
                "Invalid API version format: '{}'. Expected format: 'vXX.X' (e.g., 'v23.0')",
                api_version
            );
        }

        let graph = format!("https://graph.facebook.com/{}/NODE/EDGE", api_version);
        let mut client = Client {
            graph,
            short_live_user_access_token: "".to_string(),
            long_live_user_access_token: "".to_string(),
            page_access_token: "".to_string(),
        };
        client.long_live_user_access_token = user_access_token.long_lived_token;
        client.short_live_user_access_token = user_access_token.access_token;
        client.page_access_token = page_token;
        client
    }

    /// This method add access token to the client when the user has
    /// authenticate from the frontend
    pub fn add_access_tokens(
        mut self,
        user_access_token: UserToken,
        page_access_token: String,
    ) -> Self {
        self.long_live_user_access_token = user_access_token.long_lived_token;
        self.short_live_user_access_token = user_access_token.access_token;
        self.page_access_token = page_access_token;
        self
    }

    pub fn page_user_access_token(self) -> Self {
        self
    }

    pub fn base_url(self) -> String {
        self.graph
    }

    /// Extract the API version from the client's graph URL
    ///
    /// # Returns
    /// The API version as a string slice (e.g., "v23.0"), or None if not found
    ///
    /// # Example
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken};
    ///
    /// let client = Client::new(
    ///     UserToken::default(),
    ///     "page_token".to_string()
    /// );
    ///
    /// assert_eq!(client.api_version(), Some("v23.0"));
    /// ```
    pub fn api_version(&self) -> Option<&str> {
        self.graph
            .split('/')
            .find(|s| s.starts_with('v') && s.contains('.'))
    }

    /// This method gives an entry point to User API and Facebook pages account
    /// API
    ///
    /// # Argument
    /// * `token_live_type` -  an enum of [TokenLiveType](TokenLiveType). This
    ///   will
    /// determine what type of pages access token in the respone.
    /// If TokenLiveType::SHORTLIVE  is the passed in then the page access token
    /// return will be short (expires in few hour).
    ///
    /// Note:: either way, if the UserToken passed in at the Client only
    /// container a short live token then the page access token returned will
    /// also be a short live
    ///
    /// For information on Tokens check [UserToken](UserToken)    
    ///
    /// Or check [Facebook token doc](https://developers.facebook.com/docs/facebook-login/access-tokens/)
    pub fn accounts(self, token_live_type: TokenLiveType) -> MeApi {
        let mut base_url = "".to_owned();
        if self.short_live_user_access_token.is_empty()
            && self.long_live_user_access_token.is_empty()
        {
            base_url = self.graph + &"?access_token=".to_string() + &self.page_access_token
        } else {
            match token_live_type {
                TokenLiveType::LONGLIVE => {
                    base_url = if self.long_live_user_access_token.is_empty() {
                        self.graph
                            + &"?access_token=".to_string()
                            + &self.short_live_user_access_token
                    } else {
                        self.graph
                            + &"?access_token=".to_string()
                            + &self.long_live_user_access_token
                    }
                }
                TokenLiveType::SHORTLIVE => {
                    base_url = if self.short_live_user_access_token.is_empty() {
                        self.graph
                            + &"?access_token=".to_string()
                            + &self.long_live_user_access_token
                    } else {
                        self.graph
                            + &"?access_token=".to_string()
                            + &self.short_live_user_access_token
                    }
                }
            }
        }

        MeApi::new(base_url)
    }

    ///  This method is used to pass user data/crediteniatls to the Post CONTENT
    /// method which will be used to post  to content to the  feed : Note this
    /// API can not be use for posting of vide and image
    pub fn feed(self, page_id: String) -> FeedApi {
        let base_url = self.graph.replace("NODE", &page_id);
        FeedApi::new(base_url, self.page_access_token)
    }

    ///  This method is used to get the different data avaliable on the page
    /// feed, it takes the "page_post_id" ( combination of the page_ and the
    /// post_id)
    pub fn post(self, page_post_id: String) -> PostApi {
        let base_url = self.graph.replace("NODE", &page_post_id);
        PostApi::new(base_url, self.page_access_token)
    }

    /// Facebook Video API allows you to publish Videos on Pages and Groups.
    /// Publishing on Users is not supported.
    ///
    /// The process for publishing Videos involves choosing an upload protocol
    /// and sending a POST request.
    /// The API suppports both Resumable and Non-Resumable upload protocols.
    /// Facebook recommend that you use the Resumable Upload protocol as it
    /// is more versatile and can gracefully handle connection interruptions.
    ///
    /// #Uploading protocools
    ///
    /// * Resumable Upload
    ///
    /// The Resumable Upload protocol is the preferred publishing protocol
    /// because it large videos into smaller chunks to avoid timeouts. This is
    /// especially useful for large videos where you are more likely to
    /// encounter a connection error.
    ///
    /// It is limited to Videos of 10GB and 4 hours.
    ///
    /// * Non-Resumable Upload
    ///
    /// Non-resumable upload can be used to uplaod video of less  than 1gb and
    /// 20 minute  if greater than that then resumable_upload must be used,
    ///
    /// ```rust
    ///  use facebook_api_rs::prelude::{UserToken,Client};
    ///
    /// let video_url = "video hosted url";
    /// let post_description = "This is the description of the our post";
    /// let title = "A video about nature";   
    /// ```
    ///
    /// * Hosted video
    ///
    /// ```rust
    ///  use facebook_api_rs::prelude::{UserToken,Client};
    ///
    /// let video_url = "video hosted url";
    /// let post_description = "This is the description of the our post";
    /// let title = "A video about nature";
    ///
    /// let resp =  Client::new(UserToken::default(),"your page token".to_owned())
    ///   .video_upload("page_id".to_owned())
    ///  .post_by_link(video_url, post_description,title).await?;
    /// ```
    pub fn video_upload(self, page_id: String) -> VideoApi {
        let base_url = self.graph.replace("NODE", &page_id);
        VideoApi::new(base_url, self.page_access_token) // initit videp Api
    }

    /// Entry point to instagram Account api.
    ///
    /// # Example for getting an instagram account id
    ///
    /// To get instagram business account id associated to a facebook page by
    /// passing the page id
    ///
    /// Since facebook only allows getting business accountt through
    /// facebook page
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken};
    /// use facebook_api_rs::prelude::InstagramAccountIds;
    ///
    ///  let instagram_account_id: InstagramAccountIds =  Client::new(
    ///               UserToken::default(),
    ///               "the facebook page access_token".to_string()
    ///              )
    ///            .instagram_account()
    ///            .account_id_by_facebook_page_id("facebook_page_id".to_owned()).await;
    /// ```
    ///  # Example for getting instagram account details
    ///
    /// If you already have an instagram account id, get the account information
    /// by passing the account id
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken};
    /// use facebook_api_rs::prelude::InstagramAccount;
    ///
    ///  let instagram_account : InstagramAccount =  Client::new(
    ///               UserToken::default(),
    ///               "the facebook page access_token".to_string()
    ///              )
    ///            .instagram_account()
    ///            .account_by_id("instagram_account_id".to_owned()).await?;
    /// ```

    pub fn instagram_account(self) -> InstagramApi {
        InstagramApi::new(self.page_access_token, self.graph)
    }

    /// You can use the Instagram Graph API to publish single images or single
    /// videos (single media posts), or posts containing multiple images and
    /// videos (carousel posts), on Instagram Business accounts
    ///
    /// # Limitation
    ///  *Instagram Creator accounts are not supported.
    /// * Accounts are limited to 25 API-published posts within a 24 hour
    ///   period. Carousels count as a single post.
    /// * JPEG is the only image format supported. Extended JPEG formats such as
    ///   MPO and JPS are not supported.
    /// * Stories are not supported.
    /// * Shopping tags are not supported.
    /// * Branded content tags are not supported.
    /// * Filters are not supported.
    /// * The hashtag symbol (#) must be HTML URL-encoded %23 in captions.
    /// * Publishing to Instagram TV is not supported
    ///  
    /// Different methods can be found here
    /// [InstagramContentPublishingApi](InstagramContentPublishingApi)
    ///
    /// For more information check [Facebook content doc](https://developers.facebook.com/docs/instagram-api/guides/content-publishing)
    pub fn instagram_content_publishing(
        self,
        instagram_id: String,
    ) -> InstagramContentPublishingApi {
        let base_url = self.graph.replace("NODE", &instagram_id);

        InstagramContentPublishingApi::new(self.page_access_token, base_url)
    }

    /// Represents an Instagram album, photo, story, or video (uploaded video,
    /// live video, or video created with the Instagram TV app).
    ///
    /// Note: The api onlr read existing media on instagram and can`t be used
    /// for post. To craete or upload media use
    /// [instagram_content_publishing](Client::instagram_content_publishing)
    ///
    /// # Endpoints
    /// This api allows you to perform the following operation:
    ///
    /// * Reading of media on instagram.
    /// * Comment on a media.
    /// * Update a post.
    /// * Insights on a media
    pub fn instagram_media(self, media_id: String) -> InstagramMediaApi {
        let base_url = self.graph.replace("NODE", &media_id);

        InstagramMediaApi::new(self.page_access_token, base_url)
    }

    pub fn search_pages(self) -> PagesSearchAPI {
        let base_url = self.graph.replace("NODE/EDGE", "pages/search");
        PagesSearchAPI::new(base_url, self.page_access_token)
    }

    /// Entry point to the instagram hashtag api
    ///
    /// # Limitation
    /// * `Max query`-  You can query a maximum of 30 unique hashtags on behalf
    ///   of an Instagram Business or Creator Account within a rolling,
    /// 7 day period. Once you query a hashtag, it will count against this limit
    /// for 7 days. Subsequent queries on the same hashtag within this time
    /// frame will not count against your limit, and will not reset its initial
    /// query 7 day timer.
    ///
    /// * Personally identifiable information will not be included in responses
    ///
    /// * Emojis in hashtag queries are not supported.
    ///
    /// * The API will return a generic error for any requests that include
    /// hashtags that facebook have deemed sensitive or offensive.
    ///
    /// [facebook hashtag doc](https://developers.facebook.com/docs/instagram-api/guides/hashtag-search)
    pub fn instagram_hashtag(self, instagram_id: String) -> HashtagAPi {
        let mut base_url = self.graph.replace("NODE/", "");
        base_url = base_url + "?user_id=" + &instagram_id;
        HashtagAPi::new(self.page_access_token, base_url)
    }

    pub fn user_token(self) -> UserToken {
        // AccessTokenInformation::default()
        UserToken::default().set_url(self.graph)
    }

    pub fn batch_request(self, page_id: String) -> BatchApi {
        let base_url = self.graph.replace("/NODE/EDGE", "");
        BatchApi::new(base_url, self.page_access_token, page_id)
    }
}

#[cfg(test)]
mod test {
    use crate::graph::client::Client;
    use crate::login::token::UserToken;

    #[test]
    fn test_default_api_version() {
        let client = Client::new(UserToken::default(), "page_token".to_string());
        assert_eq!(client.api_version(), Some("v23.0"));
        assert!(client.graph.contains("v23.0"));
    }

    #[test]
    fn test_custom_api_version() {
        let client = Client::new_with_version(
            UserToken::default(),
            "page_token".to_string(),
            "v22.0".to_string(),
        );
        assert_eq!(client.api_version(), Some("v22.0"));
        assert!(client.graph.contains("v22.0"));
    }

    #[test]
    fn test_api_version_extraction() {
        let client = Client::new(UserToken::default(), "page_token".to_string());
        let version = client.api_version();
        assert!(version.is_some());
        assert!(version.unwrap().starts_with('v'));
        assert!(version.unwrap().contains('.'));
    }

    #[test]
    #[should_panic(expected = "Invalid API version format")]
    fn test_invalid_version_format_panic() {
        let _client = Client::new_with_version(
            UserToken::default(),
            "page_token".to_string(),
            "23.0".to_string(), // Missing 'v' prefix
        );
    }

    //  #[test]
    // fn test_builder() {
    // let mut token = UserToken::default();
    // token.access_token = "123".to_string();
    //
    // let accounts = Client::default()
    // .add_access_tokens(token, "".to_string())
    // .accounts(TokenLiveType::LONGLIVE)
    // .get();
    // assert_eq!(
    // "https://graph.facebook.com/v23.0/me/accounts?access_token=123",
    // accounts.url()
    // )
    // }
}
