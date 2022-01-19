//! This utils will contain different methods and struct that are shared within the pages  mod
//!

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Fields {
    pub(crate) fields: Vec<String>,
}

impl Default for Fields {
    /// This parameters are used as fields which are passed in as a query
    /// parameters to the get post request and feeds request  
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

/// expected fields gotten from the get post request
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct GetPostResponse {
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
