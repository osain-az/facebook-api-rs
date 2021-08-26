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

impl Config {
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
