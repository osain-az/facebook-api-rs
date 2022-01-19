//!  This mod will server as method binder that allow  to access different end
//! poinst availiable on the facebook-api.rs

use crate::graph::instagram::media::InstagramMediaApi;
use crate::graph::me::MeApi;
use crate::graph::pages::feed::FeedApi;
use crate::graph::pages::post::PostApi;
use crate::graph::prelude::account::InstagramApi;
use crate::graph::prelude::publish::InstagramPostApi;
use crate::login::token::{AccessTokenInformation, Token};
use crate::prelude::search::PagesSearchAPI;
use crate::prelude::video::VideoApi;
use std::option::Option::Some;

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
        let graph = "https://graph.facebook.com/v11.0/NODE/EDGE".to_string();
        //  let insta_graph = "https://graph.instagram.com/v11.0/NODE/EDGE".to_string();

        Self {
            graph,
            short_live_user_access_token: "".to_string(),
            long_live_user_access_token: "".to_string(),
            page_access_token: "".to_string(),
        }
    }
}

impl Client {
    /// This method add access token to the client, the method is expecting two
    /// input ( access token and access token type ). Since the access token
    /// type could be user token or page token , use access_token_type  to
    /// indicate which token is being passed in.
    /// For user_token, set the "access_toke_type" in the method to be
    /// "user_token" while for page token set the "access_toke_type" to be
    /// "page_token" example   Client::new(Token,"access_toke_type".
    /// to_string())
    pub fn new(user_access_token: Token, pag_access_token: String) -> Client {
        Client::default().add_access_tokens(user_access_token, pag_access_token)
    }

    /// This method add access token to the client when the user has
    /// authenticate from the frontend
    pub fn add_access_tokens(
        mut self,
        user_access_token: Token,
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

    /// This method  is used to pass user data/crediteniatls to the ME method
    /// which will be used to reach the ME API.
    /// since facebbok allows two access token ( short and long live )  to used
    /// or get a long live token for your page passed in "long_live"  while
    /// if you intented to used a short live token then pass in "short_live"
    /// . For more information on facebook documenation check
    /// <https://developers.facebook.com/docs/facebook-login/access-tokens/>
    pub fn me_by_short_or_long_live_token(self, token_live_type: String) -> MeApi {
        if token_live_type == "short_live" {
            MeApi::new(
                self.graph + &"?access_token=".to_string() + &self.short_live_user_access_token,
            )
        } else {
            MeApi::new(
                self.graph + &"?access_token=".to_string() + &self.long_live_user_access_token,
            )
        }
    }

    ///  This method is used to pass user data/crediteniatls to the Post CONTENT
    /// method which will be used to post  to content to the  feed : Note this
    /// API can not be use for posting of vide and image
    pub fn feeds(self, page_id: String) -> FeedApi {
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

    /// This method allows developers to choose which viddeo uploading method
    /// they want to use. For Large file greater than 1gb and 20 minute
    ///  method called resumable_upload must be used, for video files smaller
    /// than that either of the method can be used  ("non_resumable",
    /// "resumable_upload"), for video hosted online(video_url), the method
    /// called "post_by_link" can be used. Note: facebook recommend  using
    /// resumable method when uploading direct directly.
    pub fn video_upload(self, page_id: String) -> VideoApi {
        let base_url = self.graph.replace("NODE", &page_id);
        VideoApi::new(base_url, self.page_access_token) // initit videp Api
    }

    // Instagram end point call

    pub fn instagram_account(self, facebook_page_id: String) -> InstagramApi {
        let base_url = self.graph.replace("NODE", &facebook_page_id);
        InstagramApi::new(self.page_access_token, base_url)
    }

    pub fn instagram_publish(self, instagram_id: String) -> InstagramPostApi {
        let base_url = self.graph.replace("NODE", &instagram_id);

        InstagramPostApi::new(self.page_access_token, base_url)
    }

    pub fn instagram_media_container(self, media_container_id: String) -> InstagramMediaApi {
        let base_url = self.graph.replace("NODE", &media_container_id);

        InstagramMediaApi::new(self.page_access_token, base_url)
    }

    pub fn search_pages(self) -> PagesSearchAPI {
        let base_url = self.graph.replace("NODE/EDGE", "pages/search");
        PagesSearchAPI::new(base_url, self.page_access_token)
    }

    pub fn token_info(self) -> Token {
        // AccessTokenInformation::default()
        Token::default()
    }
}

#[cfg(test)]
mod test {
    use crate::graph::client::Client;
    use crate::login::token::Token;

    #[test]
    fn test_builder() {
        let mut token = Token::default();
        token.access_token = "123".to_string();

        let accounts = Client::default()
            .add_access_tokens(token, "".to_string())
            .me_by_short_or_long_live_token("short_live".to_string())
            .accounts();
        assert_eq!(
            "https://graph.facebook.com/v11.0/me/accounts?access_token=123",
            accounts.url()
        )
    }
}
