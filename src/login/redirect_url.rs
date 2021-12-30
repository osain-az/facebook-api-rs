#![allow(dead_code)]

use std::sync::Arc;
use crate::login::config::Config;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use seed::fetch::StatusCategory::ClientError;
use serde::{Deserialize, Serialize};
use crate::universal::HttpClient;
use crate::universal::errors::ClientErr;
use crate::universal::response::{ClientResult, deserialize_response};

/// Contains the Config struct and is used for building the login flow
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct RedirectURL<HttpC:HttpClient> {
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

    //this will be used for the http_clint
  pub  http_client: Arc<HttpC>
}

impl < HttpC:HttpClient + std::default::Default> RedirectURL<HttpC> {
    /// Constructor of the RedirectURL
    /// facebook_oath_url, client_id, and redirect_uri are retrieved from the
    /// config.json file. which the user has to configure.
    /// A random state is provided or the user may chose to create their own
    /// state. response_type has to be configured depending on the use case
    /// of the application, or else the response will default to code upon
    /// the login flow redirect. scope is optional, but inclusion must
    /// fulfill a valid scope.
    pub fn new(config: Config) -> RedirectURL<HttpC> {
        RedirectURL::default()
            .add_facebook_oath_url(&config.facebook_oath_url())
            .add_client_id(&config.client_id())
            .add_redirect_uri(&config.redirect_uri())
            .add_random_state()
            .add_response_type("")
            //MUST ADD A VALID SCOPE!
            .add_scope(&[])
            .add_full_url()
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

    /// Builds the redirect url for the login flow as a string so it may be
    /// passed through a GET request
    pub fn build_redirect_url_as_string(&mut self) -> String {
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

    pub fn add_full_url(mut self) -> Self {
        self.full_url = self.build_redirect_url_as_string();
        self
    }

    //Todo::this method is under experimental
  pub async fn login (&self)  -> Result<TestResponse, ClientErr>{
    //  let client  = HttpClient::new(None)?;
        let resp = self.http_client.get((&self.full_url.to_string()).parse().unwrap(), "").await?;
          let result  :ClientResult<TestResponse> = deserialize_response(resp.body())?;
      /*  if(self.full_url.is_empty()){
           Err(ClientErr::FacebookError("build the url before calling the login method".to_string()))
       }else {
           let resp = self.http_client.get((&self.full_url.to_string()).parse().unwrap(), "");
          Ok(())
       }*/
        Ok(result.unwrap())
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

#[derive(Debug, Deserialize)]
pub struct  TestResponse {
   pub  ok: String
}
#[cfg(test)]
mod tests {
    use crate::login::config::Config;
    use crate::login::redirect_url::RedirectURL;

    #[test]
    fn test_build_url() {
        let redirect_url = RedirectURL::new(Config {
            facebook_oath_url: "https://www.facebook.com/v11.0/dialog/oauth?".to_string(),
            client_id: "1234567890".to_string(),
            redirect_uri: "http://localhost:8001".to_string(),
        })
        .add_response_type("token")
        .add_state("0987654321")
        .add_scope(&["test".to_string()])
        .add_full_url();

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
