//! An end_point to Instagram business account for any given facebook
//! page
//! [facebook docs](https://developers.facebook.com/docs/instagram-api/reference/page)

use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::Deserialize;
use urlencoding::encode;

/// Instagram business account
/// ```
/// use facebook_api_rs::prelude::InstagramAccountId;
///
///  struct InstagramAccountIds {
///     pub instagram_business_account: InstagramAccountId:{
///     pub id:String
///  },
///    pub id: String,
/// }
/// ```
#[derive(Deserialize, Clone, Debug)]
pub struct InstagramAccountIds {
    pub instagram_business_account: InstagramAccountId,
    pub id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct InstagramAccountId {
    pub id: String,
}

#[derive(Clone)]
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

    /// Instagram account id associated to a given facebook page
    pub async fn account_id_by_facebook_page_id(
        self,
        facebook_page_id: String,
    ) -> Result<InstagramAccountIds, ClientErr> {
        let graph_url = self.base_url.replace("NODE", &facebook_page_id);
        let base_url = graph_url.replace("EDGE", "?");
        let url = base_url
            + "fields=instagram_business_account"
            + "&access_token="
            + &self.page_access_token;
        let resp = HttpConnection::get::<InstagramAccountIds>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// Get request for Instagram account detail.
    ///
    /// The response is an [InstagramAccount](InstagramAccount) if successful or
    /// [ClientErr](ClientErr) if request failed.
    ///
    /// # Argument
    ///
    /// * `instagram_id`- A string of instagram account id.
    ///
    /// # Example
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Client, UserToken};
    /// use facebook_api_rs::prelude::errors::ClientErr;
    /// use facebook_api_rs::prelude::InstagramAccount;
    ///
    ///  let instagram_account : Result< InstagramAccount, ClientErr> =  
    ///         Client::new(
    ///               UserToken::default(),
    ///               "the facebook page access_token".to_string()
    ///              )
    ///            .instagram_account()
    ///           .account_by_id("instagram_account_id".to_owned()).await;
    /// ```
    /// [facebook account doc](https://developers.facebook.com/docs/instagram-api/reference/ig-user)
    pub async fn account_by_id(self, instagram_id: String) -> Result<InstagramAccount, ClientErr> {
        let graph_url = self.base_url.replace("NODE", &instagram_id);
        let mut url = graph_url.replace("EDGE", "?");

        let url_fields = Fields::default().build_url_with_fields();

        let mut request_url =
            url + "fields=" + url_fields.as_str() + "&access_token=" + &self.page_access_token;
        let resp = HttpConnection::get::<InstagramAccount>(request_url, "".to_string()).await?;
        Ok(resp)
    }
}

/// Instagram account
///
/// ```
///  struct InstagramAccount {
///     pub biography: String,
///     pub id: String,
///     pub followers_count: u32,
///     pub follows_count: u32,
///     pub media_count: u32,
///     pub name: String,
///    pub profile_picture_url: String,
///     pub username: String,
///     pub website: String,
/// }
/// ```
#[derive(Deserialize, Clone, Debug, Default)]
pub struct InstagramAccount {
    pub biography: String,
    pub id: String,
    pub followers_count: u32,
    pub follows_count: u32,
    // #[serde(default = 0)]
    pub media_count: u32,
    pub name: String,
    pub profile_picture_url: String,
    pub username: String,
    pub website: String,
}

struct Fields {
    pub(crate) fields: Vec<String>,
}

impl Default for Fields {
    fn default() -> Self {
        let field_list = vec![
            "biography",
            "id",
            "followers_count",
            "follows_count",
            "media_count",
            "name",
            "profile_picture_url",
            "username",
            "website",
        ];

        let fields = field_list.iter().map(|&field| field.into()).collect();
        Self { fields }
    }
}

impl Fields {
    pub fn build_url_with_fields(self) -> String {
        let mut url_fields = "".to_string();
        let fields_count = Fields::default().fields.len();
        for (count, field) in Fields::default().fields.into_iter().enumerate() {
            if count < fields_count - 1 {
                url_fields = url_fields + &field + ",";
            } else {
                url_fields = url_fields + &field;
            }
        }
        url_fields = String::from(encode(url_fields.as_str())); // encode the url
        url_fields
    }
}
