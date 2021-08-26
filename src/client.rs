use crate::me_api::*;
pub use crate::{code::*, data::*, image::*, image::*, redirect_url::*, redirect_url::*, token::*};
use async_trait::async_trait;
use seed::{prelude::*, *};
///Seed Client Struct for making calls to Facebook Graph
#[derive(Debug)]
pub struct Client {
    graph: String,
    node: String,
    edge: String,
    fields: Vec<String>,
    access_token: String,
    page_access_token: Option<String>,
}
/// Empty Client
impl Default for Client {
    fn default() -> Self {
        let graph = "https://graph.facebook.com/v11.0/NODE/EDGE".to_string();

        Self {
            graph,
            node: "".to_string(),
            edge: "".to_string(),
            fields: Vec::new(),
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

    pub fn me(self) -> MeApi {
        MeApi::new(self.graph + &"?access_token=".to_string() + &self.access_token)
    }

    // pub fn accounts(self) -> AccountsAPI {
    //     AccountsAPI::new(self.graph, self.access_token)
    // }

    pub fn create_url(&self) -> String {
        self.graph.to_string()
            + &*self.node.to_string()
            + &*"/".to_string()
            + &self.edge.to_string()
            + "?access_token="
            + &self.access_token
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
}

mod test {
    use crate::client::Client;
    use crate::token::Token;

    #[test]
    fn test_builder() {
        let mut token = Token::default();
        token.access_token = "123".to_string();
        let client = Client::default().add_access_token(token).me().accounts();

        assert_eq!(
            "https://graph.facebook.com/v11.0/me/accounts?access_token=123",
            client.create_url()
        )
    }
}
