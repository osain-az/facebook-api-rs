use serde::{Deserialize};
#[derive(Deserialize, Debug)]
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
