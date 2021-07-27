use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug, Serialize)]
pub struct ResponseType {
    // Response data is included as URL parameters and contains code parameter (an encrypted string unique to each login request). This is the default behavior.
    code: String,

    // Response data is included as a URL fragment and contains an access token. Desktop apps must use this setting for response_type. This is most useful when the client will be handling the token.
    token: String,

    // Response data is included as a URL fragment and contains both an access token and the code parameter.
    code20token: String,

    // A comma-separated list of all Permissions granted to the app by the user at the time of login.
    // Can be combined with other response_type values.
    // When combined with token, response data is included as a URL fragment, otherwise included as a URL parameter.
    granted_scopes: Vec<String>,
}

/*
#[allow(dead_code)]
impl ResponseType {
    pub fn new(code: String, token: String, code20token: String, granted_scopes: Vec<String>) -> ResponseType {
        ResponseType {
            code,
            token,
            code20token,
            granted_scopes,
        }
    }
    fn get_code(&self) -> &String {
        &self.code
    }
    fn get_token(&self) -> &String {
        &self.token
    }
    fn get_code20token(&self) -> &String {
        &self.code20token
    }
    fn get_granted_scopes(&self) -> &Vec<String> {
        &self.granted_scopes
    }

    fn set_code(&mut self, code: String) {
        self.code = code;
    }
    fn set_token(&mut self, token: String) {
        self.token = token;
    }
    fn set_code20token(&mut self, code20token: String) {
        self.code20token = code20token;
    }
    fn set_granted_scopes(&mut self, granted_scopes: Vec<String>) {
        self.granted_scopes = granted_scopes;
    }
}
*/
