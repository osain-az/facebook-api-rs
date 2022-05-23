#![crate_name = "doc"]

//! An access token is an opaque string that identifies a user, app, or Page
//! and can be used by the app to make graph API calls.
//!
//! When someone connects with an app using Facebook Login and approves the
//! request for permissions, the app obtains an access token that provides
//! temporary, secure access to Facebook APIs. Access tokens are obtained via a
//! number of methods.
//! Form more information about token  check  [facebook api Token doc](https://developers.facebook.com/docs/facebook-login/access-tokens/?translation)
use crate::graph::client::Client;
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use indexmap::IndexMap;

use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};

/// The following struct is used to describe a token which may be retrieved from
/// the login flow of Facebook.
#[derive(Deserialize, Default, Clone, Debug)]
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
    pub fn new(access_token: String, long_lived_token: String) -> Self {
        Token {
            access_token,
            data_access_expiration_time: "".to_string(),
            expires_in: "".to_string(),
            long_lived_token,
            state: "".to_string(),
        }
    }
}

impl Token {
    pub fn user_access_tokens(self) -> Self {
        self
    }

    /// Method that extract different tokens and its parameters  from a
    /// successful login url from facebook.
    ///
    /// Token from the current URL by extracting the query query of
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

    /// This method will make a get request to facebook api to return
    /// information about a given token
    ///
    ///  # Arguments
    ///
    /// * `valid_access_token` - A String of a valid access token,
    /// * `debug_access_token` -  A String of the access token you intend to get
    ///   information.
    ///
    /// The response data are struct
    /// ```rust
    ///  struct AccessTokenInformation {
    ///   //Expire date in your unix time
    ///     expires_at: u64,
    ///    // The type of token ( USER/PAGE
    ///     token_type: String,
    ///    // Expire date in your lcal time
    ///     expires_at_local_date: String,
    ///     is_valid: bool,
    ///    //when the token can not access data anymore in unix time
    ///     data_access_expires_at: u64,
    ///   //when the token can not access data anymore, in your local time
    ///    pub data_access_expires_at_local_date: String,    ///
    /// }
    /// ```
    /// Note: when you try to debug a long live token, the expires_at value will
    /// be "expires_at: 0" which means it never expires for information
    ///
    /// For more information about  Facebook debung token check [facebook debug token api](https://developers.facebook.com/docs/facebook-login/access-tokens/debugging-and-error-handling)
    pub async fn access_token_information(
        self,
        valid_access_token: String,
        debug_access_token: String,
    ) -> Result<AccessTokenInformation, ClientErr> {
        let url = "https://graph.facebook.com/debug_token?".to_owned()
            + "input_token="
            + &debug_access_token
            + "&access_token="
            + &valid_access_token;

        let access_token_response =
            HttpConnection::get::<TokenResponseInformation>(url, "".to_string()).await?;
        let access_token_expiring_date = access_token_response.data.expires_at.to_owned();
        let mut access_token_information = AccessTokenInformation::default();

        // convert unix timestamp  date to human readable format  and update the new
        // constructed struct
        if access_token_expiring_date != 0 {
            let token_expiring_date_utc = Utc.timestamp(access_token_expiring_date as i64, 0);
            let token_expiring_date_local: DateTime<Local> =
                DateTime::from(token_expiring_date_utc);
            access_token_information.expires_at_local_date = token_expiring_date_local.to_rfc2822();
        } else {
            access_token_information.expires_at_local_date = access_token_expiring_date.to_string();
        }

        let token_expiring_data_time =
            Utc.timestamp(access_token_response.data.data_access_expires_at as i64, 0);
        let token_expiring_data_time_local: DateTime<Local> =
            DateTime::from(token_expiring_data_time);

        access_token_information.data_access_expires_at_local_date =
            token_expiring_data_time_local.to_rfc2822();
        access_token_information.expires_at = access_token_response.data.expires_at;

        access_token_information.data_access_expires_at =
            access_token_response.data.data_access_expires_at;
        access_token_information.is_valid = access_token_response.data.is_valid;
        access_token_information.token_type = access_token_response.data.r#type.clone();

        Ok(access_token_information)
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

#[derive(Deserialize, Default, Clone, Debug, Serialize)]
pub struct AccessTokenInformation {
    // Expire date in your unix time /
    expires_at: u64,
    // The type of token ( USER/PAGE
    token_type: String,
    // Expire date in your lcal time
    expires_at_local_date: String,
    is_valid: bool,
    // when the token can not access data anymore in unix time
    data_access_expires_at: u64,
    // when the token can not access data anymore, in your local time/    pub
    // data_access_expires_at_local_date: String,    /// in your local time
    pub data_access_expires_at_local_date: String,
}

#[derive(Deserialize, Default, Clone, Debug, Serialize)]
struct TokenResponseInformation {
    data: TokenResponseData,
}

#[derive(Deserialize, Default, Clone, Debug, Serialize)]
struct TokenResponseData {
    expires_at: u64,
    r#type: String,
    is_valid: bool,
    data_access_expires_at: u64,
}

/// Enums of different types of lives of Facebook page token that a user can
/// obtain.
///
/// When obtaining a facebook page token, you can decide to obtain:
///
/// * `long live toke` - this type of token will have a lifetime of about 60
///   days.
/// * `short live toke` - this type of token will have a lifetime of about an
///   hour or two.
///
/// Note:: You should not depend on these lifetimes remaining the same - the
/// lifetime may change without warning or expire early.
/// Form more information on Token, check [facebook token guide](https://developers.facebook.com/docs/facebook-login/access-tokens/?translation)
pub enum TokenLiveType {
    LONGLIVE,
    SHORTLIVE,
}
