//! The Post API let you perform operation (get post, update) on individual post
//! in a profile's feed.
//!
//! The profile could be a user, page, app, or group.;
//! Note: This end point does not allow you to create post or delete.
//! for more details check facebook documentation on post Api.
//! https://developers.facebook.com/docs/graph-api/reference/post
use crate::graph::pages::utils::{Fields, GetPostResponse};
use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};

/// The Post API let  you perform operation on individual post in a profile's
/// feed. The profile could be a user, page, app, or group. (get post, update);
/// Note: This end point does not allow you to create post or delete.
/// for more details check facebook documentation on post Api.
/// https://developers.facebook.com/docs/graph-api/reference/post
pub struct PostApi {
    base_url: String,
    page_access_token: String,
}

impl PostApi {
    /// this is a static method used to create an instance of the feedApi
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
}
