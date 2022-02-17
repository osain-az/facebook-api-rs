//! The Post API let you perform operation (get post, update) on individual post
//! in a profile's feed.
//!
//! The profile could be a user, page, app, or group.;
//! Note: This end point does not allow you to create post or delete.
//! for more details check facebook documentation on post Api.
//! https://developers.facebook.com/docs/graph-api/reference/post
use crate::graph::pages::utils::{Fields, From, GetPostResponse};
use crate::prelude::errors::ClientErr;
use crate::prelude::utils::ResponseStatus;
use crate::prelude::{Data, HttpConnection};
use serde::{Deserialize, Serialize};

/// The Post API let  you perform operation on individual post in a profile's
/// feed. The profile could be a user, page, app, or group. (get post, update);
/// Note: This end point does not allow you to create post or delete.
/// for more details check facebook documentation on post Api.
/// <https://developers.facebook.com/docs/graph-api/reference/page-post/>
pub struct PostApi {
    base_url: String,
    page_access_token: String,
}

impl PostApi {
    /// This is a static method used to create an instance of the feedApi
    /// Note: this method is called inside of the Client method
    pub fn new(base_url: String, access_token: String) -> PostApi {
        PostApi {
            base_url,
            page_access_token: access_token,
        }
    }

    ///  this method sends a get request to the facebook api (GET
    /// /v12.0/{post-id}). it returns the data of the post_id  you have
    /// provided
    pub async fn get(self) -> Result<GetPostResponse, ClientErr> {
        let mut url = self.base_url.replace("EDGE", "?fields=");

        let field_count = Fields::default().fields.len();
        for (count, field) in Fields::default().fields.into_iter().enumerate() {
            if count < field_count - 1 {
                url = url + &field + ",";
            } else {
                url = url + &field; // remove the comma in the last filed
            }
        }
        let base_url = url + "&access_token=" + &self.page_access_token;

        let resp = HttpConnection::get::<GetPostResponse>(base_url, "".to_string()).await?;
        Ok(resp)
    }

    //TODO: impliment post uodated methods and test newly added methods
    pub async fn update() {}

    pub async fn comments(self) -> Result<Comments, ClientErr> {
        let mut base_url = self.base_url.replace("EDGE", "commnets");
        let url = base_url + "&access_token=" + &self.page_access_token;
        let resp = HttpConnection::get::<Comments>(url, "".to_string()).await?;
        Ok(resp)
    }

    pub async fn comment_on_post(self, message: String) -> Result<ResponseStatus, ClientErr> {
        let mut base_url = self.base_url.replace("EDGE", "commnets");
        let url = base_url + "?message=" + &message + "&access_token=" + &self.page_access_token;

        let resp = HttpConnection::post::<ResponseStatus, String>(url, "".to_string()).await?;
        Ok(resp)
    }

    pub async fn delete(self) -> Result<ResponseStatus, ClientErr> {
        let mut base_url = self.base_url.replace("EDGE", "");
        let url = base_url + "&access_token=" + &self.page_access_token;
        let resp = HttpConnection::delete::<ResponseStatus>(url, "".to_string()).await?;
        Ok(resp)
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Comments {
    data: Data<Comment>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Comment {
    from: From,
    created_time: String,
    message: String,
    id: String,
}
