use seed::{*, prelude::*};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{File, HtmlInputElement};

use facebook_api_rs::prelude::*;
use facebook_api_rs::prelude::account::InstaAccount;
use facebook_api_rs::prelude::feed::{FeedPostSuccess};
use facebook_api_rs::prelude::publish::{InstaMediaConatiner, InstaPostParams};
use facebook_api_rs::prelude::search::{PagesAPI, PageSearch};
use facebook_api_rs::prelude::utils::GetPostResponse;
use facebook_api_rs::prelude::video::{FinalResponeResumableUpload, PostResponse, UploadFile, VideoParams};
use Msg::GetInstaAccountSuccess;

//use seed::prelude::js_sys::to_string;
//use seed::prelude::js_sys::input;
use crate::Msg::GetInstaAccountFailed;

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

    let response = url.hash().map(|hash| Token::extract_user_tokens(hash.to_string()));
    log!("response", url.hash());

    Model {
        redirect_url: RedirectURL::default(),
        error: None,
        user_tokens: response,
        image: None,
        accounts: None,
        pages_api: PagesAPI::default(),
        me: None,
        selectedAccount: None,
        post_type: "".to_string(),
        post_data: None,
        feed_post_response: None,
        get_post_response: None,
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

// TODO simplify struct and function for insta

#[derive(Debug, Clone)]
struct SelectedAccount {
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

impl PostData {
    fn get_message(&self) -> &str {
        &self.message
    }
    fn get_photo(&self) -> &str {
        &self.photo_url
    }
    fn get_video_url(&self) -> &str {
        &self.video_url
    }
    fn get_link_url(&self) -> &str {
        &self.link_url
    }
}

#[derive(Debug, Clone)]
struct InstaPostData {
    url: String,
    caption: String,
    location_id: String, // this should be coded
}


fn build_Post(message: String, photo_url: String, link_url: String, video_url: String) -> PostData {
    PostData {
        ..PostData {
            message,
            photo_url,
            link_url,
            video_url,
        }
    }
}

// build str
fn insta_post_params(url: String, location_id: String, caption: String) -> InstaPostParams {
    InstaPostParams {
        url,
        caption,
        location_id,
        tag_users: vec![],
    }
}
#[derive(Debug, Default, Clone)]
struct InstaPostingOption {
    caption: bool,
    location_tag: bool,
    is_post_video: bool,
    tag_users: bool,
}
//  TODO make 2 pages for facebook and insta
#[derive(Default)]
pub struct Model {
    redirect_url: RedirectURL,
    error: Option<String>,
    user_tokens: Option<Token>,
    image: Option<Data<Image>>,
    accounts: Option<Data<Accounts>>,
    pages_api: PagesAPI,
    me: Option<Data<Me>>,
    selectedAccount: Option<SelectedAccount>,
    post_type: String, // indicates the type of feed the user wants to make
    post_data: Option<PostData>,
    feed_post_response: Option<FeedPostSuccess>,
    get_post_response: Option<GetPostResponse>,

    insta_account: Option<InstaAccount>,
    insta_post_param: Option<InstaPostParams>,
    insta_media_container_id: Option<InstaMediaConatiner>,
    insta_posting_options: InstaPostingOption,

    access_token_info: Option< AccessTokenInformation>
}


// ------ ------
//    Update
// ------ ------
// TODO simplify messages



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
    UpdateSelectedccount(SelectedAccount),
    FacebookPostType(String),
    PostFaceebookFeed,
    UpdatePostData(PostData),
    PostSuccess(FeedPostSuccess),
    PostFailed(FetchError),
    GetPostSuccess(GetPostResponse),
    GetPostFailed(FetchError),
    PostVideoByUrl(String),
    PostVideoSucces(FinalResponeResumableUpload),
    PostVideoFailed(FetchError),
    SubmitPost,
    FileUpload(Option<File>),
    VideoUploadByFileSucess(PostResponse),
    VideoUploadByFileFailed(FetchError),
    ResumableUpload(Option<File>),
    ResumableUploadSucess(FinalResponeResumableUpload),
    ResumableUploadFailed(FetchError),

    //Instagram operations
    GetInstaAcoount,
    InstaMediaConatinerInit,
    InstaContainerResponse(InstaMediaConatiner),
    GetInstaAccountSuccess(InstaAccount),
    GetInstaAccountFailed(FetchError),
    UpdatInstaPostParams(InstaPostParams),
    InstagramVideoPost,
    InstaPostSucessful(InstaMediaConatiner),
    InstaPostFailed(FetchError),
    HandleInstaPostingOption(web_sys::Event),
    PagesSearch(String),
    PageSearchResponse(PageSearch),


     AccessTokenInformation,
    AccessTokenInfoData(AccessTokenInformation),


    // every error should user this
    ResponseFailed(FetchError),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ConfigFetched(Ok(config)) => model.redirect_url = RedirectURL::new(config).add_response_type("token").add_scope(&["email".to_string()]).add_full_url(),
        Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client_id and redirect_uri", fetch_error),
      //TODO remove or have the irght api call
        Msg::GetProfilePicture => {
            if let Some(response) = &model.user_tokens {
                let url = "https://graph.facebook.com/v11.0/me/picture?access_token=".to_string() + &response.access_token + "&format=json" + "&redirect=false";
                let request = fetch::Request::new(url).method(Method::Get);
                orders.perform_cmd(async {
                    fetch(request).await.unwrap().json::<Data<Image>>().await.map_or_else(Msg::GetProfilePictureFailed, Msg::GetProfilePictureSuccess)
                }
                );
            }
        },
        Msg::GetProfilePictureSuccess(image) => {
            model.image = Some(image)
        },
        Msg::GetProfilePictureFailed(_) => {}
        //TODO remove
        Msg::GetMe => {
            if let Some(response) = &model.user_tokens {
                let url = "https://graph.facebook.com/v11.0/me?access_token=".to_string() + &response.access_token;
                let request = fetch::Request::new(url).method(Method::Get);
                orders.perform_cmd(async {
                    fetch(request).await
                        .unwrap()
                        .json::<Data<Me>>()
                        .await
                        .map_or_else(Msg::GetMeFailed, Msg::GetMeSuccess)
                }
                );
            }
        },

        Msg::GetAccount => {
            if let Some(user_access_tokens) = model.user_tokens.clone() {
                let user_tokens = user_access_tokens;
                let client = Client::new(user_tokens,"".to_string());
                orders.perform_cmd(async {
                    // we are interested in the page love live token, therefore we called the long live methed
                    // by passing "long_live_token" to the method
                    client.me_by_short_or_long_live_token("long_live_token".to_string())
                        .accounts()
                        .get()
                        .await
                        .map_or_else(Msg::ResponseFailed, Msg::GetAccountSuccess)
                });
            }
        },

        Msg::GetMeSuccess(me) => {
            model.me = Some(me);
            log!(model.me );
        }

        Msg::GetMeFailed(_) => {}

        Msg::GetAccountSuccess(account) => {
            model.accounts = Some(account);
            log!(model.accounts)
        },

        Msg::GetAccountFailed(err) => {
            log!("account failed");
            log!(err);
        }

        Msg::UpdateSelectedccount(account) => {
            model.selectedAccount = Some(account);
            log!(model.selectedAccount);
        }

        Msg::PostFaceebookFeed => {
            if let Some(page_token) = &model.selectedAccount {
                let page_access_token = page_token.access_token.clone();
                let page_id = page_token.id.clone();

                if let Some(post_message) = &model.post_data {
                    let post_description = post_message.message.clone();

                    orders.perform_cmd(async move {
                        Client::new(Token::default(),page_access_token)
                            .feeds(page_id)
                            .post(&post_description)
                            .await
                            .map_or_else(Msg::PostFailed, Msg::PostSuccess)
                    });
                }

                // }
            }
        }
        Msg::SubmitPost => {
            //  handle this in a sepearte functiom
            log!(model.post_type);
            if model.post_type == "feed" {
                if let Some(page_token) = &model.selectedAccount {
                    let page_access_token = page_token.access_token.clone();
                    let page_id = page_token.id.clone();

                    if let Some(post_message) = &model.post_data {
                        let post_description = post_message.message.clone();

                        orders.perform_cmd(async move {
                            Client::new(Token::default(),page_access_token)
                                .feeds(page_id)
                                .post(&post_description)
                                .await
                                .map_or_else(Msg::PostFailed, Msg::PostSuccess)
                        });
                    }

                    // }
                }
            } else if model.post_type == "video" {
                // this is for posting video with hosted fline ( with  a url)
                if let Some(video_url) = &model.post_data {
                    let video_url = video_url.video_url.to_owned();
                    if let Some(selected_page) = &model.selectedAccount {
                        let page_access_token = selected_page.access_token.to_owned();
                        let page_id = selected_page.id.to_owned();
                        orders.perform_cmd(async move {
                            Client::new(Token::default(),page_access_token)
                                .video_upload(page_id)
                                .post_by_link(&video_url)
                                .await
                                .map_or_else(Msg::ResponseFailed, Msg::PostVideoSucces)
                        });
                    }
                }
            }
        }
        Msg::FacebookPostType(post_type) => {
            model.post_type = post_type
        }
        Msg::UpdatePostData(post_data) => {
            model.post_data = Some(post_data);
            log!(model.post_data);
        }
        Msg::PostSuccess(result) => {
            model.feed_post_response = Some(result); // store response id ( page_post_id)

            // make a new Get request to return the just posted Post
            if let Some(post_response) = &model.feed_post_response {
                let page_post_id = post_response.id.to_string();
                if let Some(selected_page) = &model.selectedAccount {
                    let page_access_token = selected_page.access_token.to_string();
                    orders.perform_cmd(async move {
                        Client::new(Token::default(),page_access_token)
                            .post(page_post_id)
                            .get()
                            .await
                            .map_or_else(Msg::ResponseFailed, Msg::GetPostSuccess)
                    });
                }
            }
        }
        Msg::PostFailed(err) => {
            log!(err)
        }
        Msg::GetPostSuccess(response) => {
            model.get_post_response = Some(response);

            log!(model.get_post_response);
        }
        Msg::GetPostFailed(err) => {
            log!(err);
        }

        Msg::PostVideoSucces(response) => {
            //  model.get_post_response = Some(response);
            log!(model.get_post_response);
        }
        Msg::PostVideoFailed(err) => {
            log!(err);
        }

        Msg::PostVideoByUrl(file_url) => {}

        Msg::FileUpload(None) => {}
        Msg::FileUpload(Some(file)) => {
            let file_uploaded = file.clone();

            //  if let Some( video_url)  = &model.post_data{
            if let Some(selected_page) = &model.selectedAccount {
                let page_access_token = selected_page.access_token.to_owned();
                let page_id = selected_page.id.to_owned();

                // used the defaukt paramters
                let video_params = VideoParams {
                    ..VideoParams::default()
                };
                let uploaded_file = UploadFile {
                    file
                };
                orders.perform_cmd(async move {
                    //This test is for nonresumable
                    Client::new(Token::default(),page_access_token)
                        .video_upload(page_id)
                        .non_resumable_post(video_params, file_uploaded)
                        .await
                        .map_or_else(Msg::ResponseFailed, Msg::VideoUploadByFileSucess)
                });
            }
        }

        Msg::VideoUploadByFileSucess(res) => {
            log!(res)
        }

        Msg::VideoUploadByFileFailed(res) => {
            log!(" thisis the eeror", res) // This erro could error send from facebook or error generated when  larger video file was uploaded.
        }

        Msg::ResumableUpload(None) => {}

        Msg::ResumableUpload(Some(file)) => {
            let file_uploaded = file.clone();
            //  if let Some( video_url)  = &model.post_data{
            if let Some(selected_page) = &model.selectedAccount {
                let page_access_token = selected_page.access_token.to_owned();
                let page_id = selected_page.id.to_owned();

                // used the defaukt paramters
                let video_params = VideoParams {
                    ..VideoParams::default()
                };
                let uploaded_file = UploadFile {
                    file
                };
                orders.perform_cmd(async move {
                    Client::new(Token::default(),page_access_token)
                        .video_upload(page_id)
                        .resumable_post(file_uploaded, video_params)
                        .await
                        .map_or_else(Msg::ResponseFailed, Msg::ResumableUploadSucess)
                });
            }
        }

        Msg::ResumableUploadSucess(res) => {
            log!("final response ", res)
        }

        Msg::ResumableUploadFailed(res) => {
            log!( res)
        }

        //Instagram operation
        Msg::GetInstaAcoount => {
            if model.selectedAccount.is_some(){

                if let Some(selected_page) = &model.selectedAccount {
                    let page_access_token = selected_page.access_token.to_owned();
                    let facebook_page_id = selected_page.id.clone();

                    orders.perform_cmd(async move {
                        Client::new(Token::default(),page_access_token.clone())
                            .instagram_account( facebook_page_id )
                            .insta_account()
                            .await
                            .map_or_else(Msg::GetInstaAccountFailed, GetInstaAccountSuccess)
                    });
                }


            }
        }

      Msg::  GetInstaAccountSuccess(resp) => {
            model.insta_account = Some(resp);

        }

     Msg::   GetInstaAccountFailed(resp) => {

        }
    Msg:: UpdatInstaPostParams( post_input) => {

        model.insta_post_param = Some(post_input);
        log!(model.insta_post_param);
    }
        Msg::InstagramVideoPost => {

            if let Some(selected_page)  = &model.selectedAccount{

         /*       if let Some(instag_account) = &model.insta_account {
                    log!(instag_account);
                    let page_token = selected_page.access_token.to_owned();
                    let page_id = selected_page.id.to_owned();
                    let insta_page_id = instag_account.instagram_business_account.id.clone();

                    if let Some(post_param) = model.insta_post_param.clone() {
                        orders.perform_cmd(async move {
                            Client::new(Token::default())
                                .instagram(insta_page_id, &page_token)
                                .post_video(post_param)
                                .await
                                .map_or_else(Msg::InstaPostFailed, Msg::InstaPostSucessful)
                        });
                    }
                }


            }
*/
                if let Some(selected_page)  = &model.selectedAccount {
                    if let Some(instag_account) = &model.insta_account {
                        if let Some(insta_media_container_id) = &model.insta_media_container_id {
                            let page_access_token = selected_page.access_token.to_owned();
                            let page_id = selected_page.id.to_owned();
                            let insta_page_id = instag_account.instagram_business_account.id.clone();
                            let insta_media_conatiner = insta_media_container_id.id.clone();
                            if let Some(post_param) = model.insta_post_param.clone() {
                                orders.perform_cmd(async move {
                                    Client::new(Token::default(),page_access_token)
                                        .instagram(insta_page_id, )
                                        .publish_video(insta_media_conatiner)
                                        .await
                                        .map_or_else(Msg::InstaPostFailed, Msg::InstaPostSucessful)
                                });
                            }
                        }
                    }
                }

                       }

        }
        Msg:: InstaPostSucessful(resp) => {
          log!(resp)
        }

        Msg:: InstaPostFailed(resp) => {
            log!(resp)
        }

        Msg:: InstaMediaConatinerInit => {

            if let Some(selected_page)  = &model.selectedAccount{

                if let Some(instag_account) = &model.insta_account {
                    log!(instag_account);
                    let page_access_token = selected_page.access_token.to_owned();
                    let page_id = selected_page.id.to_owned();
                    let insta_page_id = instag_account.instagram_business_account.id.clone();

                    if let Some(post_param) = model.insta_post_param.clone() {
                        orders.perform_cmd(async move {
                            Client::new(Token::default(),page_access_token)
                                .instagram(insta_page_id)
                                .ig_media_container(post_param, "video".to_string())
                                .await
                                .map_or_else(Msg::InstaPostFailed, Msg::InstaContainerResponse)
                        });
                    }
                }


            }


        }

        Msg::InstaContainerResponse(resp) => {
             model.insta_media_container_id = Some(resp);
            log!(model.insta_media_container_id);

        }

        Msg::  HandleInstaPostingOption(e) =>{

            let checked  = e
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|file_input| file_input.checked()).unwrap();
            let event_type = e
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|file_input| file_input.name()).unwrap();

            if event_type =="is_post_video"{
                if checked {
                    model.insta_posting_options.is_post_video = false;
                }else {
                    model.insta_posting_options.is_post_video = true;
                }
            }
            else if event_type =="location_tag" {
                if checked {
                    model.insta_posting_options.location_tag = true;
                }else {
                    model.insta_posting_options.location_tag  = false;
                };
            }
            else if event_type =="tag_user" {
                if checked {
                    model.insta_posting_options.tag_users = true;
                }else {
                    model.insta_posting_options.tag_users  = false;
                };
            };
            log!(checked)
        }
        Msg:: PagesSearch(e) => {
            log!(e);

            if let Some(selected_page)  = &model.selectedAccount{

                if let Some(instag_account) = &model.insta_account {
                    log!(instag_account);
                    let page_access_token = selected_page.access_token.to_owned();
                    let page_id = selected_page.id.to_owned();
                    let insta_page_id = instag_account.instagram_business_account.id.clone();

                        log!(e);
                        orders.perform_cmd(async move {
                            Client::new(Token::default(),page_access_token)
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


        Msg::AccessTokenInformation  => {

                if let Some(user_tokens) = &model.user_tokens {
                    let short_live_token = user_tokens.access_token.to_owned();
                    let long_live_token = user_tokens.long_lived_token.to_owned();

                     log!(short_live_token==long_live_token);
                   orders.perform_cmd(async move {
                         AccessTokenInformation::access_token_information(short_live_token, long_live_token)
                            .await
                            .map_or_else(Msg::ResponseFailed, Msg::AccessTokenInfoData)
                    });

            }
        }

        Msg::AccessTokenInfoData(resp) => {
            log!(resp)
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
            "facebook Api example",
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
                            At::Src => "src/blue_58.png",
                           At:: Width => 40,
                            At:: Height => 40,

                          // At::Src => "src/login_button.png", // attribute <a href="https://www.freeiconspng.com/img/18026">Facebook login button png</a>
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
            button![
                "Get my Profile Picture!",
                ev(Ev::Click, |_| { Msg::GetProfilePicture }),
                attrs! {
                    At:: Disabled => model.user_tokens.is_none().as_at_value()
                },
                style! {
                    St::Height => "50px",
                    St::MarginRight => px(10),
                },
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
                ev(Ev::Click, |_| { Msg:: AccessTokenInformation}),
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
            div![
                style! [

                //St:: MarginTop =>  "20px"
                  St:: MarginLeft =>  px(20)
                ],
                h3![
                    "Avaliable accounts ",
                    attrs! {
                       // At
                    }
                ],
                if let Some(accounts) = &model.accounts {
                    // if let Some(selected_account) = &model.selectedAccount {

                    display_account(accounts, model)
                } else {
                    div![""]
                }
            ],
            div![
                style! [

                //St:: MarginTop =>  "20px"
                  St:: MarginLeft =>  px(20)
                ],
                h3![
                    "Selected  account ",
                    attrs! {
                       // At
                    }
                ],
                if let Some(selected_account) = &model.selectedAccount {
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
                        At:: Disabled =>  model.selectedAccount.is_none().as_at_value()
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
                        Msg::FileUpload(file)
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
                       At:: Disabled => model.selectedAccount.is_none().as_at_value()
                    }
                ]
            ],
            insta_post_options(model),
            if model.insta_posting_options.location_tag == true {
                pages_search(model)
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

fn display_account(accounts: &Data<Accounts>, model: &Model) -> Node<Msg> {
    div![accounts.data.iter().map(|account| {
        let selected_account = SelectedAccount {
            name: String::from(&account.name),
            id: String::from(&account.id),
            access_token: String::from(&account.access_token),
        };

        div![
            if let Some(selected_account_in_model) = &model.selectedAccount {
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
                At:: Disabled =>  model.selectedAccount.is_none().as_at_value(),
                At:: Required => true
            },
            input_ev(Ev::Input, move |message| {
                Msg::UpdatePostData(build_Post(
                    message,
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ))
                //initiate or update/ build  feed struct
            })
        ]
    } else if model.post_type == "video" {
        textarea![
            attrs! {
                At:: Placeholder => "Enter a video url ",
                At:: Disabled =>  model.selectedAccount.is_none().as_at_value()
            },
            input_ev(Ev::Input, move |vaule| {
                Msg::UpdatePostData(build_Post(
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    vaule,
                ))
                //initiate or update/ build  feed struct
            })
        ]
    } else if model.post_type == "link" {
        div![
            textarea![
                attrs! {
                    At:: Placeholder => "Enter a description ",
                    At:: Disabled =>  model.selectedAccount.is_none().as_at_value()
                },
                input_ev(Ev::Input, move |message| {
                    Msg::UpdatePostData(build_Post(
                        message,
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ))
                    //initiate or update/ build  feed struct
                })
            ],
            textarea![
                attrs! {
                    At:: Placeholder => "Enter a link",
                    At:: Disabled =>  model.selectedAccount.is_none().as_at_value()

                },
                input_ev(Ev::Input, move |link| {
                    Msg::UpdatePostData(build_Post(
                        "".to_string(),
                        "".to_string(),
                        link,
                        "".to_string(),
                    ))
                    //initiate or update/ build  feed struct
                })
            ]
        ]
    } else if model.post_type == "image" {
        textarea![
            attrs! {
                At:: Placeholder => "Enter a image url"
            },
            input_ev(Ev::Input, move |photo_url| {
                Msg::UpdatePostData(build_Post(
                    "".to_string(),
                    photo_url,
                    "".to_string(),
                    "".to_string(),
                ))
                //initiate or update/ build  feed struct
            })
        ]
    } else {
        Msg::UpdatePostData(build_Post(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ));
        input![attrs! {
            At::Disabled => true
        }]
    }
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

fn display_recent_post(model: &Model) -> Node<Msg> {
    if let Some(recent_post) = &model.get_post_response {
        div![
            h4![
                "Page Name :  ".to_owned() + recent_post.from.name.as_str(),
                style![
                    St:: TextAlign => "center"
                ],
                //attrs![At::Href]
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
                Msg::UpdatInstaPostParams(insta_post_params(
                    post_url,
                    "".to_string(),
                    "#DJ, #live".to_string(),
                ))
                //initiate or update/ build  feed struct
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
                //initiate or update/ build  feed struct
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
                //initiate or update/ build  feed struct
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

// TO tag  a location, the user need to make search of relevant pages with that location

fn pages_search(model: &Model) -> Node<Msg> {
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

//fn video_upload()

//ed::browser::dom::event_handler
//pub fn ev<Ms: 'static, MsU: 'static>(trigger: impl Into<Ev>, handler: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone) -> EventHandler<Ms>

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
