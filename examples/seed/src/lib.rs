use seed::{prelude::*, *};
use wasm_bindgen::prelude::*;
use web_sys::{File, HtmlInputElement};

use facebook_api_rs::prelude::errors::ClientErr;
use facebook_api_rs::prelude::*;
use seed_routing::{ParsePath, View, *};

mod facebook;
mod instagram;
add_router!();

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    let token_response = url
        .clone()
        .hash()
        .map(|hash| Token::extract_user_tokens(hash.to_string()));

    sync_router!();
    orders.perform_cmd(async {
        // Load config from some json.
        // You can have a specific Api key here for facebook.
        Msg::ConfigFetched(
            async { fetch("/config.json").await?.check_status()?.json().await }.await,
        )
    });

    Model {
        redirect_url: RedirectURL::default(),
        user_tokens: token_response,
        accounts: None,
        switch_account_to: "".to_string(),
        facebook: facebook::Model::default(),
        instagram: instagram::Model::default(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.

#[derive(Default)]
pub struct Model {
    redirect_url: RedirectURL,
    user_tokens: Option<Token>,
    accounts: Option<Data<Accounts>>,
    switch_account_to: String,
    facebook: facebook::Model,
    instagram: instagram::Model,
}

#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Facebook,
    Instagram,
    #[default_route]
    #[as_path = ""]
    #[view = " => home"] // -> http://localhost:8000/
    Home,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ConfigFetched(fetch::Result<Config>),
    GetAccount,
    GetAccountSuccess(Data<Accounts>),
    GetMeDetails,
    GetMeDetailsSuccess(Me),
    AccessTokenInformation,
    AccessTokenInfoData(AccessTokenInformation),

    Facebook(facebook::Msg),
    Instagram(instagram::Msg),
    SwitchTo(String),
    UrlChanged(subs::UrlChanged),

    // every error should user this
    ResponseFailed(ClientErr),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ConfigFetched(Ok(config)) => model.redirect_url = RedirectURL::new(config).add_response_type("token").add_scope(&["email".to_string()]).add_full_url(),
        Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client_id and redirect_uri", fetch_error),

        Msg::GetAccount => {
            orders.send_msg(Msg::GetMeDetails);
          if let Some(user_access_tokens) = model.user_tokens.clone() {
                let user_tokens = user_access_tokens;
                let client = Client::new(user_tokens, "".to_string());
                orders.perform_cmd(async {
                    // we are interested in the page long live token, therefore we called the long
                    // live methed by passing "long_live_token" to the method
                    client
                        .me_by_short_or_long_live_token("short_live".to_string())
                        .accounts()
                        .get()
                        .await
                        .map_or_else(Msg::ResponseFailed, Msg::GetAccountSuccess)
                });
            }
        }

        Msg::GetMeDetails => {
            if let Some(user_access_tokens) = model.user_tokens.clone() {
                let user_tokens = user_access_tokens;
                let user_token = user_tokens.long_lived_token.clone();
                let client = Client::new(user_tokens, "".to_string());
                orders.perform_cmd(async {
                    // we are interested in the page long live token, therefore we called the long
                    // live methed by passing "long_live_token" to the method
                    client
                        .me_by_short_or_long_live_token("short_live".to_string())
                        .details()
                        .await
                        .map_or_else(Msg::ResponseFailed, Msg::GetMeDetailsSuccess)
                });
            }
        }
        Msg:: GetMeDetailsSuccess(resp) => {

        }

        Msg:: GetAccountSuccess(accounts) => {
            model.accounts = Some(accounts.clone());
            model.facebook.accounts= Some(accounts.clone());
            model.instagram.accounts = Some(accounts);
            log!(model.instagram.accounts)
        }

        Msg::Facebook(msg)=>{
            facebook::update(msg, &mut model.facebook, &mut orders.proxy(Msg::Facebook));

        }

        Msg::Instagram(msg)=>{
            instagram::update(msg, &mut model.instagram, &mut orders.proxy(Msg::Instagram));

        }

        Msg:: AccessTokenInformation =>{

        }

        Msg::AccessTokenInfoData(resp) => {

        }

        // handle switch betwen facebook and innstagram
        Msg:: SwitchTo(account_type) => {
            if account_type =="facebook" {
                let url = Routes::Facebook
                    .to_url();
                orders.request_url(url);
            }else if  account_type =="home" {
                let url = Routes::Home
                    .to_url();
                orders.request_url(url);
            }
            else if  account_type =="instagram" {
                let url = Routes::Instagram
                    .to_url();
                orders.request_url(url);
            }
            else {

            }

        }

        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("url changed");
            if let Ok(r) = Routes::from_url(url) {
                r.init(model, orders);
            }
        }

        // all errro should user this, except the eeror neededs to be analyzed and do something about it
        Msg::ResponseFailed(resp) => {
            log!(resp)
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        h1![
            " Welcome to facebook-Api.rs example",
            style! {
               St:: TextAlign => "center",
            },
        ],
        h3![
            " Connecting you to your facebook, instagram and ..... ",
            style! {
               St:: TextAlign => "center",
            },
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
                button![
                    img![
                        attrs! {
                            At::Src => "images/blue_58.png",
                           At:: Width => 40,
                            At:: Height => 40,

                        //   At::Src => "src/login_button.png", // attribute <a href="https://www.freeiconspng.com/img/18026">Facebook login button png</a>
                        },
                        style! {
                         St::PaddingTop => px(3),
                         St:: MarginLeft => px(7)
                        },
                    ],
                    span![
                        "Login with Facebook",
                        style! {
                           St:: MarginTop => px(11),
                           St:: MarginLeft => px(8)
                        },
                    ],
                    style! [
                        St:: Display => "flex" ,
                     St:: Color => "#1877F2" ,
                     St:: BorderRadius => px(10),
                     St:: BorderColor => " #1877F2",
                    St::MarginRight => px(10),
                    St:: BackgroundColor => "white",
                     St::Width => "270px", // 240 - 400 px
                     St::Height => "50px",
                      St:: FontSize => "1.2em"

                    ],
                ]
            ],
            attrs! {
                At::Src => format!("{:?}",model.accounts),
            },
            style! {
                St::Height => "50px"
            },
        ],
        div![
            style! [
               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            ],
            attrs! {
                At::Src => format!("{:?}",model.accounts),
            },
            style! {
                St::Height => "50px"
            },
            button![
                "Get my Account!",
                ev(Ev::Click, |_| { Msg::GetAccount }),
                attrs! {
                    At:: Disabled => model.user_tokens.is_none().as_at_value()
                }
            ],
            button![
                "Test long live token !",
                ev(Ev::Click, |_| { Msg::AccessTokenInformation }),
                attrs! {
                    At:: Disabled => model.user_tokens.is_none().as_at_value()
                },
                style! {
                    St::Height => "50px",
                    St::MarginRight => px(10),
                },
            ],
        ],
        div![
            style! [
               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            ],
            button![
                "Got to Instagram ",
                ev(Ev::Click, |_| { Msg::SwitchTo("instagram".to_string()) }),
                style! [
                 St:: Color =>  if model.user_tokens.is_none() { "grey" } else{  "#1877F2"},
                 St:: BorderRadius => px(10),
                 St:: BorderColor => " #1877F2",
                St:: MarginRight => px(10),
                St:: BackgroundColor => "white",
                 St:: Height => "36px",
                  St:: FontSize => "1.1em"
                ],
            ],
            button![
                " Go to Facebook",
                ev(Ev::Click, |_| { Msg::SwitchTo("facebook".to_string()) }),
                attrs! {
                    At:: Disabled => model.user_tokens.is_none().as_at_value()
                },
                style! [
                 St:: Color =>  if model.user_tokens.is_none() { "grey" } else{  "#1877F2"},
                 St:: BorderRadius => px(10),
                 St:: BorderColor => " #1877F2",
                St:: MarginRight => px(10),
                St:: BackgroundColor => "white",
                 St:: Height => "36px",
                  St:: FontSize => "1.1em"
                ],
            ],
        ],
        router().current_route().view(model),
    ]
}

fn home(model: &Model) -> Node<Msg> {
    div![h4![
        "Login to facebook  to test the  apis ",
        style! {
           St:: TextAlign => "center",
        },
    ]]
}

// ed::browser::dom::event_handler
// pub fn ev<Ms: 'static, MsU: 'static>(trigger: impl Into<Ev>, handler: impl
// FnOnce(web_sys::Event) -> MsU + 'static + Clone) -> EventHandler<Ms>

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
