use seed::fetch::fetch;
use seed::prelude::{Method, Request};

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct InstaAccount {
    pub instagram_business_account: InstaAccountId,
    pub id: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]

pub struct InstaAccountId {
    pub id: String,
}

pub struct InstagramApi {
    page_access_token: String,
    base_url: String,
}

impl InstagramApi {
    pub fn new(page_access_token: String, base_url: String) -> InstagramApi {
        InstagramApi {
            page_access_token,
            base_url,
        }
    }

    pub async fn insta_account(self) -> seed::fetch::Result<InstaAccount> {
        let base_url = self.base_url.replace("EDGE", "?");
        let url = base_url
            + "fields=instagram_business_account"
            + "&access_token="
            + &self.page_access_token;

        let request = Request::new(url).method(Method::Get);
        fetch(request).await?.json::<InstaAccount>().await
    }
}
