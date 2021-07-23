use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct RedirectURL {
    
    // The Facebook url preamble for the oath dialog.
    facebook_oath_url: String,
    
    // The ID of your app, found in your app's dashboard.
    client_id: String,
    
    // The URL that you want to redirect the person logging in back to.
    redirect_uri: String,
    
    // A string value created by your app to maintain state between the request and callback.
    //todo randomly generate this
    state: String,
    
    // Determines whether the response data included when the redirect back to the app occurs is in URL parameters or fragments.
    response_type: String,
    
    // A comma or space separated list of Permissions to request from the person.
    scope: Vec<String>,
}

#[allow(dead_code)]
impl RedirectURL {
    pub fn new(facebook_oath_url: String, client_id: String, redirect_uri: String, state: String, response_type: String, scope: Vec<String>) -> RedirectURL {
        RedirectURL {
            facebook_oath_url,
            client_id,
            redirect_uri,
            state,
            response_type,
            scope
        }
    }
    pub fn get_facebook_oath_url(&self) -> &String {
        &self.facebook_oath_url
    }
    pub  fn get_client_id(&self) -> &String {
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


