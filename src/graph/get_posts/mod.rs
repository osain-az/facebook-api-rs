use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
// ODO rename it feed, and move it under page folder
//#[derive(Deserialize, Debug, Default, Serialize)]

pub struct Fields {
    fields: Vec<String>,
}

impl Default for Fields {
    fn default() -> Self {
        let field_list = vec![
            "from",
            "id",
            "message_tags",
            "story",
            "story_tags",
            "permalink_url",
            "message",
            "shares",
            "comments",
            "likes",
            "reactions",
        ];
        let fields = field_list.iter().map(|&field| field.into()).collect();
        Self { fields }
    }
}

// This will be in another files
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct GetPost {
    pub id: String,
    pub message: String,
    pub from: From,
    pub permalink_url: String,
}

#[derive(Deserialize, Debug, Default, Serialize)]
pub struct From {
    pub id: String,
    pub name: String,
}

pub struct GetPostApi {
    base_url: String,
    access_token: String,
}

impl GetPostApi {
    pub fn new(base_url: String, access_token: String) -> GetPostApi {
        GetPostApi {
            base_url,
            access_token,
        }
    }

    pub async fn get(self) -> seed::fetch::Result<GetPost> {
        let mut url = self.base_url.replace("EDGE", "?fields=");

        let field_count = Fields::default().fields.len();
        for (count, field) in Fields::default().fields.into_iter().enumerate() {
            if count < field_count - 1 {
                url = url + &field + ",";
            } else {
                url = url + &field; // remove the comma in the last filed
            }
        }

        let base_url = url + "&access_token=" + &self.access_token;
        let request = Request::new(base_url).method(Method::Get);
        fetch(request).await?.json::<GetPost>().await
    }
}
