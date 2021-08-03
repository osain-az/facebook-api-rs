use crate::accounts::Accounts;
use seed::browser::fetch::FetchError;
use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MeApi {
    pub url: String,
    // ///Name of the user
    // name: String,
    // ///The user_id of the user
    // pub id: String
}

impl MeApi {
    pub fn new(graph_base: String) -> MeApi {
        Self {
            url: graph_base + "/me",
        }
    }

    pub fn accounts(self) -> AccountsAPI {
        AccountsAPI::new(self.url)
    }
}

pub struct AccountsAPI {
    pub url: String,
}

impl AccountsAPI {
    pub fn new(base_url: String) -> AccountsAPI {
        Self {
            url: base_url + "/accounts",
        }
    }

    pub async fn get(&self) -> seed::fetch::Result<Accounts> {
        let request = Request::new(&self.url).method(Method::Get);
        fetch(request).await.unwrap().json::<Accounts>().await
    }
}
