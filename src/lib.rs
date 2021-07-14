use seed::fetch::StatusCategory::Redirection;

struct RedirectURL{

    // The Facebook url preamble for the oath dialog.
    pub facebook_oath_url: String,

    // The ID of your app, found in your app's dashboard.
    pub client_id: String,

    // The URL that you want to redirect the person logging in back to.
    pub redirect_uri: String,

    // A string value created by your app to maintain state between the request and callback.
    //todo randomly generate this
    pub state: String,

    //  Determines whether the response data included when the redirect back to the app occurs is in URL parameters or fragments.
    pub response_type: ResponseType,

    // A comma or space separated list of Permissions to request from the person.
    pub scope: vec![]
}

//todo refactor!
struct  ResponseType {
    // Response data is included as URL parameters and contains code parameter (an encrypted string unique to each login request). This is the default behavior.
    ,
    // Response data is included as a URL fragment and contains an access token. Desktop apps must use this setting for response_type. This is most useful when the client will be handling the token.
    Token,
    // Response data is included as a URL fragment and contains both an access token and the code parameter.
    Code20Token,
    GrantedScopes,
}

fn build_redirect_url(facebook_oath_url: String, client_id: String, redirect_uri: String, state: String, scope: vec![]) -> RedirectURL {
    RedirectURL {
        facebook_oath_url,
        client_id,
        redirect_uri,
        state,
        kind: ResponseType,
        scope
    }
}



impl RedirectURL {
    fn facebook_oath_url(&self) -> &String {
        &self.facebook_oath_url
}
    fn client_id(&self) -> &String {
        &self.client_id
}
    fn redirect_uri(&mut self) -> &mut String {
        &mut self.redirect_uri
    }
    fn state(&self) -> &String {
        &self.state
    }
    fn response_type(&self) -> &String {
        &self.response_type
    }
    fn scope(&self) -> &String {
        &self.scope
    }

}


fn main() {

    let url = RedirectURL {
        facebook_oath_url: String::from("test"),
        client_id: String::from("09vudsfv987uh2398rehyfewb"),
        redirect_uri: String::from("43289wg234gty34509yj453+9yh"),
        state: String::from("OK"),
        kind: ResponseType::Token,
        scope: (123,1,321)
    };

}