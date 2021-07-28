use crate::extract_query_fragments::extract_query_fragments;
use crate::token::Token;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use seed::Url;
use serde::{Deserialize, Serialize};
///A struct which describes the config.json file structure.
/// the json file fields are stored in this struct, and are then
/// added to the RedirectURL struct.
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// The Facebook url preamble for the oath dialog.
    facebook_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    client_id: String,

    /// The URL that you want to redirect the person logging in back to.
    redirect_uri: String,
}

///Contains the Config struct and is used for building the login flow
///
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct RedirectURL {
    /// The Facebook url preamble for the oath dialog.
    facebook_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    client_id: String,

    /// The URL that you want to redirect the person logging in back to.
    redirect_uri: String,

    /// A string value created by your app to maintain state between the request and callback.
    state: String,

    /// Determines whether the response data included when the redirect back to the app occurs is in URL parameters or fragments.
    response_type: String,

    /// A comma or space separated list of Permissions to request from the person.
    scope: Vec<String>,

    /// The full url of the login flow.
    full_url: String,
}

impl RedirectURL {
    /// Constructor of the RedirectURL
    /// facebook_oath_url, client_id, and redirect_uri are retrieved from the config.json file.
    /// which the user has to configure.
    /// A random state is provided or the user may chose to create their own state.
    /// response_type has to be configured depending on the use case of the application, or else the response
    /// will default to code upon the login flow redirect.
    /// scope is optional, but inclusion must fulfill a valid scope.
    pub fn new(config: Config) -> RedirectURL {
        RedirectURL::default()
            .add_facebook_oath_url(&config.facebook_oath_url)
            .add_client_id(&config.client_id)
            .add_redirect_uri(&config.redirect_uri)
            .add_random_state()
            .add_response_type("")
            //MUST ADD A VALID SCOPE!
            .add_scope(&[])
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

    pub fn add_scope(mut self, scope: &[String]) -> Self {
        self.scope = scope.to_vec();
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

    ///Builds the redirect url for the login flow as a string so it may be passed thorugh a GET request
    pub fn build_redirect_url_as_string(&mut self) -> String {
        let full_url = "".to_string()
            + &self.facebook_oath_url
            + &"client_id=".to_string()
            + &self.client_id
            + "&redirect_uri="
            + &self.redirect_uri
            + "&response_type="
            + &self.response_type
            + "&state="
            + &*self.state
            + "&scope="
            + &self.scope.iter().cloned().collect::<String>();
        full_url
    }

    pub fn add_full_url(mut self) -> Self {
        self.full_url = self.build_redirect_url_as_string();
        self
    }

    pub fn get_facebook_oath_url(&self) -> &String {
        &self.facebook_oath_url
    }
    pub fn get_client_id(&self) -> &String {
        &self.client_id
    }
    pub fn get_redirect_uri(&self) -> &String {
        &self.redirect_uri
    }
    pub fn get_state(&self) -> &String {
        &self.state
    }
    pub fn get_response_type(&self) -> &String {
        &self.response_type
    }
    pub fn get_scope(&self) -> &Vec<String> {
        &self.scope
    }
    pub fn get_full_url(&self) -> &String {
        &self.full_url
    }

    fn set_facebook_oath_url(&mut self, facebook_oath_url: String) {
        self.facebook_oath_url = facebook_oath_url;
    }
    fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }
    fn set_redirect_uri(&mut self, redirect_uri: String) {
        self.redirect_uri = redirect_uri;
    }
    fn set_state(&mut self, state: String) {
        self.state = state;
    }
    fn set_response_type(&mut self, response_type: String) {
        self.response_type = response_type;
    }
    fn set_scope(&mut self, scope: Vec<String>) {
        self.scope = scope;
    }
}
