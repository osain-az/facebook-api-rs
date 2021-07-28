use crate::extract_query_fragments::extract_query_fragments;
use seed::Url;

///The following struct is used to describe a token which may be retrieved from
/// the login flow of Facebook.
#[derive(Default, Debug)]
pub struct Token {
    ///access_token is used for API calls and it contains response data such as scopes
    pub access_token: String,

    ///Expires in 90 days based when the user was last active
    /// When this 90-day period expires, the user can still access your app — that is,
    /// they are still authenticated — but your app can't access their data.
    /// To regain data access, your app must ask the user to re-authorize your app's permissions.
    pub data_access_expiration_time: String,

    ///The expiration time of the Access Token
    pub expires_in: String,

    ///A long-lived token generally lasts about 60 days
    ///These tokens are refreshed once per day, when the person using your app makes a request to Facebook's servers.
    /// If no requests are made, the token will expire after about 60 days and
    /// the person will have to go through the login flow again to get a new token.
    pub long_lived_token: String,

    ///A string value created by your app to maintain state between the request and callback.
    pub state: String,
}

impl Token {
    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
}

///Gets gets a Token from the current URL by extracting the query query of the URL
pub fn get_token() -> Token {
    let query = extract_query_fragments(Url::current());

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
            _ => panic!("unknown field: {}"),
        }
    }
    response
}