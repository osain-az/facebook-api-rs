use facebook_api_rs::prelude::*;
use seed::{prelude::*, *};
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    
    
    let query = extract_query_fragments(url);
    
    
    
    orders.perform_cmd(async {
        // Load config from some json.
        // You can have a specific Api key here for facebook.
        Msg::ConfigFetched(
            async { fetch("/config.json").await?.check_status()?.json().await }.await,
        )
        
    });
    
    let iterations = query.iter();
    
    
    let mut response = Token::default();
    
    for e in iterations { 
        
        log!("{}", e.clone());
        
        match e.0.as_str() {
            "access_token" => {response.access_token = e.1.to_string();
                log!(e.1.as_str());
            },
            "data_access_expiration_time" => {response.data_access_expiration_time = e.1.to_string();
                log!(e.1.as_str());
            },
            "expires_in" => {response.expires_in = e.1.to_string();
                log!(e.1.as_str());
            },
            "long_lived_token" => {response.long_lived_token = e.1.to_string();
                log!(e.1.as_str());
            },
            "state" => {response.state = e.1.to_string();
                log!(e.1.as_str());
            },
            _ => panic!("unknown field: {}"),
        }


        log!("respone: {:?}", response);
        
    }


    
    /*
    
    
    access_token: token.get_index(0).unwrap().to_string(), 
    data_access_expiration_time: token.get_index(1).unwrap(),
    expires_in: token.get_index(2).unwrap(), 
    long_lived_token: token.get_index(3).unwrap(), 
    state: token.get_index(4).unwrap()
    
    */
    /*
    orders.perform_cmd(async move {
        
        Msg::TokenReceived(token.clone())
        
    });
    */
    
    Model {
        config: None,
        error: None,
        response: response,
        image:None,
    }
    
    
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    config: Option<RedirectURL>,
    error: Option<String>,
    response: Token,
    image : Option<Data<Image>>
 
}



impl Model {
    fn get_config(&self) -> &Option<RedirectURL> {
        &self.config
    }
}

/// Extract the query parameters from the url string and return an `IndexMap`
/// for the Enum Variant.
/// # Panics
/// The function will panic a key that has no value.
/// # Warns
/// A warning will be emitted if the query parameter string only contains `?`
/// with no query. Theses choices are opinionated for now.
pub fn extract_query_fragments(url : Url) -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    if let Some(hash) = url.hash() {
        let key_value: Vec<&str> = hash.split('&').collect();
        
        for pair in key_value {
            let mut sub = pair.split('=');
            let key = sub.next().unwrap_or_else(|| {
                panic!(
                    "we should have a key for the parameter key but got {}",
                    hash
                )
            });
            let value = sub.next().unwrap_or_else(|| {
                panic!("we should have a value for the key but got {}", hash)
            });
            query.insert(key.to_string(), value.to_string());
            
            
        }
    }
    query
}


// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// `Msg` describes the different events you can modify state with.
enum Msg {

    SignedFailed(String),
    ConfigFetched(fetch::Result<RedirectURL>),
    GetFriends,
    GetFriendsSuccess(Data<Image>),
    GetFriendsFailed(FetchError),

}
    
    // `update` describes how to handle each `Msg`.
    fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::ConfigFetched(Ok(config)) => model.config = Some(config),
            Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client:id and api_key", fetch_error),
            Msg::SignedFailed(err) => {model.error = Some(err)},
            Msg::GetFriends => {

              //  https://graph.facebook.com/v11.0/me/picture?access_token=EAAHNQH0awn4BAHBrjQQZA76SJxoZCYCWqUZBtZBMYJHcxMU3y38nUh8pgcqKIae3SRQ7ctWRxSRsCFGcRbHpU0nd776LTu3FZAGNVtXtJmUdZCKEBhXiQacY4KT1b9Jh8p5oWmy5K9E9BrkKC3QJYi6ZCvbFDNkXOObjOez1IN7HwysARRntWLQsAzEp8286J6cZAOxNz67GMrrITGsX88n9W9m8myGA3rrICdLBHzn5Un9U7CZAGl1To&__activeScenarioIDs=%5B%5D&__activeScenarios=%5B%5D&debug=all&format=json&method=get&pretty=0&redirect=false&suppress_http_code=1&transport=cors
                let url = "https://graph.facebook.com/v11.0/me/picture?access_token=".to_string() + model.response.access_token.as_str() + "&format=json"+ "&redirect=false";
                let request = fetch::Request::new(url).method(Method::Get);

                orders.perform_cmd( async  {

                    fetch(request).await.unwrap().json::<Data<Image>>().await.map_or_else(Msg::GetFriendsFailed,Msg::GetFriendsSuccess)

                }
            );
            },
            Msg::GetFriendsSuccess(image) => {
                    model.image = Some(image)
            },
            Msg::GetFriendsFailed(_) => {

            }
        }
        
    }
    
    
    
    // ------ ------
    //     View
    // ------ ------
    
    // `view` describes what to display.
    fn view(model: &Model) -> Node<Msg> { 
        
        
        
        div![
        if let Some(loaded_config) = &model.config {
            let url = loaded_config.get_facebook_oath_url().to_string()+loaded_config.get_client_id()+loaded_config.get_redirect_uri()+loaded_config.get_state()+loaded_config.get_response_type();
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
        },



    button![
            "Get my friends!",
            ev(Ev::Click, |_| { Msg::GetFriends })
        ] ,
            add_image(model.image.as_ref())

        
    ]
  



    }
   
    

    fn add_image(image : Option<&Data<Image>>) -> Node<Msg> {

      if   let Some(img) = image {

            img![
                attrs!{
                    At::Src=> img.data.url
                }
            ]
        } else {
            div![" no image for now"]
        }
    }
    
    #[cfg(test)]
    mod test {
        use seed::Url;
        use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
        use crate::extract_query_fragments;
        
        wasm_bindgen_test_configure!(run_in_browser);
        
        
        
        
        
        
        
        
        
        
        
        #[wasm_bindgen_test]
        fn test_extract_query_fragments() {
            let url: Url = "https://localhost:8001/?#access_token=EAAHNQH0awn4BACL9uLJx6v7bS0vGIqCd41uiprqFsGf3kcR6ZCImycS3NJTIoOqlmH77gnLx9nvf8KpdvMaQqNfoH7BcBe4ZAQgfvhf3RJFPZCMMvwM2pntTImHNEGQ2yOtYIkuZCdEUzho4FqqRfY0JSn4UZA0CUmOIdxeZASsOVBOcF9hAP5xluZBQHQvbwu3juK7iYTkpQZDZD&data_access_expiration_time=1634728338&expires_in=6462&long_lived_token=EAAHNQH0awn4BAEUg0mr6aEXHMtgDCHMARJMAMbmabb6hg089Dn6ufspDZAieowZA1D1w9n87x6xmdIxOMZBIZBlMBlb1r9BNnhBwuHwj6AVbS7ik2svICi6BUSysAL2ZBkAGjLdy8bVF0Ucf25vOMxlQ5qiKrwSL8LpmjJITnSFZCvCNR7u0XL&state=43y345eyghtrshyetnu35eyub65twrvys".parse().unwrap();
            
            
            let url_fragment = extract_query_fragments(url);
            
            assert_eq!(url_fragment.get("access_token").unwrap(), "EAAHNQH0awn4BACL9uLJx6v7bS0vGIqCd41uiprqFsGf3kcR6ZCImycS3NJTIoOqlmH77gnLx9nvf8KpdvMaQqNfoH7BcBe4ZAQgfvhf3RJFPZCMMvwM2pntTImHNEGQ2yOtYIkuZCdEUzho4FqqRfY0JSn4UZA0CUmOIdxeZASsOVBOcF9hAP5xluZBQHQvbwu3juK7iYTkpQZDZD");
            
            assert_eq!(url_fragment.get("data_access_expiration_time").unwrap(), "1634728338");
            
            assert_eq!(url_fragment.get("expires_in").unwrap(), "6462");
            
            assert_eq!(url_fragment.get("long_lived_token").unwrap(), "EAAHNQH0awn4BAEUg0mr6aEXHMtgDCHMARJMAMbmabb6hg089Dn6ufspDZAieowZA1D1w9n87x6xmdIxOMZBIZBlMBlb1r9BNnhBwuHwj6AVbS7ik2svICi6BUSysAL2ZBkAGjLdy8bVF0Ucf25vOMxlQ5qiKrwSL8LpmjJITnSFZCvCNR7u0XL");
            
            assert_eq!(url_fragment.get("state").unwrap(), "43y345eyghtrshyetnu35eyub65twrvys");
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
    