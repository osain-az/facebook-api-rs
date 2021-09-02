use facebook_api_rs::prelude::*;
use seed::{prelude::*, *};
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        // Load config from some json.
        // You can have a specific Api key here for facebook.
        Msg::ConfigFetched(

            async { fetch("/config.json").await?.check_status()?.json().await }.await,
        )
    });

    let response = url.hash().map(|hash|Token::get_token(hash.to_string()));

    Model {
        redirect_url: RedirectURL::default(),
        error: None,
        response: response,
        image: None,
        account: None,
        pages_api: PagesAPI::default(),
        me: None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    redirect_url: RedirectURL,
    error: Option<String>,
    response: Option<Token>,
    image: Option<Data<Image>>,
    account: Option<Data<Accounts>>,
    pages_api: PagesAPI,
    me: Option<Data<Me>>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// `Msg` describes the different events you can modify state with.
enum Msg {
    ConfigFetched(fetch::Result<Config>),
    GetProfilePicture,
    GetProfilePictureSuccess(Data<Image>),
    GetProfilePictureFailed(FetchError),
    GetMe,
    GetMeSuccess(Data<Me>),
    GetMeFailed(FetchError),
    GetAccount,
    GetAccountSuccess(Data<Accounts>),
    GetAccountFailed(FetchError),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
            Msg::ConfigFetched(Ok(config)) => model.redirect_url = RedirectURL::new(config).add_response_type("token").add_scope(&["email".to_string()]).add_full_url(),
            Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client_id and redirect_uri", fetch_error),
            Msg::GetProfilePicture => {
           if let Some(response) = &model.response {
               let url = "https://graph.facebook.com/v11.0/me/picture?access_token=".to_string() + &response.access_token + "&format=json"+ "&redirect=false";
               let request = fetch::Request::new(url).method(Method::Get);
               orders.perform_cmd( async  {

                   fetch(request).await.unwrap().json::<Data<Image>>().await.map_or_else(Msg::GetProfilePictureFailed,Msg::GetProfilePictureSuccess)

               }
               );
           }
            },
            Msg::GetProfilePictureSuccess(image) => {
                    model.image = Some(image)
            },
            Msg::GetProfilePictureFailed(_) => {}

            Msg::GetMe => {
                if let Some(response) = &model.response {


                    let url = "https://graph.facebook.com/v11.0/me?access_token=".to_string() + &response.access_token ;
                    let request = fetch::Request::new(url).method(Method::Get);
                    orders.perform_cmd( async  {
                        fetch(request).await
                            .unwrap()
                            .json::<Data<Me>>()
                            .await
                            .map_or_else( Msg::GetMeFailed, Msg::GetMeSuccess)

                    }
                    );
                }


              /*  let request = fetch::Request::new(url).method(Method::Get);*/

              /*  orders.perform_cmd( async  {
                    fetch(request).await
                        .unwrap()
                        .json::<Data<Me>>()
                        .await
                        .map_or_else( Msg::GetMeFailed, Msg::GetMeSuccess)

                }
                );*/
            },

            Msg::GetAccount => {

                if let Some(response) = &model.response {
                    log!("click to get accounts");
                    let client = Client::new(response.clone());
                    orders.perform_cmd(async {
                        client.me().accounts().get()
                            .await
                            .map_or_else(Msg::GetAccountFailed, Msg::GetAccountSuccess)
                    });
                }
    },


        Msg::GetMeSuccess(me) => {
            model.me = Some(me);
        }

        Msg::GetMeFailed(_) =>{
            log!(" get me failed")
        }

        Msg::GetAccountSuccess(account) => {
            log!(account);
        model.account = Some(account);

        },

        Msg::GetAccountFailed(err) =>{
            log!("account failed");
            log!(err);
        }
        
}
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        style! [
            St::Width => "100%",

         ],
        h1![
            "facebook Api example",
            style! [
            St:: Display => "center",

         ],
        ],
        div![
         style! [
            St:: Display => "flex",
            St:: JustifyContent => "center",
            St:: MarginTop =>  "20px"
         ],
        a![
            attrs! {
                At::Href => model.redirect_url.get_full_url()
            },
            button![img![
                attrs! {
                    At::Src => "src/login_button.png", // attribute <a href="https://www.freeiconspng.com/img/18026">Facebook login button png</a>
                },
                style! {
                    St::Height => "100px",
                    St::Width => "300px",
                },
                // Button style
                style! [
                St::Border => "none",
                St::BackgroundColor => "white",
                St::MarginLeft => "auto",
                St::MarginRight => "auto",

                ],
            ],]
        ],
        button![
            "Get my Profile Picture!",
            ev(Ev::Click, |_| { Msg::GetProfilePicture }),
             attrs! {
                        At:: Disabled => model.response.is_none().as_at_value()
                    },
           style! {
                St::Height => "50px"

            },
        ],
            attrs! {
                At::Src => format!("{:?}",model.account),
            },
            style! {
                St::Height => "50px"
            },
            button!["Get my Account!", ev(Ev::Click, |_| { Msg::GetAccount }),
                attrs! {
                        At:: Disabled => model.response.is_none().as_at_value()
                    }],
        ],
        div![
            h3!( "Avaliable accounts ", attrs!{
            }),
        ]

    ]
}

fn add_image(image: Option<&Data<Image>>) -> Node<Msg> {
    div![""]
}

fn add_account(account: Option<&Data<Accounts>>) -> Node<Msg> {
      log!(account);
  //  if let Some(account) = account {
    //    div![attrs!{At::Src => account.data.get_access_token()}

   // } else {
        div![""]
   // }
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
