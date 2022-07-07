# Facebook API for Rust and Wasm

- This crate is intended for front_end and supposed to be used with the custom `https client` of your choice as long as it uses [http-types](https://docs.rs/http-types/2.11.0/http_types/).

- The Facebook API requires an `access token`.

- The implementation of the Facebook API is done via the help of the following [Facebook for Developers documentation](https://developers.facebook.com/docs/facebook-login/manually-build-a-login-flow)


# ToDo

### - Milestone version 0.1.x  <br/>
- [x] Invoking the Login Dialog and Setting the Redirect URL <br/>
- [x] Handling Login Dialog Response <br/>
- [ ] Canceled Login <br/>
- [ ] Confirming Identity <br/>
- [x] Exchanging Code for an Access Token <br/>
- [x] Handling Response which is returned in JSON format <br/>
- [x] Inspecting Access Tokens <br/>
- [ ] Checking Permissions <br/>
- [ ] Re-asking for Declined Permissions <br/>
- [x] Store Access Tokens and Login Status <br/>
- [ ] Logging People Out <br/>
- [ ] Detecting When People Uninstall Apps <br/>
- [ ] Responding to Requests to Delete User Data<br/>
- [x] Storage and tracking and login status



### Notes

- Feel free to tag along on the project 🦊

# Installing
The crate can be use at the frontend or server.
It also has two features: 

- `reqwest`: this uses `Reqwest` for http connection. This is the default feature
- `web-sys`: this uses `web-sys` for http connection. 

Either of the feature can be use for frontend 

### Frontend Cargo.toml
```toml
      #For frontend using default feature (reqwest)
 facebook_api_rs = {git = "https://github.com/Ringrev/facebook-api-rs", rev = "de4256d86150397d9b686079f86dd5428846db37"}

#For frontend using web-sys feature.
facebook_api_rs = {git = "https://github.com/Ringrev/facebook-api-rs", default-features = false, features = ["web-sys"], rev = "de4256d86150397d9b686079f86dd5428846db37"}
```

### Server Cargo.toml
At the backend only the default feature(reqwest) should be used 
```toml
 facebook_api_rs = {git = "https://github.com/Ringrev/facebook-api-rs", rev = "de4256d86150397d9b686079f86dd5428846db37"}
```

# Usage

##Manual Login Flow
 The crate uses manual login flow. 
### Basic Steps 
* `Build the login url`: this can be done at the frontend or backend.
It is recommended to store credential at the server(backend);

```rust
    use facebook_api_rs::prelude::{Config, LoginResponseType, LoginUrlParameters};

 // server side or client
pub fn login_url_handler() -> String {
    let redirect_uri = "".to_owned(); // the uri you want to be redirected to after login
    let facebook_app_id = "your app_id"; // your facebook app id 

    let config = Config::new(facebook_app_id.to_owned(), redirect_uri);
    
    // returning access token in the login response url. This will be verified at the server.
    let response_type : LoginResponseType::TOKEN;
     
     // returning code as the login response. This will be exchanged for access token at the server
    // let response_type : LoginResponseType::CODE; // check LoginResponseTyp for doc
     
    let login_url = LoginUrlParameters::new(config)
        .add_response_type(response_type)
        .add_scope(vec![]) //  an array of permission you ant to request
        .full_login_url();
     
    // user login url
    login_url
}
```
* `After user login to facebook, capture the tokens or error`:  

```rust
use facebook_api_rs::prelude::{
    Account, Accounts, Client, Me, Config, TokenLiveType, UserToken,
};
 async fn handle_user_login_response( url: String) {
   // After login redirect url 
    let tokens = UserToken::extract_user_tokens(url);
    
    if let Some(error) = tokens.login_error { 
        // handle error 
        return;
    }
    
// If you requested for code (LoginResponseType::CODE ) while building the login url then send the code to the 
// server to exchange for an access_token.
    let code = tokens.code;

// If you requested for a token (LoginResponseType::TOKEN ) while building the login url then send the access_token and the user id 
// to the server to verify
    let access_token = tokens;
// To verify the access token, you will need the login user id.
    let user: Me = Client::new(tokens, "".to_owned())
        .accounts(TokenLiveType::LONGLIVE)
        .user().await?;

// send access_token and the user_id to the server
    let user_id = user.id;
}
```
* `exchange code for access_token or verify access_token at the server`:  

```rust
  use crate::facebook_api_rs::prelude::{UserToken, Config};

async fn handle_token_verification() {
// To verify access_token sent from client side
    let access_token = "access_token from client";
    
//A valid token, it could be app_token, user_token, client_token, admin_token, page_token.
    let valid_token = "a valid token";
    let access_token_information = UserToken::access_token_information(
        valid_token,
        access_token
    ).await?;

// verify the access token 
    let user_id = "userid";
    let app_id = "your_app_id";

    if !access_token_information.is_valid {
        // handle it as you want 
        return "token not valid"
    }
    if access_token_information.app_id != app_id {
        // handle it as you want
        return "token not valid"
    }
    if access_token_information.user_id != user_id {
        // handle it as you want
        return "token not valid"
    }
}

// To exchange code for access_token
async fn exchange_code_for_access_token() {
    let code = "The code sent from client".to_string();
// The redirect_uri, must be the same uri used when building the login url.
    let redirect_uri = "uri";
    let config = Config::new("your app_id".to_owned(), redirect_uri);

    let access_token = UserToken::default()
        .exchange_code_for_access_token_at_server(
            code,
            "your app_secret".to_string(), config).await?;
}
```

## Making request to Facebook graph api. 

All request/methods can be found through the Client binder while errors are handle by ClientErr.

```rust
  // Get different accounts/pages a user have access to.
async fn exchange_code_for_access_token() {
    // intend to get page long live token 
    let page_access_token_type = TokenLiveType::LONGLIVE;   
    // intend to get page short live token 
    let page_access_token_type = TokenLiveType::SHORTLIVE;
    // Token from previous steps
    let tokens = "user token";
    let pages: Result<Accounts, ClientErr> = Client::new(tokens, "".to_owned()).accounts(page_access_token_type).get().await;
}
```
