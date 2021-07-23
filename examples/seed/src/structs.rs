
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
#[derive(Default, Debug)]
pub struct Token {
    
    pub  access_token: String,
    
    pub  data_access_expiration_time: String,
    
    pub expires_in: String,
    
    pub long_lived_token: String,
    
    pub  state: String,
    
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Data<T>
{
    pub data : T
}


#[derive(Deserialize, Debug, Serialize)]
pub struct Image {
    height:u16,
    width:u16,
    is_silhouette:bool,
    pub url:String,
}


pub type Code = String;

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
