use crate::instagram::Msg::{InstagramAccountDetailsSuccess, InstagramAccountIdSuccess};
use facebook_api_rs::prelude::account::{InstaAccountIds, InstagramAccount};
use facebook_api_rs::prelude::errors::ClientErr;
use facebook_api_rs::prelude::media::MediaContainerStatus;
use facebook_api_rs::prelude::publish::{InstaMediaContainerId, InstaPostParams};
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
        instagram_account: None,
        media_container_status: None,
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
        tag_users: None,
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
    pub insta_account: Option<InstaAccountIds>, // this is just the instagram id
    pub insta_post_param: Option<InstaPostParams>,
    pub insta_media_container_id: Option<InstaMediaContainerId>,
    pub insta_posting_options: InstaPostingOption,
    pub access_token_info: Option<AccessTokenInformation>,
    instagram_account: Option<InstagramAccount>,
    media_container_status: Option<MediaContainerStatus>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UpdateSelectedAccount(SelectedAccount),

    // Instagram operations
    GetInstagramAccountId,
    InstaMediaContainerInit,
    InstaContainerResponse(InstaMediaContainerId),
    InstagramAccountIdSuccess(InstaAccountIds),
    UpdateInstaPostParams(InstaPostParams, String),
    InstagramVideoPost,
    InstaPostSuccessful(InstaMediaContainerId),
    HandleInstaPostingOption(web_sys::Event),
    PagesSearch(String),
    GetInstagramAccountDetails(InstaAccountIds),
    InstagramAccountDetailsSuccess(InstagramAccount),
    PageSearchResponse(PageSearch),

    MediaContainerStatus,
    MediaContainerStatusResponse(MediaContainerStatus),

    FetchInstagramAccountId,
    FetchInstagramAccountIdSuccess(InstaAccountIds),

    // every error should user this
    TestFailed(ClientErr),
    ResponseFailed(ClientErr),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UpdateSelectedAccount(account) => {
            model.selected_account = Some(account);
        }
        // This method let you get  Instagram account based on the selected facebook page
        Msg::GetInstagramAccountId => {
            if model.selected_account.is_some() {
                if let Some(selected_page) = &model.selected_account {
                    let page_access_token = selected_page.access_token.to_owned();
                    let facebook_page_id = selected_page.id.clone();

                    orders.perform_cmd(async move {
                        Client::new(Token::default(), page_access_token.clone())
                            .instagram_account(facebook_page_id)
                            .account_id()
                            .await
                            .map_or_else(Msg::TestFailed, InstagramAccountIdSuccess)
                    });
                }
            }
        }

        Msg::InstagramAccountIdSuccess(resp) => {
            model.insta_account = Some(resp);
        }
        Msg::FetchInstagramAccountId => {
            if model.selected_account.is_some() {
                if let Some(selected_page) = &model.selected_account {
                    let page_access_token = selected_page.access_token.to_owned();
                    let facebook_page_id = selected_page.id.clone();

                    orders.perform_cmd(async move {
                        Client::new(Token::default(), page_access_token.clone())
                            .instagram_account(facebook_page_id)
                            .account_id()
                            .await
                            .map_or_else(Msg::TestFailed, Msg::FetchInstagramAccountIdSuccess)
                    });
                }
            };
        }
        Msg::FetchInstagramAccountIdSuccess(insta_id) => {
            orders.send_msg(Msg::GetInstagramAccountDetails(insta_id));
        }
        Msg::GetInstagramAccountDetails(insta_id) => {
            let instagram_id = insta_id.instagram_business_account.id.clone();
            if model.selected_account.is_some() {
                if let Some(selected_page) = &model.selected_account {
                    let page_access_token = selected_page.access_token.to_owned();

                    orders.perform_cmd(async move {
                        Client::new(Token::default(), page_access_token.clone())
                            .instagram_account(instagram_id)
                            .account_details()
                            .await
                            .map_or_else(Msg::TestFailed, InstagramAccountDetailsSuccess)
                    });
                }
            }
        }

        Msg::InstagramAccountDetailsSuccess(insta_account_details) => {
            model.instagram_account = Some(insta_account_details)
        }

        Msg::UpdateInstaPostParams(post_input, input_type) => {
            if let Some(post_params) = model.insta_post_param.clone() {
                let mut post_data = post_params;
                if input_type == "url" {
                    post_data.url = post_input.url;
                } else if input_type == "caption" {
                    post_data.caption = post_input.caption;
                }
                model.insta_post_param = Some(post_data);
            } else {
                model.insta_post_param = Some(post_input);
            }

            log!(model.insta_post_param);
        }
        // this method let you post videos to instagram
        Msg::InstagramVideoPost => {
            if model.selected_account.is_some() {
                if let Some(selected_page) = &model.selected_account {
                    if let Some(insta_account) = &model.insta_account {
                        if let Some(insta_media_container_id) = &model.insta_media_container_id {
                            let page_access_token = selected_page.access_token.to_owned();
                            let insta_page_id = insta_account.instagram_business_account.id.clone();
                            let insta_media_container = insta_media_container_id.id.clone();
                            if model.insta_post_param.clone().is_some() {
                                orders.perform_cmd(async move {
                                    Client::new(Token::default(), page_access_token)
                                        .instagram_publish(insta_page_id)
                                        .publish_media(insta_media_container)
                                        .await
                                        .map_or_else(Msg::ResponseFailed, Msg::InstaPostSuccessful)
                                });
                            }
                        }
                    }
                }
            }
        }

        Msg::InstaPostSuccessful(resp) => {
            log!(resp)
        }

        // this method is use to initialize the media container
        Msg::InstaMediaContainerInit => {
            if let Some(selected_page) = &model.selected_account {
                if let Some(insta_account) = &model.insta_account {
                    log!("accountgggugugugu gugug", insta_account);
                    let page_access_token = selected_page.access_token.to_owned();
                    let insta_page_id = insta_account.instagram_business_account.id.clone();

                    if let Some(post_param) = model.insta_post_param.clone() {
                        orders.perform_cmd(async move {
                            Client::new(Token::default(), page_access_token)
                                .instagram_publish(insta_page_id)
                                .post_media(post_param, "video".to_string()) //note: for photo passing in "photo" instead of "video" that was passed in
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
            log!(event_type);
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

        Msg::MediaContainerStatus => {
            if model.selected_account.is_some() {
                if let Some(selected_page) = &model.selected_account {
                    if let Some(insta_media_container_id) = &model.insta_media_container_id {
                        let page_access_token = selected_page.access_token.to_owned();

                        let insta_media_container = insta_media_container_id.id.clone();
                        orders.perform_cmd(async move {
                            Client::new(Token::default(), page_access_token)
                                .instagram_media_container(insta_media_container)
                                .status()
                                .await
                                .map_or_else(Msg::ResponseFailed, Msg::MediaContainerStatusResponse)
                        });
                    }
                }
            }
        }

        Msg::MediaContainerStatusResponse(status) => model.media_container_status = Some(status),

        Msg::PagesSearch(e) => {
            log!(e);

            if let Some(selected_page) = &model.selected_account {
                if let Some(_insta_account) = &model.insta_account {
                    let page_access_token = selected_page.access_token.to_owned();
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

        // all error should user this, except the error needed to be analyzed and do something
        // about it
        Msg::ResponseFailed(resp) => {
            log!(resp)
        }
        Msg::TestFailed(resp) => {
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
            h4!["To use instagram business account, select the Facebook page that is connected to your account \
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
                    "Available facebook page accounts",
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
                    span!["Business : ".to_owned() + &selected_account.name,]
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
                    "Get Instagram Account id ",
                    ev(Ev::Click, |_| { Msg::GetInstagramAccountId }),
                    attrs! {
                       At:: Disabled => model.selected_account.is_none().as_at_value()
                    }
                ],
                 button![
                    "Get Instagram Account details ",
                    ev(Ev::Click, |_| { Msg::FetchInstagramAccountId }),
                    attrs! {
                       At:: Disabled => model.selected_account.is_none().as_at_value()
                    }
                ]
            ],
            div![
                style! [

                   St:: Display => "flex",
                   St:: JustifyContent => "center",
                   St:: MarginTop =>  "20px"
                ],

               h4!["Account details"] ,
                if let Some(instagram) =  &model.instagram_account{
                    span!["Account name : ".to_owned()+ &instagram.name]
                }else{
                  p![""]

                }
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
                    St::  MarginLeft => px(15),
                   St:: Display => "flex",
                   St:: JustifyContent => "center",
                ],
                instagram_post_params(model)
            ],
        ]
    ]
}

fn instagram_post_params(model: &Model) -> Node<Msg> {
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
                Msg::UpdateInstaPostParams(
                    insta_post_params(post_url, "".to_string(), "#DJ, #live".to_string()),
                    "url".to_string(),
                )
                // initiate or update/ build  feed struct
            })
        ]],
        div![textarea![
            attrs! {
                At:: Placeholder => "Caption "
            },
            input_ev(Ev::Input, move |caption| {
                Msg::UpdateInstaPostParams(
                    insta_post_params("".to_string(), "".to_string(), caption),
                    "caption".to_string(),
                )
            })
        ]],
        div![textarea![
            attrs! {
                At:: Placeholder => "   Location tag id"
            },
            input_ev(Ev::Input, move |location_id| {
                Msg::UpdateInstaPostParams(
                    insta_post_params("".to_string(), location_id, "".to_string()),
                    "location".to_string(),
                )
            })
        ]],
        div![
            style![
                St:: MarginTop => px(10)
            ],
            button![
                " submit  container feed ",
                ev(Ev::Click, |_| { Msg::InstaMediaContainerInit })
            ],
        ],
        div![
            style! [

               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "10px"
            ],
            button![
                "Check container media status ",
                ev(Ev::Click, |_| { Msg::MediaContainerStatus }),
            ],
            if let Some(media_status) = &model.media_container_status {
                div![span![
                    "media status  ".to_owned() + &media_status.status_code
                ]]
            } else {
                div![span![
                    "Note: only published the item when the status is 'FINISHED', check above "
                        .to_owned()
                ]]
            }
        ],
        div![
            style![
                St:: MarginTop => px(10)
            ],
            button![
                "publish feed ",
                ev(Ev::Click, |_| { Msg::InstagramVideoPost }),
                attrs! {
                   At:: Disabled => model.media_container_status.is_none().as_at_value(),
                },
            ],
        ],
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
                        Msg::UpdateSelectedAccount(selected_account)
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
                        Msg::UpdateSelectedAccount(selected_account)
                    })
                ]
            },
            span!["Business : ".to_owned() + &account.name,],
        ]
    })]
}
