use crate::graph::me::MeApi;

use crate::graph::pages::feed::FeedApi;
use crate::graph::prelude::account::InstagramApi;
use crate::graph::prelude::publish::InstagramPostApi;
use crate::login::token::{AccessTokenInformation, Token};
use crate::prelude::search::PagesSearchAPI;
use crate::prelude::video::VideoApi;
use seed::{prelude::*, *};
use std::option::Option::Some;
use crate::graph::pages::post::PostApi;

/// This mod will server as method binder that allow other mothod to feed
/// content to the api, it also hold all the user creditenials that will pass to
/// each method.

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

        Self {
            graph,
            short_live_user_access_token: "".to_string(),
            long_live_user_access_token: "".to_string(),
            page_access_token: "".to_string(),
        }
    }
}

impl Client {
    /// this method add access token to the client, the method is expecting two
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

    /// this method add access token to the client when the user has
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

    /// this method  is used to pass user data/crediteniatls to the ME method
    /// which will be used to reach the ME API

    pub fn me_by_short_or_long_live_token(self, toke_live_type: String) -> MeApi {
        if toke_live_type == "short_live" {
            MeApi::new(
                self.graph + &"?access_token=".to_string() + &self.short_live_user_access_token,
            )
        } else {
            MeApi::new(
                self.graph + &"?access_token=".to_string() + &self.long_live_user_access_token,
            )
        }
    }

    /// this method  is used to pass user data/crediteniatls to the ME method
    /// which will be used to reach the ME API

    ///  this method is used to pass user data/crediteniatls to the Post CONTENT
    /// method which will be used to feed content to the  feed : Note this
    /// API can not be use for posting of vide and image
    //   pub fn post(self, page_id: String, page_token: String) -> PostApi {
    // let base_url = self.graph.replace("NODE", &page_id);
    // PostApi::new(base_url, page_token)
    // }

    ///  This method used to get the different methods avaliable on the page
    /// feed API
    pub fn feeds(self, page_post_id: String) -> FeedApi {
        let base_url = self.graph.replace("NODE", &page_post_id);
        FeedApi::new(base_url, self.page_access_token)
    }


    ///  This method used to get the different methods avaliable on the page
    /// feed API
    pub fn post(self, page_post_id: String) -> PostApi {
        let base_url = self.graph.replace("NODE", &page_post_id);
        PostApi::new(base_url, self.page_access_token)
    }
    

    /// Posting video using link ( hosted file)
    ///  This method take the page id and the page token you want feed video to.
    // pub fn get_post_video_link(self, page_id: String, page_token: &str) -> PostApi {
    // let base_url = self.graph.replace("NODE", &page_id);
    // PostApi::new(base_url, page_token.to_string())
    // }
    /// This method allows developers to choose which viddeo uploading method
    /// they want to use. For Large file greater than 1gb and 20 minute
    /// user method called resumable_upload, for video files smaller than
    /// that use method called "non_resumable". Although facebook recommend
    /// using resumable method.
    pub fn video_upload(self, page_id: String) -> VideoApi {
        let base_url = self.graph.replace("NODE", &page_id);
        VideoApi::new(base_url, self.page_access_token) // initit videp Api
    }

    // Instagram end point call

    pub fn instagram_account(self, facebook_page_id: String) -> InstagramApi {
        let base_url = self.graph.replace("NODE", &facebook_page_id);
        InstagramApi::new(self.page_access_token, base_url)
    }

    pub fn instagram(self, instagarm_id: String) -> InstagramPostApi {
        let base_url = self.graph.replace("NODE", &instagarm_id);

        InstagramPostApi::new(self.page_access_token, base_url)
    }

    pub fn search_pages(self) -> PagesSearchAPI {
        let base_url = self.graph.replace("NODE/EDGE", "pages/search");
        PagesSearchAPI::new(base_url, self.page_access_token)
    }

    pub fn token_info(self) -> AccessTokenInformation {
        AccessTokenInformation::default()
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
