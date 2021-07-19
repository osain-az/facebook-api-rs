use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use seed::prelude::UnwrapThrowExt;

#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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

pub fn main() {




    #[derive(Serialize, Deserialize, Debug)]
    struct AccessTokenData{
        access_token: String,
        token_type: String,
        expires_in: u32
    }

    impl AccessTokenData {

    }

    struct CanceledLogin{
        error_redirect_uri: String,
        error_reason: String,
        error: String,
        error_description: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Code {
        code: String
    }

    impl Code {

        fn get_code(&self) -> &String {
            &self.code
        }

    }
    #[tokio::main]
    async fn main2() -> Result<(), Box<dyn std::error::Error>> {

        let response_type = ResponseType::new(
            "some code".parse().unwrap(),
            "the access token".parse().unwrap(),
            "I don't remember".parse().unwrap(),
            vec!["username".to_string(), "likes".to_string()]);


        let mut redirect_url = RedirectURL::new("https://www.facebook.com/v11.0/dialog/oauth?".parse().unwrap(),
                                                "client_id=507151837217406".parse().unwrap(),
                                                r#"&redirect_uri=http://localhost:8000"#.parse().unwrap(),
                                                r#"&state="{st=state123abc,ds=123456789}""#.parse().unwrap(),
                                                response_type,
                                                vec!["test".to_string()],
        );

       // println!("{:?}", redirect_url);
      //  println!("{}{}{}{}", redirect_url.get_facebook_oath_url(), redirect_url.get_client_id(), redirect_url.get_redirect_uri(), redirect_url.get_state());


        // Build the client using the builder pattern
        let client = reqwest::Client::builder()
            .build()?;

        let graph = r#"https://graph.facebook.com/v11.0/oauth/access_token?"#;

        let secret = r#"&client_secret=cb112e5dd460eccf5d539a694affbe6d"#;
        let a_string = redirect_url.get_client_id().to_string()+ &*redirect_url.get_redirect_uri().to_string() + &*redirect_url.get_state().to_string();

        let res = client
            .get(redirect_url.get_facebook_oath_url().to_string())
            .query(&a_string)
            .send()
            .await?;

        let code = res
            .json::<Code>()
            .await?;










                // This will POST a body of `{"lang":"rust","body":"json"}`
                let mut map = HashMap::new();
                map.insert("facebook smth", "facebook details");
                map.insert("body", "json");

                // Perform the actual execution of the network request
                let res = client
                    .get(redirect_url.get_facebook_oath_url().to_string()+redirect_url.get_client_id()+redirect_url.get_redirect_uri()+redirect_url.get_state())
                    .json(&map);

                println!("body = {:?} ------------------------------------------------", res);
        /*

                let a_form = {redirect_url.get_client_id().to_string()+ &*redirect_url.get_redirect_uri().to_string() + &*secret.to_string()};
                let res_2 = client
                    .post(graph.to_string())
                    .query(a_form.as_str());

                println!("res_2body: {:?}", res_2);

                let res_3 = client
                    .get(graph.to_string()+redirect_url.get_client_id()+redirect_url.get_redirect_uri()+secret+r#"&code=testing/"#)
                    .send()
                    .await?;
            //    println!("body = {:?}", res_2);


                // Parse the response body as Json in this case
                let data = client.post(graph.to_string()+redirect_url.get_client_id()+redirect_url.get_redirect_uri()+secret+r#"&code=testing"#)
                    .json(&map)
                    .send()
                    .await?;

        */
        println!("data {:?}", code);
        Ok(())
    }

}

