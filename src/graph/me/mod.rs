use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::*;
use serde::{Deserialize, Serialize};

use crate::graph::accounts::AccountsAPI;
use crate::graph::data::Data;

/// This mod will  contain different method and struck  used to intracting with ME  API

/// This struct contain different data gotten as a response  when a user sign

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

    /// The /me node is a special endpoint that translates to the object ID of the person or Page whose access token is currently being used
    /// to make the API calls. If you had a User access token, you could retrieve a User's name and ID by using:
    /// The data in the response will depend on the "Fields" parameters  you pass along the get request exmaple fields=id,name,email,picture.......

    pub async fn get(&self) -> seed::fetch::Result<Data<Me>> {
        log!(self.url);
        let request = Request::new(&self.url).method(Method::Get);
        fetch(request).await?.json::<Data<Me>>().await
    }
}
