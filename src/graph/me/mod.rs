//! This mod will  contain different method and struck  used to interacting with
//! ME API
//!
//! The /me node is a special endpoint that translates to the object ID of
//! the person or Page whose access token is currently being used to make the
//! API calls.
//!
//!  If you had a User access token, you could
//! retrieve a User's name and ID by using: The data in the response
//! will depend on the "Fields" parameters  you pass along the get request
//! exmaple fields=id,name,email,picture......

use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::*;
use serde::{Deserialize, Serialize};

use crate::graph::accounts::AccountsAPI;
use crate::graph::data::Data;
use crate::graph::prelude::Accounts;

/// This struct contain different data gotten as a response  when a user sign in
#[derive(Deserialize, Serialize)]
pub struct Me {
    name: String,
    id: String,
    last_name:String,
    first_name:String,
    picture: PictureData,
    email:String,
}

impl Me {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn picture(&self) -> &PictureData {
        &self.picture
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}

#[derive(Deserialize, Serialize)]
 pub struct PictureData{
   pub data: FacebookPictureUserPicture
}

#[derive(Deserialize, Serialize)]
pub struct  FacebookPictureUserPicture {
    url: String,
}

impl FacebookPictureUserPicture {
    pub fn url(&self) -> &str {
        &self.url
    }
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

    /// The /me node is a special endpoint that translates to the object ID of
    /// the person or Page whose access token is currently being used
    /// to make the API calls. If you had a User access token, you could
    /// retrieve a User's name and ID by using: The data in the response
    /// will depend on the "Fields" parameters  you pass along the get request
    /// exmaple fields=id,name,email,picture.......
    pub async fn details(&self) -> seed::fetch::Result<Me> {
        let fields = "&fields=id,name,picture, email,first_name,last_name,about,birthday,gender,link";
          let base_ur = self.url.replace("EDGE", "");
           let url = base_ur + fields;
        let request = Request::new(url).method(Method::Get);
        fetch(request).await?.json::<Me>().await
    }
}
