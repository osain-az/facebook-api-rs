use crate::accounts::Accounts;
use crate::data::Data;
use crate::me::Me;
use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeApi {
    pub url: String,
}

impl MeApi {
    pub fn new(graph_base: String) -> MeApi {
        MeApi {
            url: graph_base.replace("NODE", "me"),
        }
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

pub struct AccountsAPI {
    pub url: String,
}

impl AccountsAPI {
    pub fn new(base_url: String) -> AccountsAPI {
        AccountsAPI {
            url: base_url.replace("EDGE", "accounts"),
        }
    }
    // pub fn get_access_token(&self) -> String {
    //     "?access_token=".to_string() + &self.access_token
    // }

    pub async fn get(&self) -> seed::fetch::Result<Data<Accounts>> {
        log!(self.url);
        let request = Request::new(&self.url).method(Method::Get);
        fetch(request).await?.json::<Data<Accounts>>().await
    }
}
