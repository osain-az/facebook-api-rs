#![allow(dead_code)]
//! Facebook Manual login flow.
//!
//! The login let user login into your app and obtain information about the
//! user.
//!
//! //! For details of facebook manual login flow, check [facebook doc](https://developers.facebook.com/docs/facebook-login/guides/advanced/manual-flow#login).

use crate::login::config::Config;
use crate::universal::client::HttpConnection;
use crate::universal::errors::ClientErr;

use crate::prelude::ResponseType;
use crate::universal::HttpClient;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug, Default, Clone, Serialize)]
pub struct LoginUrlParameters {
    /// The Facebook url preamble for the oath dialog.
    facebook_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    client_id: String,

    /// The URL that you want to redirect the person logging in back to.
    redirect_uri: String,

    /// A string value created by your app to maintain state between the request
    /// and callback. This parameter should be used for preventing [Cross-site
    /// Request Forgery](https://en.wikipedia.org/wiki/Cross-site_request_forgery) and will be passed back to you, unchanged, in your
    /// redirect URI.
    state: String,

    /// Determines whether the response data included when the redirect back to
    /// the app occurs is in URL parameters or fragments.
    response_type: String,

    /// A comma or space separated list of Permissions to request from the
    /// person.
    scope: Vec<String>,

    /// The full url of the login flow.
    full_url: String,
}

impl LoginUrlParameters {
    /// Constructor of the Facebook login url parameters
    ///
    /// The constructor accept a config  struct  with facebook_oath_url,
    /// client_id, and redirect_uri.
    ///
    /// Other optional parameters are:
    ///
    /// * `state` - By default a random state will be generated or the user may
    ///   chose to create their own
    /// state.
    /// * `response_type` has to be configured depending on the use case
    /// of the application, or else the response will default to code upon
    /// the login flow redirect. scope is optional, but inclusion must
    /// fulfill a valid scope.
    pub fn new(config: Config) -> LoginUrlParameters {
        LoginUrlParameters::default()
            .add_facebook_oath_url(&config.facebook_oath_url())
            .add_client_id(&config.client_id())
            .add_redirect_uri(&config.redirect_uri())
            .add_random_state()
            .add_response_type(LoginResponseType::CODE)
            .add_scope(["".to_owned()].to_vec())
    }

    pub fn add_client_id(mut self, client_id: &str) -> Self {
        self.client_id = client_id.to_string();
        self
    }

    pub fn add_facebook_oath_url(mut self, url: &str) -> Self {
        self.facebook_oath_url = url.to_string();
        self
    }

    pub fn add_redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.redirect_uri = redirect_uri.to_string();
        self
    }

    pub fn add_state(mut self, state: &str) -> Self {
        self.state = state.to_string();
        self
    }

    pub fn add_response_type(mut self, response_type: LoginResponseType) -> Self {
        let resp_type = match response_type {
            LoginResponseType::CODE20TOKEN => "code%20token".to_string(),
            LoginResponseType::TOKEN => "token".to_string(),
            LoginResponseType::CODE => "code".to_string(),
            LoginResponseType::GRANTEDSCOPE => "granted_scopes".to_string(),
        };
        self.response_type = resp_type;
        self
    }

    pub fn add_scope(mut self, scope: Vec<String>) -> Self {
        self.scope = scope;
        self
    }

    pub fn add_random_state(mut self) -> Self {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        self.state = rand_string;
        self
    }

    /// Builds the redirect url for the login flow as a string so it may be
    /// passed through a GET request
    fn build_login_url_as_string(&mut self) -> String {
        let full_url = "".to_string()
            + &self.facebook_oath_url
            + "client_id="
            + &self.client_id
            + "&redirect_uri="
            + &self.redirect_uri
            + "&response_type="
            + &self.response_type
            + "&state="
            + &*self.state
            + "&scope="
            + &self.scope.iter().cloned().collect::<String>();
        self.full_url = full_url.clone();
        full_url
    }

    pub fn full_login_url(mut self) -> String {
        self.full_url = self.build_login_url_as_string();
        self.full_url
    }

    pub fn facebook_oath_url(&self) -> &String {
        &self.facebook_oath_url
    }

    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    pub fn redirect_uri(&self) -> &String {
        &self.redirect_uri
    }

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn response_type(&self) -> &String {
        &self.response_type
    }

    pub fn scope(&self) -> &Vec<String> {
        &self.scope
    }
}

/// Determines the response data included in redirect url after successfull
pub enum LoginResponseType {
    /// Response data is included as URL parameters and contains code parameter
    /// (an encrypted string unique to each login request). This is the
    /// default behavior
    CODE,
    /// Response data is included as a URL fragment and contains an access
    /// token. Desktop apps must use this setting for response_type. This is
    /// most useful when the client will be handling the token.
    TOKEN,
    /// Response data is included as a URL fragment and contains both an access
    /// token and the code parameter.
    CODE20TOKEN,
    /// A comma-separated list of all Permissions granted to the app by the user
    /// at the time of login. Can be combined with other response_type
    /// values. When combined with token, response data is included as a URL
    /// fragment, otherwise included as a URL parameter.
    GRANTEDSCOPE,
}

#[cfg(test)]
mod tests {
    use crate::login::config::Config;
    use crate::login::login::LoginUrlParameters;
    use crate::prelude::{LoginResponseType, ResponseType};

    #[test]
    fn test_build_url() {
        let redirect_url = LoginUrlParameters::new(Config {
            facebook_oath_url: "https://www.facebook.com/v11.0/dialog/oauth?".to_string(),
            client_id: "1234567890".to_string(),
            redirect_uri: "http://localhost:8001".to_string(),
        })
        .add_response_type(LoginResponseType::TOKEN)
        .add_state("0987654321")
        .add_scope(&["test".to_string()])
        .full_login_url();

        assert_eq!(
            redirect_url.facebook_oath_url,
            "https://www.facebook.com/v11.0/dialog/oauth?"
        );
        assert_eq!(redirect_url.client_id, "1234567890");
        assert_eq!(redirect_url.redirect_uri, "http://localhost:8001");
        assert_eq!(redirect_url.state, "0987654321");
        assert_eq!(redirect_url.response_type, "token");

        let scope = &["test".to_string()];
        assert_eq!(redirect_url.scope, scope);

        assert_eq!(redirect_url.full_url, "https://www.facebook.com/v11.0/dialog/oauth?client_id=1234567890&redirect_uri=http://localhost:8001&response_type=token&state=0987654321&scope=test")
    }
}
