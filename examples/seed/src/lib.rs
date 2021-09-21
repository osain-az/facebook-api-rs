use facebook_api_rs::prelude::*;
//use seed::prelude::js_sys::to_string;
//use seed::prelude::js_sys::input;
use gloo_file::FileList;
use seed::{prelude::*, *};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{File, HtmlInputElement};
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

    let response = url.hash().map(|hash| Token::get_token(hash.to_string()));

    Model {
        redirect_url: RedirectURL::default(),
        error: None,
        response: response,
        image: None,
        accounts: None,
        pages_api: PagesAPI::default(),
        me: None,
        selectedAccount: None,
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

#[derive(Default)]
pub struct Model {
    redirect_url: RedirectURL,
    error: Option<String>,
    response: Option<Token>,
    image: Option<Data<Image>>,
    accounts: Option<Data<Accounts>>,
    pages_api: PagesAPI,
    me: Option<Data<Me>>,
    selectedAccount: Option<SelectedAccount>,
    post_type: String, // indicates the type of post the user wants to make
    post_data: Option<PostData>,
    feed_post_response: Option<FeedPostSuccess>,
    get_post_response: Option<GetPost>,
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
    UpdateSelectedccount(SelectedAccount),
    FacebookPostType(String),
    PostFaceebookFeed,
    UpdatePostData(PostData),
    PostSuccess(FeedPostSuccess),
    PostFailed(FetchError),
    GetPostSuccess(GetPost),
    GetPostFailed(FetchError),
    PostVideoByUrl(String),
    PostVideoSucces(VideoPostResponse),
    PostVideoFailed(FetchError),
    SubmitPost,
    FileUpload(Option<File>),
    VideoUploadByFileSucess(PostResponse),
    VideoUploadByFileFailed(FetchError),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ConfigFetched(Ok(config)) => model.redirect_url = RedirectURL::new(config).add_response_type("token").add_scope(&["email".to_string()]).add_full_url(),
        Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client_id and redirect_uri", fetch_error),
        Msg::GetProfilePicture => {
            if let Some(response) = &model.response {
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

        Msg::GetMe => {
            if let Some(response) = &model.response {
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
            if let Some(response) = &model.response {
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

        Msg::GetMeFailed(_) => {
        }

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
                let token = page_token.access_token.clone();
                let page_id = page_token.id.clone();

                if let Some(post_message) = &model.post_data{
                    let post_description = post_message.message.clone();

                   orders.perform_cmd(async move {
                      Client::new(Token::default())
                          .post(page_id,token)
                          .feed_post(&post_description)
                          .await
                          .map_or_else(Msg::PostFailed,Msg::PostSuccess)

                    });
                }

                // }
            }
        }
        Msg::SubmitPost => {
            //  handle this in a sepearte functiom
            log!(model.post_type);
              if model.post_type =="feed" {
                  if let Some(page_token) = &model.selectedAccount {
                      let token = page_token.access_token.clone();
                      let page_id = page_token.id.clone();

                      if let Some(post_message) = &model.post_data{
                          let post_description = post_message.message.clone();

                          orders.perform_cmd(async move {
                              Client::new(Token::default())
                                  .post(page_id,token)
                                  .feed_post(&post_description)
                                  .await
                                  .map_or_else(Msg::PostFailed,Msg::PostSuccess)

                          });
                      }

                      // }
                  }
              } else if model.post_type =="video" {
                  // this is for posting video with hosted fline ( with  a url)
                  if let Some( video_url)  = &model.post_data{
                      let video_url = video_url.video_url.to_owned();
                      if let Some(selected_page ) = &model.selectedAccount{
                          let page_token   =  selected_page.access_token.to_owned();
                          let page_id = selected_page.id.to_owned();
                          orders.perform_cmd(async move {
                              Client::new(Token::default())
                                  .get_post_video_link(page_id,&page_token)
                                  .post_by_link(&video_url)
                                  .await
                               .map_or_else(Msg::PostVideoFailed,Msg::PostVideoSucces)
                          });
                      }
                  }
              }

        }
        Msg::FacebookPostType(post_type) =>{

            model.post_type= post_type
        }
        Msg::UpdatePostData(post_data) =>{
            model.post_data=  Some( post_data);
            log!(model.post_data);
        }
        Msg:: PostSuccess(result) => {
            model.feed_post_response = Some(result); // store response id ( page_post_id)

            // make a new Get request to return the just posted Post
            if let Some( post_response)  = &model.feed_post_response{
                let page_post_id = post_response.id.to_string();
                if let Some(selected_page ) = &model.selectedAccount{
                      let page_token   =  selected_page.access_token.to_string();
                    orders.perform_cmd(async move  {
                        Client::new(Token::default())
                            .get_post(page_post_id,page_token)
                            .get()
                            .await
                            .map_or_else(Msg::GetPostFailed,Msg::GetPostSuccess)
                    });
                }
            }
        }
        Msg:: PostFailed(err) => {

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

        Msg:: PostVideoByUrl(file_url) => {

     }

        Msg::FileUpload(None)  => {

        }
     Msg::FileUpload(Some(file))  => {

         let fy = file.clone();
            let test_file = UploadFile{
             file: file
         };
       //  if let Some( video_url)  = &model.post_data{
             if let Some(selected_page ) = &model.selectedAccount{
                 let page_token   =  selected_page.access_token.to_owned();
                 let page_id = selected_page.id.to_owned();

                 // used the defaukt paramters
                 let video_params = VideoParams{

                     ..VideoParams::default()
                 };
                let uploaded_file =UploadFile{
                     file:fy
                 };
                 let tes = &uploaded_file;
                 orders.perform_cmd(async move {
                     Client::new(Token::default())
                         .post_video_file(page_id,&page_token)
                         .init_video_upload( uploaded_file, video_params)
                         .post_video(  test_file)
                         .await
                         .map_or_else(Msg:: VideoUploadByFileFailed,Msg:: VideoUploadByFileSucess)
                 });
             }

     }

        Msg:: VideoUploadByFileSucess(res) =>{
            log!(res)
        }

        Msg:: VideoUploadByFileFailed(res) =>{
            log!(res)
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
                    "Login with Facebook",
                    style! [
                     St:: Color => "white" ,
                     St:: BorderRadius => px(10),
                    St::MarginRight => px(10),
                    St:: BackgroundColor => "#192aa3d9",
                     St::Height => "50px",
                      St:: FontSize => "1.3em"
                    ],
                ]
            ],
            button![
                "Get my Profile Picture!",
                ev(Ev::Click, |_| { Msg::GetProfilePicture }),
                attrs! {
                    At:: Disabled => model.response.is_none().as_at_value()
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
                    At:: Disabled => model.response.is_none().as_at_value()
                }
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
                    option![attrs! {At::Value => ""}, "Select type post"],
                    option![attrs! {At::Value => "feed"}, "Post to feed"],
                    option![attrs! {At::Value => "video"}, "video post"],
                    option![attrs! {At::Value => "image"}, "Image post"],
                    option![attrs! {At::Value => "link"}, "Link post "],
                    input_ev(Ev::Input, Msg::FacebookPostType)
                ],
                button![
                    style! {

                       St:: MarginRight =>  "20px"
                    },
                    "Submit post",
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
                //initiate or update/ build  post struct
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
                //initiate or update/ build  post struct
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
                    //initiate or update/ build  post struct
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
                    //initiate or update/ build  post struct
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
                //initiate or update/ build  post struct
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
                    "Open post on facebook",
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
