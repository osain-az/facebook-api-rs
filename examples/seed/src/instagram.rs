use crate::instagram::Msg::GetInstaAccountSuccess;
use facebook_api_rs::prelude::account::InstaAccount;
use facebook_api_rs::prelude::publish::{InstaMediaConatiner, InstaPostParams};
use facebook_api_rs::prelude::search::{PageSearch, PagesAPI};
use facebook_api_rs::prelude::*;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(_: Url, _: &mut Model, _: &mut impl Orders<Msg>) -> Model {
    Model {
        user_tokens: None,
        accounts: None,
        pages_api: PagesAPI::default(),
        selected_account: None,
        insta_account: None,
        insta_post_param: None,
        insta_media_container_id: None,
        access_token_info: None,
        insta_posting_options: InstaPostingOption {
            caption: false,
            location_tag: false,
            is_post_video: true,
            tag_users: false,
        },
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
#[derive(Debug, Clone)]
pub struct SelectedAccount {
    access_token: String,
    name: String,
    id: String,
}

#[derive(Debug, Clone)]
struct InstaPostData {
    url: String,
    caption: String,
    location_id: String, // this should be coded
}

fn insta_post_params(url: String, location_id: String, caption: String) -> InstaPostParams {
    InstaPostParams {
        url,
        caption,
        location_id,
        tag_users: vec![],
    }
}

#[derive(Debug, Default, Clone)]
pub struct InstaPostingOption {
    caption: bool,
    location_tag: bool,
    is_post_video: bool,
    tag_users: bool,
}

#[derive(Default)]
pub struct Model {
    pub user_tokens: Option<Token>,
    pub accounts: Option<Data<Accounts>>,
    pub pages_api: PagesAPI,
    pub selected_account: Option<SelectedAccount>,
    pub insta_account: Option<InstaAccount>,
    pub insta_post_param: Option<InstaPostParams>,
    pub insta_media_container_id: Option<InstaMediaConatiner>,
    pub insta_posting_options: InstaPostingOption,
    pub access_token_info: Option<AccessTokenInformation>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UpdateSelectedccount(SelectedAccount),

    // Instagram operations
    GetInstaAcoount,
    InstaMediaConatinerInit,
    InstaContainerResponse(InstaMediaConatiner),
    GetInstaAccountSuccess(InstaAccount),
    UpdatInstaPostParams(InstaPostParams),
    InstagramVideoPost,
    InstaPostSucessful(InstaMediaConatiner),
    HandleInstaPostingOption(web_sys::Event),
    PagesSearch(String),
    PageSearchResponse(PageSearch),

    // every error should user this
    ResponseFailed(FetchError),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UpdateSelectedccount(account) => {
            model.selected_account = Some(account);
        }
        // This method let you get  Instagram account based on the selected facebook page
        Msg::GetInstaAcoount => {
            if model.selected_account.is_some() {
                if let Some(selected_page) = &model.selected_account {
                    let page_access_token = selected_page.access_token.to_owned();
                    let facebook_page_id = selected_page.id.clone();

                    orders.perform_cmd(async move {
                        Client::new(Token::default(), page_access_token.clone())
                            .instagram_account(facebook_page_id)
                            .insta_account()
                            .await
                            .map_or_else(Msg::ResponseFailed, GetInstaAccountSuccess)
                    });
                }
            }
        }

        Msg::GetInstaAccountSuccess(resp) => {
            model.insta_account = Some(resp);
        }

        Msg::UpdatInstaPostParams(post_input) => {
            model.insta_post_param = Some(post_input);
            log!(model.insta_post_param);
        }
        // this method let you post videos to instagram
        Msg::InstagramVideoPost => {
            if model.selected_account.is_some() {
                if let Some(selected_page) = &model.selected_account {
                    if let Some(instag_account) = &model.insta_account {
                        if let Some(insta_media_container_id) = &model.insta_media_container_id {
                            let page_access_token = selected_page.access_token.to_owned();
                            let insta_page_id =
                                instag_account.instagram_business_account.id.clone();
                            let insta_media_conatiner = insta_media_container_id.id.clone();
                            if model.insta_post_param.clone().is_some() {
                                orders.perform_cmd(async move {
                                    Client::new(Token::default(), page_access_token)
                                        .instagram(insta_page_id)
                                        .publish_video(insta_media_conatiner)
                                        .await
                                        .map_or_else(Msg::ResponseFailed, Msg::InstaPostSucessful)
                                });
                            }
                        }
                    }
                }
            }
        }

        Msg::InstaPostSucessful(resp) => {
            log!(resp)
        }

        // this method is use to initialize the media conatiner
        Msg::InstaMediaConatinerInit => {
            if let Some(selected_page) = &model.selected_account {
                if let Some(instag_account) = &model.insta_account {
                    log!(instag_account);
                    let page_access_token = selected_page.access_token.to_owned();
                    let insta_page_id = instag_account.instagram_business_account.id.clone();

                    if let Some(post_param) = model.insta_post_param.clone() {
                        orders.perform_cmd(async move {
                            Client::new(Token::default(), page_access_token)
                                .instagram(insta_page_id)
                                .ig_media_container(post_param, "video".to_string())
                                .await
                                .map_or_else(Msg::ResponseFailed, Msg::InstaContainerResponse)
                        });
                    }
                }
            }
        }

        Msg::InstaContainerResponse(resp) => {
            model.insta_media_container_id = Some(resp);
            log!(model.insta_media_container_id);
        }

        Msg::HandleInstaPostingOption(e) => {
            let checked = e
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|file_input| file_input.checked())
                .unwrap();
            let event_type = e
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|file_input| file_input.name())
                .unwrap();

            if event_type == "is_post_video" {
                if checked {
                    model.insta_posting_options.is_post_video = false;
                } else {
                    model.insta_posting_options.is_post_video = true;
                }
            } else if event_type == "location_tag" {
                if checked {
                    model.insta_posting_options.location_tag = true;
                } else {
                    model.insta_posting_options.location_tag = false;
                };
            } else if event_type == "tag_user" {
                if checked {
                    model.insta_posting_options.tag_users = true;
                } else {
                    model.insta_posting_options.tag_users = false;
                };
            };
            log!(checked)
        }

        Msg::PagesSearch(e) => {
            log!(e);

            if let Some(selected_page) = &model.selected_account {
                if let Some(instag_account) = &model.insta_account {
                    log!(instag_account);
                    let page_access_token = selected_page.access_token.to_owned();
                    log!(e);
                    orders.perform_cmd(async move {
                        Client::new(Token::default(), page_access_token)
                            .search_pages()
                            .init_search()
                            .await
                            .map_or_else(Msg::ResponseFailed, Msg::PageSearchResponse)
                    });
                }
            }
        }
        Msg::PageSearchResponse(resp) => {
            log!(resp)
        }

        // all errro should user this, except the eeror neededs to be analyzed and do something
        // about it
        Msg::ResponseFailed(resp) => {
            log!(resp)
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            h4!["To use instagram business account, select the Facbook page that is connected to your account \
             then click the 'Get instagram account button'  "],

            style! [
               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            ]],
        div![
            style! [
               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            ],
            div![
                style! [
                //St:: MarginTop =>  "20px"
                  St:: MarginLeft =>  px(20)
                ],
                h3![
                    "Avaliable facebook page accounts",
                    attrs! {
                       // At
                    }
                ],
                if let Some(accounts) = &model.accounts {
                    display_account(accounts, model)
                } else {
                    div![""]
                }
            ],
            div![
                style! [
                  St:: MarginLeft =>  px(20)
                ],
                h3![
                    "Selected  account ",
                ],
                if let Some(selected_account) = &model.selected_account {
                    span!["Businnes : ".to_owned() + &selected_account.name,]
                } else {
                    div![""]
                }
            ],
        ],
        div![
            div![
                style! [
                  St:: Display => "flex",
                  St:: JustifyContent => "center",
                  St:: MarginTop =>  "20px"
                ],
                h2!["Instagram section "]
            ],
            div![
                style! [

                   St:: Display => "flex",
                   St:: JustifyContent => "center",
                   St:: MarginTop =>  "20px"
                ],
                button![
                    "Get Instagram Account",
                    ev(Ev::Click, |_| { Msg::GetInstaAcoount }),
                    attrs! {
                       At:: Disabled => model.selected_account.is_none().as_at_value()
                    }
                ]
            ],
            insta_post_options(model),
            if model.insta_posting_options.location_tag {
                pages_search()
            } else {
                div![]
            },
            div![
                style! [
                    St::  MarginTop => px(20),
                   St:: Display => "flex",
                   St:: JustifyContent => "center",
                ],
                instagram_post_params(model)
            ],
        ]
    ]
}

fn instagram_post_params(_model: &Model) -> Node<Msg> {
    div![
        h5!["Insta feed inputs "],
        div![style! {
            St:: Display => "flex",
           St:: JustifyContent => "center",
           St:: MarginTop =>  "20px"
        },],
        div![textarea![
            attrs! {
                At:: Placeholder => "url to photo or video"
            },
            input_ev(Ev::Input, move |post_url| {
                Msg::UpdatInstaPostParams(insta_post_params(
                    post_url,
                    "".to_string(),
                    "#DJ, #live".to_string(),
                ))
                // initiate or update/ build  feed struct
            })
        ]],
        div![textarea![
            attrs! {
                At:: Placeholder => "Caption "
            },
            input_ev(Ev::Input, move |caption| {
                Msg::UpdatInstaPostParams(insta_post_params(
                    "".to_string(),
                    "".to_string(),
                    caption,
                ))
            })
        ]],
        div![textarea![
            attrs! {
                At:: Placeholder => "   Location tag id"
            },
            input_ev(Ev::Input, move |location_id| {
                Msg::UpdatInstaPostParams(insta_post_params(
                    "".to_string(),
                    location_id,
                    "".to_string(),
                ))
            })
        ]],
        div![
            button![" sumbit  container feed "],
            ev(Ev::Click, |_| { Msg::InstaMediaConatinerInit })
        ],
        div![
            button!["publish feed "],
            ev(Ev::Click, |_| { Msg::InstagramVideoPost })
        ]
    ]
}

fn insta_post_options(model: &Model) -> Node<Msg> {
    div![
        style! [
            St:: Display => "flex",
           St:: JustifyContent => "center",
           St:: MarginTop =>  "20px"
        ],
        div![
            input![
                attrs! {
                    At::Type => "CheckBox",
                    At:: Name  => "is_video_post",
                    At:: Checked => model.insta_posting_options.is_post_video.as_at_value(),
                },
                ev(Ev::Change, |e| { Msg::HandleInstaPostingOption(e) })
            ],
            span![" video feed ? ".to_owned()],
        ],
        div![
            input![
                attrs! {
                    At::Type => "CheckBox",
                    At:: Name  => "location_tag",
                    At:: Checked => model.insta_posting_options.location_tag.as_at_value(),
                },
                ev(Ev::Change, |e| { Msg::HandleInstaPostingOption(e) })
            ],
            span![" location tag included ?  ".to_owned()],
        ],
        div![
            input![
                attrs! {
                    At::Type => "CheckBox",
                    At:: Name  => "users_tag",
                    At:: Checked => model.insta_posting_options.tag_users.as_at_value(),
                },
                ev(Ev::Change, |e| { Msg::HandleInstaPostingOption(e) })
            ],
            span!["tag user ".to_owned()],
        ],
    ]
}

// TO tag  a location, the user need to make search of relevant pages with that
// location
fn pages_search() -> Node<Msg> {
    div![
        h4!["Enter location"],
        div![textarea![
            attrs! {
                At:: Placeholder => "   Location tag id"
            },
            input_ev(Ev::Input, Msg::PagesSearch)
        ]],
        div![p!["Possible locations"]]
    ]
}

pub fn display_account(accounts: &Data<Accounts>, model: &Model) -> Node<Msg> {
    div![accounts.data.iter().map(|account| {
        let selected_account = SelectedAccount {
            name: String::from(&account.name),
            id: String::from(&account.id),
            access_token: String::from(&account.access_token),
        };

        div![
            if model.selected_account.is_some() {
                input![
                    "choose",
                    attrs! {
                        At::Type =>"Radio",
                        At:: Name  => "account",
                    },
                    ev(Ev::Change, |_| {
                        Msg::UpdateSelectedccount(selected_account)
                    })
                ]
            } else {
                input![
                    "choose",
                    attrs! {
                        At::Type =>"Radio",
                      //  At:: Checked => IF!()
                    },
                    ev(Ev::Click, |_| {
                        Msg::UpdateSelectedccount(selected_account)
                    })
                ]
            },
            span!["Businnes : ".to_owned() + &account.name,],
        ]
    })]
}
