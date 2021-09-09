use crate::graph::getPosts::GetPostApi;
use crate::graph::me::MeApi;
use crate::login::token::Token;
use crate::prelude::PostApi;
use seed::{prelude::*, *};

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
    pub fn new(access_token: Token) -> Client {
        Client::default().add_access_token(access_token.access_token)
    }

    pub fn add_access_token(mut self, access_token: String) -> Self {
        self.access_token = access_token;
        self
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
    
    pub fn me(self) -> MeApi {
        MeApi::new(self.graph + &"?access_token=".to_string() + &self.access_token)
    }
    
    pub fn post(self, page_id: String, page_token: String) -> PostApi {
        let base_url = self.graph.replace("NODE", &page_id);
        PostApi::new(base_url, page_token)
    }
    
    pub fn get_post(self, page_post_id: String, page_token: String) -> GetPostApi {
        let base_url = self.graph.replace("NODE", &page_post_id);
        GetPostApi::new(base_url, page_token)
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
            .add_access_token(token.access_token)
            .me()
            .accounts();

        assert_eq!(
            "https://graph.facebook.com/v11.0/me/accounts?access_token=123",
            accounts.url()
        )
    }
}
