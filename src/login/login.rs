#![allow(dead_code)]
//! Facebook Manual login flow.
//!
//! The login let user login into your app and obtain information about the
//! user.
//!
//! //! For details of facebook manual login flow, check [facebook doc](https://developers.facebook.com/docs/facebook-login/guides/advanced/manual-flow#login).

use crate::login::config::Config;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;

/// Parameters for build Facebook login URL.
///
/// # Example
///
/// * Using default parameters
/// ```
/// use facebook_api_rs::prelude::{Config, LoginUrlParameters, LoginResponseType};
/// let config = Config::new("your app id".to_string(), "your redirect uri".to_string());///
///
///  let login_url = LoginUrlParameters::new(config).full_login_url();
/// ```
/// * Adding custom parameters
/// ```
/// use facebook_api_rs::prelude::{Config, LoginUrlParameters, LoginResponseType};
/// let config = Config::new("your app id".to_string(), "your redirect uri".to_string());
///
///   let login_url = LoginUrlParameters::new(config)
///        .add_state("your state")
///         .add_response_type(LoginResponseType::TOKEN)
///         .add_scope(vec!["email".to_owned()])
///         .full_login_url();
/// ```
///
///  # Canceled Login
///
/// If people using your app don't accept the Login dialog and clicks
/// Cancel, they'll be redirected to the following:
///
/// ```
/// YOUR_REDIRECT_URI?
///  error_reason=user_denied
///  &error=access_denied
///  &error_description=Permissions+error
/// ```
#[derive(Debug, Default, Clone, Serialize)]
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

    /// Determines whether the response data included in the redirect url  to
    /// the app occurs is in URL parameters or fragments.
    ///
    /// The Enum of the different types response type can be found at
    ///
    /// ```
    /// use crate::facebook_api_rs::prelude::LoginResponseType;
    /// ```
    response_type: String,

    /// A comma separated list of Permissions to request from the
    ///   person using your app. To check [permission list](https://developers.facebook.com/docs/permissions/reference)
    scope: Vec<&'static str>,
    scope_as_string: String,

    /// The full url of the login flow.
    full_url: String,
}

impl LoginUrlParameters {
    /// Constructor for Facebook login url parameters
    ///
    /// The constructor accept a config  struct  with facebook_oath_url,
    /// client_id, and redirect_uri.
    ///
    /// # optional parameters
    ///
    /// * `state` - By default, a random state will be generated or the user may
    ///   choose to create their own state.
    ///
    /// * `response_type` has to be configured depending on the use case
    /// of the application. By default, it uses code as the response type.
    ///
    /// To determine which response type to used, check facebook guide on [Confirming Identity](https://developers.facebook.com/docs/facebook-login/guides/advanced/manual-flow#confirm)
    ///
    /// * `Scope` -  A comma separated list of Permissions to request from the
    ///   person using your app. To check [permission list](https://developers.facebook.com/docs/permissions/reference)
    ///
    /// # Example
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Config, LoginUrlParameters, LoginResponseType};
    /// let config = Config::new("your app id".to_string(), "your redirect uri".to_string());
    ///
    ///  // Using default parameters
    ///  let login_url = LoginUrlParameters::new(config).full_login_url();
    ///
    /// // Override default parameters
    ///   let login_url = LoginUrlParameters::new(config)
    ///        .add_state("your state")
    ///         .add_response_type(LoginResponseType::TOKEN)
    ///         .add_scope(vec!["email"])
    ///         .full_login_url();
    /// ```
    ///
    ///  # Canceled Login
    ///
    /// If people using your app don't accept the Login dialog and clicks
    /// Cancel, they'll be redirected to the following:
    ///
    /// ```
    ///  use crate::facebook_api_rs::prelude::{UserToken, Config};
    /// let response_url = "YOUR_REDIRECT_URI?
    ///  error_reason=user_denied
    ///  &error=access_denied
    ///  &error_description=Permissions+error";
    ///
    /// // Check if the response url contain an error and Capture it.
    ///   if response_url.contains("error"){
    ///   let login_error = UserToken::extract_user_tokens(login_response_url).login_error;
    ///     };
    ///     // Or we don`t have to check
    ///   let user_token = UserToken::extract_user_tokens(login_response_url);
    ///     // check if the token has an error
    ///     if let Some(login_error) = user_token.login_error{
    ///      // handle error
    ///    };
    /// ```
    ///
    /// # Re-asking for Declined Permissions
    /// Facebook Login lets people decline sharing some permissions with your
    /// app. If they declined any permissions, they wont be asked again.
    ///
    /// This is because once someone has declined a permission, the Login Dialog
    /// will not re-ask them for it unless you explicitly tell the dialog you're
    /// re-asking for a declined permission.
    ///
    /// To re.request for declined permission:
    /// ```
    ///   let login_url = LoginUrlParameters::new(config)
    ///        .add_state("your state")
    ///         .add_response_type(LoginResponseType::TOKEN)
    ///         .add_scope(vec!["email".to_owned()])
    ///         .re_request_permission__url();
    /// ```
    pub fn new(config: Config) -> LoginUrlParameters {
        LoginUrlParameters::default()
            .add_facebook_oath_url(&config.facebook_oath_url())
            .add_client_id(&config.client_id())
            .add_redirect_uri(&config.redirect_uri())
            .add_random_state()
            .add_response_type(LoginResponseType::CODE)
            .add_scope([""].to_vec())
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

    /// A string value created by your app to maintain state between the request
    /// and callback. This parameter should be used for preventing [Cross-site
    /// Request Forgery](https://en.wikipedia.org/wiki/Cross-site_request_forgery) and will be passed back to you, unchanged, in your
    /// redirect URI
    pub fn add_state(mut self, state: &str) -> Self {
        self.state = state.to_string();
        self
    }

    /// Determines data included in the response url.
    ///    
    /// To determine which response type to used, check facebook guide on [Confirming Identity](https://developers.facebook.com/docs/facebook-login/guides/advanced/manual-flow#confirm)
    ///
    /// # Example
    ///  ```
    /// use facebook_api_rs::prelude::{Config, LoginUrlParameters,
    /// LoginResponseType}; let config = Config::new("your app
    /// id".to_string(), "your redirect url".to_string());
    ///
    /// // Override default parameters
    ///   let login_url = LoginUrlParameters::new(config)
    ///        .add_state("your state")
    ///         .add_response_type(LoginResponseType::TOKEN)
    ///         .add_scope(vec!["email"].to_vec())
    ///         .full_login_url();
    /// ```
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

    /// A comma separated list of Permissions to request from the
    /// person using your app. To check [permission list](https://developers.facebook.com/docs/permissions/reference)
    pub fn add_scope(mut self, scopes: Vec<&'static str>) -> Self {
        let scope_count = scopes.len();
        self.scope = scopes;
        let mut loop_count = 1;
        let mut scopes_string = "".to_owned();

        for scope_ in &self.scope {
            if scope_count == loop_count {
                scopes_string += &*format!("{scope_}")
            } else {
                scopes_string += &*format!("{scope_},")
            }
            loop_count += 1;
        }

        self.scope_as_string = scopes_string;
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
            + &self.scope_as_string;
        self.full_url = full_url.clone();
        full_url
    }

    fn build_re_request_permission_url(&mut self) -> String {
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
            + "&auth_type=rerequest"
            + "&scope="
            + &self.scope_as_string;
        self.full_url = full_url.clone();
        full_url
    }

    /// Re-authentication enables your app to confirm a person's identity even
    /// if it was verified previously.
    fn build_enabling_re_authentication_url(&mut self) -> String {
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
            + "&auth_type=reauthenticate"
            + "&scope="
            + &self.scope_as_string;
        self.full_url = full_url.clone();
        full_url
    }

    pub fn full_login_url(mut self) -> String {
        self.full_url = self.build_login_url_as_string();
        self.full_url
    }

    pub fn re_request_permission_url(mut self) -> String {
        self.full_url = self.build_re_request_permission_url();
        self.full_url
    }

    /// Re-authentication enables your app to confirm a person's identity even
    /// if it was verified previously.
    pub fn re_authentication_url(mut self) -> String {
        self.full_url = self.build_enabling_re_authentication_url();
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

    pub fn scope(&self) -> &Vec<&'static str> {
        &self.scope
    }
}

/// Enum that determines the response data included in redirect url after
/// successfully.
/// [Confirming Identity](https://developers.facebook.com/docs/facebook-login/guides/advanced/manual-flow#confirm)
pub enum LoginResponseType {
    /// The parameters in response URL should contain a code
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
            facebook_oath_url: "https://www.facebook.com/v23.0/dialog/oauth?".to_string(),
            client_id: "1234567890".to_string(),
            redirect_uri: "http://localhost:8001".to_string(),
        })
        .add_response_type(LoginResponseType::TOKEN)
        .add_state("0987654321")
        .add_scope(vec!["test"].to_vec());

        assert_eq!(
            redirect_url.facebook_oath_url(),
            "https://www.facebook.com/v23.0/dialog/oauth?"
        );
        assert_eq!(redirect_url.client_id(), "1234567890");
        assert_eq!(redirect_url.redirect_uri(), "http://localhost:8001");
        assert_eq!(redirect_url.state(), "0987654321");
        assert_eq!(redirect_url.response_type(), "token");

        let scope = ["test".to_string()].to_vec();
        assert_eq!(redirect_url.scope(), &scope);

        let full_url = redirect_url.full_login_url();
        assert_eq!(full_url, "https://www.facebook.com/v23.0/dialog/oauth?client_id=1234567890&redirect_uri=http://localhost:8001&response_type=token&state=0987654321&scope=test")
    }
}
