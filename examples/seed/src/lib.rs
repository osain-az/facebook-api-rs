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

    let token = match url.hash() {
        Some(hash) => Token::get_token(hash.to_string()),
        None => Token::default(),
    };

    Model {
        redirect_url: RedirectURL::default(),
        error: None,
        response: token,
        image: None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    redirect_url: RedirectURL,
    error: Option<String>,
    response: Token,
    image: Option<Data<Image>>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// `Msg` describes the different events you can modify state with.
enum Msg {
    SignedFailed(String),
    ConfigFetched(fetch::Result<Config>),
    GetFriends,
    GetFriendsSuccess(Data<Image>),
    GetFriendsFailed(FetchError),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {


            Msg::ConfigFetched(Ok(config)) => model.redirect_url = RedirectURL::new(config).add_response_type("token").add_scope(&["email".to_string()]).add_full_url(),
            Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client:id and api_key", fetch_error),
            Msg::SignedFailed(err) => {model.error = Some(err)},
            Msg::GetFriends => {
                let url = "https://graph.facebook.com/v11.0/me/picture?access_token=".to_string() + &*model.response.access_token + "&format=json"+ "&redirect=false";
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
    log!(model.response.access_token);
    div![
        a![
            attrs! {
                At::Href => model.redirect_url.get_full_url()
            },
            button![img![
                attrs! {
                    At::Src => "src/fb.jpeg",
                },
                style! {
                    St::Height => "750px",
                    St::Width => "750px",
                },
                // Button style
                style! [
                St::Border => "none",
                St::BackgroundColor => "white"
                ],
            ],]
        ],
        button!["Get my friends!", ev(Ev::Click, |_| { Msg::GetFriends })],
        add_image(model.image.as_ref())
    ]
}

fn add_image(image: Option<&Data<Image>>) -> Node<Msg> {
    if let Some(img) = image {
        img![attrs! {
            At::Src=> img.data.url
        }]
    } else {
        div![" no image for now"]
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
