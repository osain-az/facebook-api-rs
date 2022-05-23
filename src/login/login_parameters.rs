#![allow(dead_code)]

use crate::login::config::Config;
use crate::universal::client::HttpConnection;
use crate::universal::errors::ClientErr;

use async_trait::async_trait;
// use crate::universal::reqwest::ReqwestClient;
use crate::universal::response::{deserialize_response, ClientResult};
// use crate::universal::seed_client::SeedClient;
use crate::universal::HttpClient;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug, Default, Clone, Serialize)]
pub struct LoginParameters {
    /// The Facebook url preamble for the oath dialog.
    facebook_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    client_id: String,

    /// The URL that you want to redirect the person logging in back to.
    redirect_uri: String,

    /// A string value created by your app to maintain state between the request
    /// and callback.
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

impl LoginParameters {
    /// Constructor of the Facebook login url parameters
    ///
    /// facebook_oath_url, client_id, and redirect_uri are retrieved from the
    /// config.json file. which the user has to configure.
    /// A random state is provided or the user may chose to create their own
    /// state. response_type has to be configured depending on the use case
    /// of the application, or else the response will default to code upon
    /// the login flow redirect. scope is optional, but inclusion must
    /// fulfill a valid scope.
    pub fn new(config: Config) -> LoginParameters {
        LoginParameters::default()
            .add_facebook_oath_url(&config.facebook_oath_url())
            .add_client_id(&config.client_id())
            .add_redirect_uri(&config.redirect_uri())
            .add_random_state()
            .add_response_type("")
            //MUST ADD A VALID SCOPE!
            .add_scope(config.scope)
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

    pub fn add_response_type(mut self, response_type: &str) -> Self {
        self.response_type = response_type.to_string();
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

#[cfg(test)]
mod tests {
    use crate::login::config::Config;
    use crate::login::login_parameters::LoginParameters;

    #[test]
    fn test_build_url() {
        let redirect_url = LoginParameters::new(Config {
            facebook_oath_url: "https://www.facebook.com/v11.0/dialog/oauth?".to_string(),
            client_id: "1234567890".to_string(),
            redirect_uri: "http://localhost:8001".to_string(),
            scope: vec![],
        })
        .add_response_type("token")
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
