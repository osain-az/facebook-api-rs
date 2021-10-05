use crate::graph::get_posts::GetPostApi;
use crate::graph::me::MeApi;

use crate::graph::pages::PagesAPI;
use crate::graph::prelude::account::InstagramApi;
use crate::graph::prelude::publish::InstagramPostApi;
use crate::graph::video::VideoApi;
use crate::login::token::Token;
use crate::prelude::{PagesSearchAPI, PostApi, VideoParams};
use seed::{prelude::*, *};

/// This mod will server as method binder that allow other mothod to post content to the api,
/// it also hold all the user creditenials that will pass to each method.

/// Client Struct for making calls to Facebook Graph
#[derive(Debug)]
pub struct Client {
    graph: String,
    access_token: String,
    page_access_token: Option<String>,
}

/// Empty Client
impl Default for Client {
    fn default() -> Self {
        let graph = "https://graph.facebook.com/v11.0/NODE/EDGE".to_string();

        Self {
            graph,
            access_token: "".to_string(),
            page_access_token: None,
        }
    }
}

impl Client {
    /// this method add access token to the client when the user has authenticate from the frontend

    pub fn new(access_token: Token) -> Client {
        Client::default().add_access_token(access_token.access_token)
    }

    /// this method add access token to the client when the user has authenticate from the frontend

    pub fn add_access_token(mut self, access_token: String) -> Self {
        self.access_token = access_token;
        self
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }

    /// this method  is used to pass user data/crediteniatls to the ME method which will be used to reach the ME API

    pub fn me(self) -> MeApi {
        MeApi::new(self.graph + &"?access_token=".to_string() + &self.access_token)
    }

    ///  this method is used to pass user data/crediteniatls to the Post CONTENT  method which will
    /// be used to post content to the  feed : Note this API can not be use for posting of vide and image
    ///
    pub fn post(self, page_id: String, page_token: String) -> PostApi {
        let base_url = self.graph.replace("NODE", &page_id);
        PostApi::new(base_url, page_token)
    }

    ///  This method passes the page_post_id and the page token to the get method which will be use to get the
    /// content of the id that was pass
    pub fn get_post(self, page_post_id: String, page_token: String) -> GetPostApi {
        let base_url = self.graph.replace("NODE", &page_post_id);
        GetPostApi::new(base_url, page_token)
    }

    ///Posting video using link ( hosted file)
    ///  This method take the page id and the page token you want post video to.
    pub fn get_post_video_link(self, page_id: String, page_token: &str) -> PostApi {
        let base_url = self.graph.replace("NODE", &page_id);
        PostApi::new(base_url, page_token.to_string())
    }

    ///
    ///This method allows developers to choose which viddeo uploading method they want to use.
    /// For Large file greater than 1gb and 20 minute  user method called resumable_upload, for video
    /// files smaller than that use method called "non_resumable".
    ///Although facebook recommend using resumable method.
    ///
    pub fn video_upload(self, page_id: String, page_token: &str) -> VideoApi {
        let base_url = self.graph.replace("NODE", &page_id);
        VideoApi::new(base_url, page_token.to_string()) // initit videp Api
    }

    // Instagram end point call
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
            .add_access_token(token.access_token)
            .me()
            .accounts();
        assert_eq!(
            "https://graph.facebook.com/v11.0/me/accounts?access_token=123",
            accounts.url()
        )
    }
}
