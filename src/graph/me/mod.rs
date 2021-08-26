use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::*;
use serde::{Deserialize, Serialize};

use crate::graph::accounts::AccountsAPI;
use crate::graph::data::Data;

#[derive(Deserialize, Serialize)]
pub struct Me {
    name: String,
    user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeApi {
    url: String,
}

impl MeApi {
    pub fn new(graph_base: String) -> MeApi {
        MeApi {
            url: graph_base.replace("NODE", "me"),
        }
    }
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn accounts(self) -> AccountsAPI {
        AccountsAPI::new(self.url)
    }

    pub async fn get(&self) -> seed::fetch::Result<Data<Me>> {
        log!(self.url);
        let request = Request::new(&self.url).method(Method::Get);
        fetch(request).await?.json::<Data<Me>>().await
    }
}
