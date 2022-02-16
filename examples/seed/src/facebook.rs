use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use web_sys::File;

use facebook_api_rs::prelude::errors::ClientErr;
use facebook_api_rs::prelude::feed::FeedPostSuccess;
use facebook_api_rs::prelude::search::PagesAPI;
use facebook_api_rs::prelude::utils::{GetPostResponse, PostResponse};
use facebook_api_rs::prelude::video::{FinalResponeResumableUpload, VideoParams};
use facebook_api_rs::prelude::*;

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
        post_type: "".to_string(),
        post_data: None,
        feed_post_response: None,
        get_post_response: None,
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

#[derive(Debug)]
pub struct PostData {
    message: String,
    photo_url: String,
    link_url: String, // this for external link
    video_url: String,
}

#[derive(Debug, Clone)]
struct InstaPostData {
    url: String,
    caption: String,
    location_id: String, // this should be coded
}

fn build_post(message: String, photo_url: String, link_url: String, video_url: String) -> PostData {
    PostData {
        message,
        photo_url,
        link_url,
        video_url,
    }
}

#[derive(Default)]
pub struct Model {
    pub user_tokens: Option<Token>,
    pub accounts: Option<Data<Accounts>>,
    pub pages_api: PagesAPI,
    pub selected_account: Option<SelectedAccount>,
    pub post_type: String, // indicates the type of feed the user wants to make
    pub post_data: Option<PostData>,
    pub feed_post_response: Option<FeedPostSuccess>,
    pub get_post_response: Option<GetPostResponse>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UpdateSelectedccount(SelectedAccount),

    FacebookPostType(String),
    UpdatePostData(PostData),
    PostSuccess(FeedPostSuccess),
    GetPostSuccess(GetPostResponse),
    SubmitPost,

    PostVideoSucces(FinalResponeResumableUpload),
    NoneResumableUpload(Option<File>),
    VideoUploadByFileSucess(PostResponse),
    ResumableUpload(Option<File>),
    ResumableUploadSucess(FinalResponeResumableUpload),

    // every error should user this
    ResponseFailed(ClientErr),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UpdateSelectedccount(account) => {
            log!(&account);
            model.selected_account = Some(account);
        }
        // This method is used to post your feed, Note, to post a video used  the different  methods
        // that support video posting
        Msg::SubmitPost => {
            //  handle this in a sepearte functiom
            log!(model.post_type);
            if model.post_type == "feed" {
                if let Some(page_token) = &model.selected_account {
                    let page_access_token = page_token.access_token.clone();
                    let page_id = page_token.id.clone();

                    if let Some(post_message) = &model.post_data {
                        let post_description = post_message.message.clone();

                        orders.perform_cmd(async move {
                            Client::new(Token::default(), page_access_token)
                                .feeds(page_id)
                                .post(&post_description)
                                .await
                                .map_or_else(Msg::ResponseFailed, Msg::PostSuccess)
                        });
                    }
                }
            } else if model.post_type == "video" {
                // this is for posting video with hosted fline ( with  a url)
                if let Some(video_url) = &model.post_data {
                    let video_url = video_url.video_url.to_owned();
                    if let Some(selected_page) = &model.selected_account {
                        let page_access_token = selected_page.access_token.to_owned();
                        let page_id = selected_page.id.to_owned();
                        orders.perform_cmd(async move {
                            Client::new(Token::default(), page_access_token)
                                .video_upload(page_id)
                                .post_by_link(&video_url, "", "", "video")
                                .await
                                .map_or_else(Msg::ResponseFailed, Msg::PostVideoSucces)
                        });
                    }
                }
            }
        }

        Msg::FacebookPostType(post_type) => model.post_type = post_type,

        Msg::UpdatePostData(post_data) => {
            model.post_data = Some(post_data);
            log!(model.post_data);
        }

        // This method used to get single post from your  feed
        Msg::PostSuccess(result) => {
            model.feed_post_response = Some(result); // store response id ( page_post_id)

            // make a new Get request to return the just posted Post
            if let Some(post_response) = &model.feed_post_response {
                let page_post_id = post_response.id.to_string();
                if let Some(selected_page) = &model.selected_account {
                    let page_access_token = selected_page.access_token.to_string();
                    orders.perform_cmd(async move {
                        Client::new(Token::default(), page_access_token)
                            .post(page_post_id)
                            .get()
                            .await
                            .map_or_else(Msg::ResponseFailed, Msg::GetPostSuccess)
                    });
                }
            }
        }

        Msg::GetPostSuccess(response) => {
            model.get_post_response = Some(response);
            log!(model.get_post_response);
        }

        Msg::PostVideoSucces(_) => {
            log!(model.get_post_response);
        }

        Msg::NoneResumableUpload(None) => {}
        // The NoneResumableUpload upload post videos to your page, this method is limited less than
        // 1gb size of video
        Msg::NoneResumableUpload(Some(file)) => {
            let file_uploaded = file;
            if let Some(selected_page) = &model.selected_account {
                let page_access_token = selected_page.access_token.to_owned();
                let page_id = selected_page.id.to_owned();

                // used the defaukt paramters
                let video_params = VideoParams {
                    ..VideoParams::default()
                };

                orders.perform_cmd(async move {
                    //Todo:File uplaod does not work with reqwest_async feature  yet

                    // Client::new(Token::default(), page_access_token)
                    //     .video_upload(page_id)
                    //     .non_resumable_post(video_params, file_uploaded)
                    //     .await
                    //     .map_or_else(Msg::ResponseFailed, Msg::VideoUploadByFileSucess)
                });
            }
        }
        Msg::VideoUploadByFileSucess(res) => {
            log!(res)
        }

        Msg::ResumableUpload(None) => {}

        // this method (resumable  upload) is used to upload videos to facebook page by chuncking
        // the file, for more information check the documenations
        Msg::ResumableUpload(Some(file)) => {
            let file_uploaded = file;
            //  if let Some( video_url)  = &model.post_data{
            if let Some(selected_page) = &model.selected_account {
                let page_access_token = selected_page.access_token.to_owned();
                let page_id = selected_page.id.to_owned();

                // used the defaukt paramters
                let video_params = VideoParams {
                    ..VideoParams::default()
                };

                orders.perform_cmd(async move {
                    //Todo:File uplaod does not work with reqwest_async feature  yet
                    Client::new(Token::default(), page_access_token)
                    /* .video_upload(page_id)
                    .resumable_post(file_uploaded, video_params)
                    .await
                    .map_or_else(Msg::ResponseFailed, Msg::ResumableUploadSucess)*/
                });
            }
        }

        Msg::ResumableUploadSucess(res) => {
            log!("final response ", res)
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
            style! [
               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            ],
            div![
                style! [
                  St:: MarginLeft =>  px(20)
                ],
                h3!["Available accounts :",],
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
                h3!["Selected  account ",],
                if let Some(selected_account) = &model.selected_account {
                    span!["Businnes : ".to_owned() + &selected_account.name,]
                } else {
                    div![""]
                }
            ],
        ],
        div![
            h2![
                "Post section",
                style! [
                    St:: Width => percent(100),
                   St:: TextAlign => "center",

                ],
            ],
            div![
                style! {
                    St:: Display => "flex",
                   St:: JustifyContent => "center",
                   St:: MarginTop =>  "20px"
                },
                select![
                    option![attrs! {At::Value => ""}, "Select type feed"],
                    option![attrs! {At::Value => "feed"}, "Post to feed"],
                    option![attrs! {At::Value => "video"}, "video feed"],
                    option![attrs! {At::Value => "image"}, "Image feed"],
                    option![attrs! {At::Value => "link"}, "Link feed "],
                    input_ev(Ev::Input, Msg::FacebookPostType)
                ],
                button![
                    style! {
                       St:: MarginRight =>  "20px"
                    },
                    "Submit feed",
                    ev(Ev::Click, |_| Msg::SubmitPost),
                    attrs! {
                        At:: Disabled =>  model.selected_account.is_none().as_at_value()
                    },
                ],
                post_input(model)
            ]
        ],
        div![
            h4![
                "Recent Post ",
                style![
                     St:: Width => percent(100),
                    St:: TextAlign => "center",
                ]
            ],
            display_recent_post(model)
        ],
        div![
            style! {
                St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            },
            div![
                h5!["Non resumable upload"],
                input![
                    C!["fileput_field"],
                    attrs! {
                        At::Type => "file",
                    },
                    id!["fileput_field"],
                    ev(Ev::Change, |e| {
                        let file = e
                            .target()
                            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                            .and_then(|file_input| file_input.files())
                            .and_then(|file_list| file_list.get(0));
                        Msg::NoneResumableUpload(file)
                    })
                ],
            ],
            div![
                h5![" Resumable upload"],
                input![
                    C!["resumable file upload"],
                    attrs! {
                        At::Type => "file",
                    },
                    id!["fileput_field"],
                    ev(Ev::Change, |e| {
                        let file = e
                            .target()
                            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                            .and_then(|file_input| file_input.files())
                            .and_then(|file_list| file_list.get(0));
                        Msg::ResumableUpload(file)
                    })
                ],
            ],
        ],
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
            // span!["Businnes : ".to_owned() + testing.name,],
            span!["Businnes : ".to_owned() + &account.name,],
        ]
    })]
}

fn post_input(model: &Model) -> Node<Msg> {
    if model.post_type == "feed" {
        textarea![
            attrs! {
                At:: Placeholder => "Enter a feed decription ",
                At:: Disabled =>  model.selected_account.is_none().as_at_value(),
                At:: Required => true
            },
            input_ev(Ev::Input, move |message| {
                Msg::UpdatePostData(build_post(
                    message,
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ))
                // initiate or update/ build  feed struct
            })
        ]
    } else if model.post_type == "video" {
        textarea![
            attrs! {
                At:: Placeholder => "Enter a video url ",
                At:: Disabled =>  model.selected_account.is_none().as_at_value()
            },
            input_ev(Ev::Input, move |vaule| {
                Msg::UpdatePostData(build_post(
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    vaule,
                ))
                // initiate or update/ build  feed struct
            })
        ]
    } else if model.post_type == "link" {
        div![
            textarea![
                attrs! {
                    At:: Placeholder => "Enter a description ",
                    At:: Disabled =>  model.selected_account.is_none().as_at_value()
                },
                input_ev(Ev::Input, move |message| {
                    Msg::UpdatePostData(build_post(
                        message,
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ))
                    // initiate or update/ build  feed struct
                })
            ],
            textarea![
                attrs! {
                    At:: Placeholder => "Enter a link",
                    At:: Disabled =>  model.selected_account.is_none().as_at_value()

                },
                input_ev(Ev::Input, move |link| {
                    Msg::UpdatePostData(build_post(
                        "".to_string(),
                        "".to_string(),
                        link,
                        "".to_string(),
                    ))
                    // initiate or update/ build  feed struct
                })
            ]
        ]
    } else if model.post_type == "image" {
        textarea![
            attrs! {
                At:: Placeholder => "Enter a image url"
            },
            input_ev(Ev::Input, move |photo_url| {
                Msg::UpdatePostData(build_post(
                    "".to_string(),
                    photo_url,
                    "".to_string(),
                    "".to_string(),
                ))
                // initiate or update/ build  feed struct
            })
        ]
    } else {
        input![attrs! {
            At::Disabled => true
        }]
    }
}

fn display_recent_post(model: &Model) -> Node<Msg> {
    if let Some(recent_post) = &model.get_post_response {
        div![
            h4![
                "Page Name :  ".to_owned() + recent_post.from.name.as_str(),
                style![
                    St:: TextAlign => "center"
                ],
                // attrs![At::Href]
            ],
            p![
                "Post description :  ".to_owned() + recent_post.message.as_str(),
                style![
                    St::MarginRight=>"auto" ,
                    St:: MarginLeft => "auto",
                    St:: TextAlign =>"center"
                ]
            ],
            a![
                attrs! {
                    At:: Target => "_blank",
                    At:: Href => recent_post.permalink_url.as_str()
                },
                button![
                    "Open feed on facebook",
                    style![

                        St::MarginRight=>"auto" ,
                        St:: MarginLeft => "auto",
                        St:: Display => "block"
                    ]
                ],
            ],
            style![
                 St:: Width => percent(100),
                St:: Border => "solid, pink , 2px",
                St::  BorderRadius => px(5 )
            ]
        ]
    } else {
        div![]
    }
}
