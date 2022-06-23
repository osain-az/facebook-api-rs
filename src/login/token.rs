#![crate_name = "doc"]
#![allow(dead_code)]
//! An access token is an opaque string that identifies a user, app, or Page
//! and can be used by the app to make graph API calls.
//!
//! When someone connects with an app using Facebook Login and approves the
//! request for permissions, the app obtains an access token that provides
//! temporary, secure access to Facebook APIs. Access tokens are obtained via a
//! number of methods.
//! Form more information about token  check  [facebook api Token doc](https://developers.facebook.com/docs/facebook-login/access-tokens/?translation)
use crate::graph::client::Client;
use crate::login::config::Config;
use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// UserToken is Obtain after a successful login to Facebook.
///
/// This kind of access token is needed any time the app calls an API to read,
/// modify or write a specific person's Facebook data on their behalf. User
/// access tokens are generally obtained via a login dialog and require a person
/// to permit your app to obtain one.
///
/// The tokens are retrieved from a successful login redirect url from facebook.
///
/// The data in the redirect URL depends on the `LoginResponseType` chosen when
/// building the login url.
///
/// By default, a `LoginResponseType::CODE` is used. This means that the
/// value in response data from facebook will `code` instead of `access_token`.
/// And you will need to exchange the code for an access token.
///
/// This is recommended by Facebook to be done in server side using the
/// respective method.
///
/// # Examples
///
/// # Example for when code is the response type
///
/// * Get code at the client side.
/// ```
/// use facebook_api_rs::prelude::Config;
/// use crate::facebook_api_rs::prelude::UserToken;
///
///  let login_response_url =  "redirect_url/?#........................".to_string();
///  let user_token =   UserToken::extract_user_tokens(login_response_url);
///  // send the code to sever side to exchange for an access_token
///  let code = user_token.code;
/// ```
///  * At the server side: exchange code for access_token
/// ```
/// use crate::facebook_api_rs::prelude::{UserToken, Config};
///   
///  let code = "The code sent from client".to_string();
///  let config = Config::new("your app_id".to_owned(), "your redirect_uri".to_string());
///
///  let access_token  = UserToken::default()
///         .exchange_code_for_access_token_at_server(
///         code,
///         "your app_secret".to_string(), config);
/// ```
///
/// # Example for when token is the response type
/// When the response type is a token  instead of code, the response data will
/// be an access_token. Ans you will need to verify that at the server side.
///
/// * Get token at the client side.
/// ```
///  use crate::facebook_api_rs::prelude::{UserToken, Config};
///
///  let login_response_url =  "redirect_url/?#........................".to_string();
///  let user_token =   UserToken::extract_user_tokens(login_response_url);
/// // send the access_token to sever for verification
///  let access_token  = user_token.access_token;
/// ```
/// * At Server side: Verify the token.
/// The verification can be done by inspecting the access_token gotten from the
/// client which the response will be
/// [AccessTokenInformation](AccessTokenInformation)
/// ```    
///  use crate::facebook_api_rs::prelude::{UserToken, Config};
///  
///  let access_token_information = UserToken::access_token_information(
///        "a valid access_token".to_owned(),
///          "inspecting_token".to_owned()
///        );
/// ```
/// For information on verifying of access_token and exchanging of code check Facebook [Confirming Identity](https://developers.facebook.com/docs/facebook-login/guides/advanced/manual-flow#confirm)
#[derive(Deserialize, Default, Clone, Debug)]
pub struct UserToken {
    /// Response data is included as URL parameters and contains code parameter
    /// (an encrypted string unique to each login request). This is the default
    /// behavior if this parameter is not specified. It's most useful when your
    /// server will be handling the token.
    ///
    /// If the  login redirect url contain code then you must exchange it for
    ///  access_token. This should be done at the server.
    pub code: String,
    /// access_token is used for API calls
    ///
    /// This token is obtained either from exchange with  `code` or direct from
    /// the login redirect url. If it is obtained from client side then it
    /// should be verified at the server.
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
    ///
    /// Note: Long access token can also expire due to other reasons, to check
    /// if a given token has expired use the method :
    ///
    /// # Example
    ///
    /// ```
    /// use facebook_api_rs::prelude::UserToken;
    ///  let valid_token = "".to_owned();
    ///  let debug_token = "".to_owned(); // the token you want check or debug
    ///
    ///  UserToken::access_token_information(valid_token, debug_token)
    /// ```
    pub long_lived_token: String,

    /// A string value created by your app to maintain state between the request
    /// and callback.
    pub state: String,
}

impl UserToken {
    //@Todo: do we need this constructor ?
    pub fn new(access_token: String, long_lived_token: String) -> Self {
        UserToken {
            code: "".to_string(),
            access_token,
            data_access_expiration_time: "".to_string(),
            expires_in: "".to_string(),
            long_lived_token,
            state: "".to_string(),
        }
    }
}

impl UserToken {
    pub fn user_access_tokens(self) -> Self {
        self
    }

    /// Extract different tokens and its parameters from a
    /// successful login redirect url.
    ///
    /// # Argument
    /// * `hash` - A String of hash from the redirect url.
    ///
    /// # Example
    ///
    /// # Example for when code in the response data
    ///
    /// * Get code at the client side.
    /// ```
    /// use facebook_api_rs::prelude::Config;
    /// use crate::facebook_api_rs::prelude::UserToken;
    ///  // The redirect url after login
    ///  let login_response_url =  "redirect_url/?#........................".to_string();
    ///
    ///  let user_token =   UserToken::extract_user_tokens(login_response_url);
    ///  // Send the code to sever side to exchange for an access_token
    ///  let code  = user_token.code;
    /// ```
    ///  * At the server side: exchange code for access token
    /// ```
    /// use crate::facebook_api_rs::prelude::{UserToken, Config};
    ///   
    ///    let code = "The code sent from client".to_string();
    ///     let config = Config::new("your app_id".to_owned(), "your redirect_uri".to_string());
    ///
    ///  let access_token  = UserToken::default()
    ///         .exchange_code_for_access_token_at_server(
    ///         code,
    ///         "your app_secret".to_string(), config);
    /// ```
    ///
    /// # Example for when token is the response type
    ///
    /// When the response type is a token  instead of code, the response data
    /// will be an access_token. And you will need to verify that at the
    /// server side.
    ///
    /// * Get token at the client side.
    /// ```
    ///  use crate::facebook_api_rs::prelude::{UserToken, Config};
    ///
    ///  let login_response_url =  "redirect_url/?#........................".to_string();
    ///
    ///  let user_token =   UserToken::extract_user_tokens(login_response_url);
    /// // send the access_token to sever verification
    ///  let access_token  = user_token.access_token;
    /// ```
    ///
    /// * At Server side: verified the access_token
    ///
    /// At the server side inspect the access_token gotten from the client
    /// ```    
    ///  use crate::facebook_api_rs::prelude::{UserToken, Config};
    ///  
    ///  let access_token_information = UserToken::access_token_information(
    ///        "a valid access_token".to_owned(),
    ///          "inspecting_token".to_owned()
    ///        );
    /// ```
    /// # Panic
    /// It will panic when :
    ///  * `Login error` ->
    ///  If for any reason the login failed then the response url will contain
    /// an `error`. If the URL with an error is passed in, panic will occur
    /// with a message from the response url.
    ///
    ///  * Empty url query parameters ->  If no query parameters if found in the
    ///    url, panic will occur.

    pub fn extract_user_tokens(url: String) -> UserToken {
        let mut response = UserToken::default();
        let updated_url = url.replace("#", "");
        let query_params: HashMap<_, _> = Url::parse(&updated_url)
            .unwrap()
            .query_pairs()
            .into_owned()
            .collect();

        if query_params.is_empty() {
            panic!(
                "There was no query parameter in uri argument that was passed in. error_message: number of argument found  {:?}.   The url argument : {} ",
                query_params.len(), url
            )
        }

        for params in query_params {
            match params.0.as_str() {
                "access_token" => {
                    response.access_token = params.1;
                }
                "code" => {
                    response.code = params.1;
                }
                "data_access_expiration_time" => {
                    response.data_access_expiration_time = params.1;
                }
                "expires_in" => {
                    response.expires_in = params.1;
                }
                "long_lived_token" => {
                    response.long_lived_token = params.1;
                }
                "state" => {
                    response.state = params.1;
                }
                "error" => {
                    panic!(
                        "There was an error in login to facebook. error_message:  {}",
                        params.1
                    )
                }
                &_ => {}
            }
        }

        response
    }

    pub async fn exchange_short_live_to_long_live_token_at_server(
        self,
        short_live_token: String,
        app_secret: String,
        config: Config,
    ) -> Result<ExchangeToken, ClientErr> {
        let url = config
            .facebook_oath_url()
            .replace("dialog/oauth", "oauth/access_token")
            + "&client_id="
            + &config.client_id
            + "&client_secret="
            + &app_secret
            + "&fb_exchange_token="
            + &short_live_token
            + "&grant_type="
            + "fb_exchange_token";

        let access_token = HttpConnection::get::<ExchangeToken>(url, "".to_string()).await?;
        Ok(access_token)
    }

    pub async fn generate_app_access_token_at_server(
        self,
        short_live_token: String,
        app_secret: String,
        app_id: String,
    ) -> Result<String, ClientErr> {
        let base_url = "https://graph.facebook.com/oauth/access_token";
        let url = format!(
            "{}?client_id={}
            &client_secret={}
            &grant_type=client_credentials",
            base_url, app_id, app_secret
        );

        let access_token = HttpConnection::get::<String>(url, "".to_string()).await?;
        Ok(access_token)
    }

    /// Exchanging Code for an access_token
    ///
    /// # Argument
    ///
    /// * `code`-  A string gotten from the extracted from login redirect url
    /// * `app_secret`- The app secret from your [App Dashboard](https://developers.facebook.com/apps)
    /// * `config` - A `Config` struct

    pub async fn exchange_code_for_access_token_at_server(
        self,
        code: String,
        app_secret: String,
        config: Config,
    ) -> Result<ExchangeToken, ClientErr> {
        let url = config
            .facebook_oath_url()
            .replace("dialog/oauth", "oauth/access_token")
            + "&client_id="
            + &config.client_id
            + "&client_secret="
            + &app_secret
            + "&code="
            + &code;

        let access_token = HttpConnection::get::<ExchangeToken>(url, "".to_string()).await?;
        Ok(access_token)
    }

    /// This method will make a get request to facebook api to return
    /// information about a given token
    ///
    ///  # Arguments
    ///
    /// * `valid_access_token` - A String of a valid access token. This could be
    ///   app_token, user_token, or page_token
    /// * `debug_access_token` -  A String of the access token you intend to get
    ///   information.
    ///
    /// The response data is a struct
    /// ```
    ///  use crate::facebook_api_rs::prelude::{AccessTokenInformation};
    /// ```
    //  Note: when you try to debug a long live token, the expires_at value will
    //  be "expires_at: 0" which means it never expires for information
    /// For more information about  Facebook debug token check [facebook debug token api](https://developers.facebook.com/docs/facebook-login/access-tokens/debugging-and-error-handling)
    pub async fn access_token_information(
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

        // convert unix timestamp  date to human-readable format  and update the new
        // constructed struct
        if access_token_expiring_date != 0 {
            let token_expiring_date_utc = Utc.timestamp(access_token_expiring_date, 0);
            let token_expiring_date_local: DateTime<Local> =
                DateTime::from(token_expiring_date_utc);
            access_token_information.expires_at_local_date = token_expiring_date_local.to_rfc2822();
        } else {
            access_token_information.expires_at_local_date = access_token_expiring_date.to_string();
        }

        let token_expiring_data_time =
            Utc.timestamp(access_token_response.data.data_access_expires_at, 0);
        let token_expiring_data_time_local: DateTime<Local> =
            DateTime::from(token_expiring_data_time);

        access_token_information.data_access_expires_at_local_date =
            token_expiring_data_time_local.to_rfc2822();
        access_token_information.expires_at = access_token_response.data.expires_at;

        access_token_information.data_access_expires_at =
            access_token_response.data.data_access_expires_at;
        access_token_information.is_valid = access_token_response.data.is_valid;
        access_token_information.token_type = access_token_response.data.r#type.clone();
        access_token_information.user_id = access_token_response.data.user_id;
        access_token_information.app_id = access_token_response.data.app_id;
        access_token_information.scopes = access_token_response.data.scopes;

        Ok(access_token_information)
    }
}

// /// Extract data from the url fragment and return an `IndexMap`
// for the Enum Variant.
// # Panics
// The function will panic a key that has no value.
// # Warns
// with no query. These choices are opinionated for now.
// fn extract_query_fragments(hash: String) -> HashMap<String, String> {
// let mut query: HashMap<String, String> = HashMap::new();
//
// let key_value: Vec<&str> = hash.split('&').collect();
//
// for pair in key_value {
// let mut sub = pair.split('=');
// let key = sub.next().unwrap_or_else(|| {
// panic!(
// "we should have a key for the parameter key but got {}",
// hash
// )
// });
// let value = sub
// .next()
// .unwrap_or_else(|| panic!("we should have a value for the key but got {}",
// hash)); query.insert(key.to_string(), value.to_string());
// }
// query
// }

/// ```
/// pub struct AccessTokenInformation {
///     //Expire date in your unix time /
///    pub expires_at: u64,
///     // The type of token ( USER/PAGE
///    pub token_type: String,
///     // Expire date in your local time
///    pub expires_at_local_date: String,
///    pub is_valid: bool,
///     /// When the token can not access data anymore in unix time
///    pub data_access_expires_at: i64,
///     /// When the token can not access data anymore, in your local time,
///     pub data_access_expires_at_local_date: String,
///    pub app_id: String,
///    pub application: String,
///    pub scopes: Vec<String>,
///    pub granular_scopes: Vec<GranularScopes>,
///    pub user_id: u32,
/// }
/// ```

#[derive(Deserialize, Default, Clone, Debug, Serialize)]
pub struct AccessTokenInformation {
    /// Expire date in your unix time /
    pub expires_at: i64,
    /// The type of token ( USER/PAGE
    pub token_type: String,
    /// Expire date in your local time
    pub expires_at_local_date: String,
    pub is_valid: bool,
    /// When the token can not access data anymore in unix time
    pub data_access_expires_at: i64,
    /// When the token can not access data anymore, in your local time,
    pub data_access_expires_at_local_date: String,
    pub app_id: String,
    pub application: String,
    pub scopes: Vec<String>,
    pub granular_scopes: Vec<GranularScopes>,
    pub user_id: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TokenResponseInformation {
    data: TokenResponseData,
}

#[derive(Deserialize, Default, Clone, Debug, Serialize)]
struct TokenResponseData {
    expires_at: i64,
    r#type: String,
    is_valid: bool,
    data_access_expires_at: i64,
    app_id: String,
    application: String,
    scopes: Vec<String>,
    granular_scopes: Vec<GranularScopes>,
    user_id: String,
}

#[derive(Deserialize, Default, Clone, Debug, Serialize)]
pub struct GranularScopes {
    target_ids: Vec<String>,
    scope: String,
}

/// Enum of different types of lives of Facebook page token that a user can
/// obtain.
///
/// When obtaining a facebook page token, you can decide to obtain:
///
/// * `long live toke` - this type of token will have a lifetime of about 60
///   days.
/// * `short live toke` - this type of token will have a lifetime of about an
///   hour or two.
///
/// Note:: You should not depend on these lifetimes - the
/// lifetime may change without warning or expire early due to other reasons
/// Form more information on Token, check [facebook token guide](https://developers.facebook.com/docs/facebook-login/access-tokens/?translation)
pub enum TokenLiveType {
    LONGLIVE,
    SHORTLIVE,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExchangeToken {
    /// {access-token},
    access_token: String,
    /// {type}
    token_type: String,
    /// {seconds-til-expiration}
    expires_in: u32,
}
