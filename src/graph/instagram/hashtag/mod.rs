//! An end_point to Instagram Hashtag Search.
//!
//! [instagram hashtag docs](https://developers.facebook.com/docs/instagram-api/reference/ig-hashtag-search)

use crate::prelude::errors::ClientErr;
use crate::prelude::utils::Id;
use crate::prelude::HttpConnection;
use serde::Deserialize;

pub struct HashtagAPi {
    access_token: String,
    base_url: String,
}

impl HashtagAPi {
    pub fn new(access_token: String, base_url: String) -> HashtagAPi {
        HashtagAPi {
            access_token,
            base_url,
        }
    }

    /// Get a specific hashtag's  Id from a given hashtag name
    ///
    /// # Limitation
    /// - You can query a maximum of 30 unique hashtags within a 7 day period.
    /// - The API will return a generic error for any queries that include
    ///   hashtags that we have deemed sensitive or offensive.
    ///
    /// Note: if you search for a key w
    pub async fn hashtag_search(self, hashtag_key_word: String) -> Result<HashtagId, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "ig_hashtag_search");
        let url = base_url + "&q=" + &hashtag_key_word + "&access_token=" + &self.access_token;

        let resp = HttpConnection::get::<HashtagId>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// Get a collection of media objects from recent post that have a given
    /// hashtag. For example: `#Coke`
    ///
    /// # Argument
    ///
    /// * `hashtag_id`- The id of the hashtag.
    pub async fn recent_medias_by_hashtag_id(
        self,
        hashtag_id: String,
    ) -> Result<MediaIds, ClientErr> {
        let edge = hashtag_id + "/recent_media";
        let base_url = self.base_url.replace("EDGE", edge.as_str());
        let url = base_url + "&access_token=" + &self.access_token;

        let resp = HttpConnection::get::<MediaIds>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// Get a collection of media objects from top media post that have a given
    /// hashtag. For example: `#Coke`
    ///
    /// # Argument
    ///
    /// * `hashtag_id`- The id of the hashtag.
    ///
    /// # Example
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken};
    ///  // We dont actually need userToken since we have the page token that the instagram account is associated with.
    /// Client::new(UserToken::default(), "facebook_page_token".to_owned() )
    /// ```
    pub async fn top_medias_by_hashtag_id(self, hashtag_id: String) -> Result<MediaIds, ClientErr> {
        let edge = hashtag_id + "/top_media";
        let base_url = self.base_url.replace("EDGE", edge.as_str());
        let url = base_url + "&access_token=" + &self.access_token;

        let resp = HttpConnection::get::<MediaIds>(url, "".to_string()).await?;
        Ok(resp)
    }
}

/// ```
///  struct MediaIds {
///    data: Vec![
///          {id}
///        ]};
/// ```
#[derive(Deserialize, Debug)]
pub struct MediaIds {
    pub data: Vec<Id>,
}

#[derive(Deserialize, Debug)]
pub struct HashtagId {
    pub id: String,
}
