use crate::graph::client::Client;
use seed::prelude::IndexMap;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

use seed::fetch::fetch;

/// The following struct is used to describe a token which may be retrieved from
/// the login flow of Facebook.
#[derive(Deserialize, Default, Clone, Debug, Serialize)]
pub struct Token {
    /// access_token is used for API calls and it contains response data such as
    /// scopes
    pub access_token: String,

    /// Expires in 90 days based when the user was last active
    /// When this 90-day period expires, the user can still access your app —
    /// that is, they are still authenticated — but your app can't access
    /// their data. To regain data access, your app must ask the user to
    /// re-authorize your app's permissions.
    pub data_access_expiration_time: String,

    /// The expiration time of the Access Token
    pub expires_in: String,

    /// A long-lived token generally lasts about 60 days
    /// These tokens are refreshed once per day, when the person using your app
    /// makes a request to Facebook's servers. If no requests are made, the
    /// token will expire after about 60 days and the person will have to go
    /// through the login flow again to get a new token.
    pub long_lived_token: String,

    /// A string value created by your app to maintain state between the request
    /// and callback.
    pub state: String,
}

impl Token {
    pub fn user_access_tokens(self) -> Self {
        self
    }

    /// Gets  Token from the current URL by extracting the query query of
    /// the URL
    pub fn extract_user_tokens(hash: String) -> Token {
        let query = extract_query_fragments(hash);

        let iterations = query.iter();

        let mut response = Token::default();

        for e in iterations {
            match e.0.as_str() {
                "access_token" => {
                    response.access_token = e.1.to_string();
                }
                "data_access_expiration_time" => {
                    response.data_access_expiration_time = e.1.to_string();
                }
                "expires_in" => {
                    response.expires_in = e.1.to_string();
                }
                "long_lived_token" => {
                    response.long_lived_token = e.1.to_string();
                }
                "state" => {
                    response.state = e.1.to_string();
                }
                _ => panic!("unknown field: {}", e.0.as_str()),
            }
        }
        response
    }

    /// this method is used to get information regarding a given token,
    /// it accepts a valid access token and the toekn you intend to get
    /// information, for more inform  check facbook deocumentation https://developers.facebook.com/docs/facebook-login/access-tokens/debugging-and-error-handling
    pub async fn access_token_information(
        self,
        valid_access_token: String,
        long_live_token: String,
    ) -> seed::fetch::Result<AccessTokenInformation> {
        // https://developers.facebook.com/docs/facebook-login/access-tokens/debugging-and-error-handling
        let url = "https://graph.facebook.com/debug_token?".to_owned()
            + "input_token="
            + &long_live_token
            + "&access_token="
            + &valid_access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<AccessTokenInformation>().await
        // let test = result.as_ref()

        //  let response = test.unwrap().expires_at.to_owned();
        //   let expired_time = response / (1.12_f32.powf(8.0)) as u64;
        //  log!("expired_time", expired_time);
        //  result
    }

    // also the need to hanlde
}

/// Extract data from  from the url fragment and return an `IndexMap`
/// for the Enum Variant.
/// # Panics
/// The function will panic a key that has no value.
/// # Warns
/// with no query. Theses choices are opinionated for now.
pub fn extract_query_fragments(hash: String) -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();

    let key_value: Vec<&str> = hash.split('&').collect();

    for pair in key_value {
        let mut sub = pair.split('=');
        let key = sub.next().unwrap_or_else(|| {
            panic!(
                "we should have a key for the parameter key but got {}",
                hash
            )
        });
        let value = sub
            .next()
            .unwrap_or_else(|| panic!("we should have a value for the key but got {}", hash));
        query.insert(key.to_string(), value.to_string());
    }
    query
}

#[derive(Deserialize, Copy, Default, Clone, Debug, Serialize)]
pub struct AccessTokenInformation {
    data: ResponseData,
}

#[derive(Deserialize, Copy, Default, Clone, Debug, Serialize)]

pub struct ResponseData {
    pub expires_at: u64,
    pub is_valid: bool,
}
impl AccessTokenInformation {
    /// this method is used to get information regarding a given token,
    /// it accepts a valid access token and the toekn you intend to get
    /// information, for more inform  check facbook deocumentation https://developers.facebook.com/docs/facebook-login/access-tokens/debugging-and-error-handling
    ///                                                                         
    pub async fn access_token_information(
        access_token: String,
        long_live_token: String,
    ) -> seed::fetch::Result<AccessTokenInformation> {
        let url = "https://graph.facebook.com/debug_token?".to_owned()
            + "input_token="
            + &long_live_token
            + "&access_token="
            + &access_token;
        let request = Request::new(url).method(Method::Get);
        let result = fetch(request).await?.json::<AccessTokenInformation>().await;
        let test = result.as_ref();

        let response = test.unwrap().data.expires_at.to_owned();
        let expired_time = response / (1.12_f32.powf(8.0)) as u64;

        // Note: time is in unix time sampe
        log!("expired_time", expired_time);
        result
    }
}
