mod sign_in;

use seed::{prelude::*, *};
use serde::{Serialize, Deserialize};
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
   let code_param= url.search().get("code");
    log!(code_param);
        orders.perform_cmd(async {
        // Load config from some json.
        // You can have a specific Api key here for facebook.
        Msg::ConfigFetched(
            async { fetch("/config.json").await?.check_status()?.json().await }.await,
        )
    });
    if let Some(code) = code_param {
        let code_param = Token {
            access_token: code.get(0).unwrap().to_string()
        };
        Model {
            config: None,
            error: None,
            token: Some(code_param),
        }

    } else {     Model {
        config: None,
        error: None,
        token: None,
    } }


}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    config: Option<RedirectURL>,
    error: Option<String>,
    token: Option<Token>,
}

impl Model {
    fn get_config(&self) -> &Option<RedirectURL> {
        &self.config
    }
}


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
    response_type: ResponseType,

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

pub struct Token {
    access_token: String,
}

#[allow(dead_code)]
impl RedirectURL {
    pub fn new(facebook_oath_url: String, client_id: String, redirect_uri: String, state: String, response_type: ResponseType, scope: Vec<String>) -> RedirectURL {
        RedirectURL {
            facebook_oath_url,
            client_id,
            redirect_uri,
            state,
            response_type,
            scope
        }
    }
    fn get_facebook_oath_url(&self) -> &String {
        &self.facebook_oath_url
    }
    fn get_client_id(&self) -> &String {
        &self.client_id
    }
    fn get_redirect_uri(&self) -> &String {
        &self.redirect_uri
    }
    fn get_state(&self) -> &String {
        &self.state
    }
    fn get_response_type(&self) -> &ResponseType {
        &self.response_type
    }
    fn get_scope(&self) -> &Vec<String> {
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
    fn set_response_type(&mut self, response_type: ResponseType) {
        self.response_type = response_type;
    }
    fn set_scope(&mut self, scope: Vec<String>) {
        self.scope = scope;
    }
}

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

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// `Msg` describes the different events you can modify state with.
enum Msg {
    SignedFailed(String),
    ConfigFetched(fetch::Result<RedirectURL>),}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
    Msg::ConfigFetched(Ok(config)) => model.config = Some(config),
    Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client:id and api_key", fetch_error),
    Msg::SignedFailed(err) => {model.error = Some(err)}
    }

}



// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    if let Some(loaded_config) = &model.config {
        let url = loaded_config.get_facebook_oath_url().to_string()+loaded_config.get_client_id()+loaded_config.get_redirect_uri()+loaded_config.get_state();
        a![
            attrs! {
                At::Href => url
            },
            button![ img![
                    attrs! {
                    At::Src => "src/fb.jpeg",
                    },
                    style! {
                            St::Height => "750px",
                            St::Width => "750px",
                    }
                ,
                // Button style
                style! [
                    St::Border => "none",
                    St::BackgroundColor => "white"
                ],
            ],
            ]
        ]
    } else {
        div![]
    }
}
// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
