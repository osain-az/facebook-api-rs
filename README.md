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

- `reqwest`: this uses `Reqwest` for http connection(default feature)
- `web-sys`: this uses `web-sys`  for http connection. 

Either of the feature can be use for frontend 

### Frontend 
```toml
      #For frontend using default feature (reqwest)
 facebook_api_rs = {git = "https://github.com/Ringrev/facebook-api-rs", rev = "de4256d86150397d9b686079f86dd5428846db37"}

#For frontend using web-sys feature.
facebook_api_rs = {git = "https://github.com/Ringrev/facebook-api-rs", default-features = false, features = ["web-sys"], rev = "de4256d86150397d9b686079f86dd5428846db37"}
```

### Server
At the backend only the default feature(reqwest) should be used 
```toml
 facebook_api_rs = {git = "https://github.com/Ringrev/facebook-api-rs", rev = "de4256d86150397d9b686079f86dd5428846db37"}
```

# Usage

##Manual Login Flow
 The crate uses manual login flow. 
### Steps 
* `Build the login url`: this can be done at the frontend or backend.
It is recommended to store credential at the server(backend). 

```rust
 // server side 
pub fn login_url_handler() -> String {
    use facebook_api_rs::prelude::{Config as FB_config, LoginResponseType, LoginUrlParameters};
    let redirect_uri = "".to_owned();
    let facebook_app_id = &CUSTOM_CONFIG.facebook.app_id; // your facebook app id 

    let config = FB_config::new(facebook_app_id.to_owned(), redirect_uri);
    
    let response_type : LoginResponseType::TOKEN; // check LoginResponseTyp for doc
    let login_url = LoginUrlParameters::new(config)
        .add_response_type(response_type)
        .add_scope(vec!["".to_string()]) //  an array of permission you ant to request
        .full_login_url();
    // send the url to the frontend to login user 
    login_url
}
```
* `login user to facebook at the frontend`:  

```rust
use facebook_api_rs::prelude::{
    Account, Accounts, Client, Config, TokenLiveType, UserToken,
};
// After login redirect url 
let tokens = UserToken::extract_user_tokens(url);

// If you request for code (LoginResponseType::CODE ) while building the url then send the code to the 
// server to exchange for an access_token.

 let code = tokens.code;

// If you request for a token (LoginResponseType::TOKEN ) while building the url then send the access_token to the 
// server to verify.
let access_token = tokens.acces_token; 
```
* `exchange code for access_token or verify access_token at the server`:  

```rust
  use crate::facebook_api_rs::prelude::{UserToken, Config};

// To verify access_token sent from client side 

let verifying_token = "access_token from client"; 
//A valid token, it could be app_token, user_token, client_token, admin_token, page_token.
let valid_token  = "a valid token"
let access_token_information = UserToken::access_token_information(
      valid_token,
     verifying_token
        ).await?;

// To exchange code for access_token

 let code = "The code sent from client".to_string();
// The redirect_uri must be the uri used when building the login url.
  let redirect_uri = "uri";
 let config = Config::new("your app_id".to_owned(), redirect_uri);

 let access_token  = UserToken::default()
        .exchange_code_for_access_token_at_server(
         code,
         "your app_secret".to_string(), config).await;
```

## Making request to Facebook graph api. 

All request/methods can be found through the Client binder while errors are handle by ClientErr

```rust
  // Get different accounts/pages a user have access to.

 let page_access_token_type = TokenLiveType::LONGLIVE;
 // Token from previous steps
 let acess_token = "user token" 
 let pages :Result<Accounts, ClientErr>  = Client::new(acess_token, "".to_owned()).accounts(page_access_token_type).get().await;


```
