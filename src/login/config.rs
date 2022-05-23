use serde::{Deserialize, Serialize};
/// A struct which describes the config.json file structure.
/// the json file fields are stored in this struct, and are then
/// added to the RedirectURL struct.
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// The Facebook url preamble for the oath dialog.
    pub facebook_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    pub client_id: String,

    /// The URL that you want to redirect the person logging in back to.
    pub redirect_uri: String,

    pub scope: Vec<String>,
}

impl Config {
    pub fn new(
        facebook_oath_url: String,
        client_id: String,
        redirect_uri: String,
        scope: Vec<String>,
    ) -> Self {
        Config {
            facebook_oath_url,
            client_id,
            redirect_uri,
            scope,
        }
    }

    pub fn facebook_oath_url(&self) -> &str {
        &self.facebook_oath_url
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }
}
