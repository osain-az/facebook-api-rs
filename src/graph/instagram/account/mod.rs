//! This Api is used to get instagram business account for any given facebook page
//! <https://developers.facebook.com/docs/instagram-api/reference/page>

use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use seed::prelude::js_sys::encode_uri;
use urlencoding::encode;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct InstaAccount {
    pub instagram_business_account: InstaAccountId,
    pub id: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]

pub struct InstaAccountId {
    pub id: String,
}

#[derive( Clone)]
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

    /// This method is use to get instagram business account id.
    /// for reference check <https://developers.facebook.com/docs/instagram-api/reference/page>
    pub async fn account_id(self) -> seed::fetch::Result<InstaAccount> {
        let base_url = self.base_url.replace("EDGE", "?");
        let url = base_url
            + "fields=instagram_business_account"
            + "&access_token="
            + &self.page_access_token;

        let request = Request::new(url).method(Method::Get);
        fetch(request).await?.json::<InstaAccount>().await
    }

    /// This method is used to get instagram business account with its details (name, user, id ,etc).
    /// It accepts the instagram page id.
    /// for reference check <https://developers.facebook.com/docs/instagram-api/reference/ig-user>
    pub async fn account_details(self) -> seed::fetch::Result<InstagramAccount> {
              log!("the base url ", self.base_url);

            let mut url = self.base_url.replace("EDGE", "?");

            // add required filed needed to be returned
            let  fields_count = Fields::default().fields.len();
             let mut url_fields = "".to_string();

            for (count, field) in Fields::default().fields.into_iter().enumerate() {
                if count < fields_count - 1 {
                    url_fields = url_fields+ &field + ",";
                } else {
                    url_fields= url_fields+ &field; // remove the comma in the last filed
                }
            }
            url_fields = String::from(encode(url_fields.as_str())); // encode the url

             let mut request_url = url
                 + "fields="
                 +url_fields.as_str()
                + "&access_token="
                + &self.page_access_token;
            let request = Request::new(request_url).method(Method::Get);
           fetch(request).await?.json::<InstagramAccount>().await
    }
}


#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct InstagramAccount {
    //https://developers.facebook.com/docs/instagram-api/reference/ig-user/
    //This is a public fields, which means it can be returned if available.
  //  biography: String,

    ///This is a public fields, which means it can be returned if available.
  pub  id:  String,

    ///This is not a public fields, which means it may be returned depending on the user setting.
    pub  ig_id: u32,

    ///This is a public fields, which means it can be returned if available.
    pub  followers_count:  u32,

    ///This is not a public fields, which means it may be returned depending on the user setting.
    pub  follows_count : u32,

    ///This is not a public fields, which means it may be returned depending on the user setting.
    pub   media_count: u32,

    ///This is not a public fields, which means it may be returned depending on the user setting.
    pub   name: String,

    //This is not a public fields, which means it may be returned depending on the user setting.
  //  profile_picture_url:  String,

    ///This is not a public fields, which means it may be returned depending on the user setting.
    pub  username : String,

    //This is not a public fields, which means it may be returned depending on the user setting.
   // website :  String,
}
pub struct Fields {
    pub(crate) fields: Vec<String>,
}

impl Default for Fields {
    /// These parameters are used as fields which are passed in as a query
    /// parameters to the get post request and feeds request
    fn default() -> Self {
        let field_list = vec![
            "biography",
            "id",
            "ig_id",
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
