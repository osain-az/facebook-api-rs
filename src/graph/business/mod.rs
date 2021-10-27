use crate::graph::data::Data;
use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::*;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};

use crate::graph::accounts::AccountsAPI;
use crate::graph::data::Data;

#[derive(Deserialize, Debug, Default, Serialize)]
pub struct Businesses {
    // the model should the structed according to the permissions the facebook_app have, since the result
    //will depend on the access
    pub primary_page: String, // not commpletely
    pub name: String,
    pub id: String,
    pub profile_picture_uri: String,
    pub created_by: String,
}

impl Businesses {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_creator(&self) -> &String {
        &self.created_by
    }
}

pub struct BusinessesAPI {
    url: String,
}

impl BusinessAPI {
    pub fn new(base_url: String) -> BusinessesAPI {
        BusinessAPI {
            url: base_url.replace("EDGE", "businesses"),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    pub async fn get(&self) -> seed::fetch::Result<Data<Businesses>> {
        let request = Request::new(&self.url).method(Method::Get);
        fetch(request).await?.json::<Data<Businesses>>().await
    }

    pub async fn postVideo(&self) -> seed::fetch::Result<Data<Businesses>> {
        let request = Request::new(&self.url).method(Method::Get);
        fetch(request).await?.json::<Data<Businesses>>().await
    }
}
