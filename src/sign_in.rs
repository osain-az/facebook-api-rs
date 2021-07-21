
/*
use seed::{prelude::*, *};
use crate::RedirectURL;

pub fn facebook_button(model: &Mode, redirect_url: RedirectURL) -> Node<Msg> {

    let url = redirect_url.get_facebook_oath_url().to_string()+redirect_url.get_client_id()+redirect_url.get_redirect_uri()+redirect_url.get_state();

    [div![
        attrs!{At::Href => url},
        button![]
    ]



}
 */

/*
    let url = "https://connect.facebook.net/en_US/sdk.js#xfbml=1&version=v11.0".to_string();
    let url_nonce = "nonce=\"\#zBU0xm95\"\".to_string();
    url.push_str(&client_id);



    vec![div![id!("fb.jpeg-root"),],
          Script![attrs! {
           At::Src=>"https://connect.facebook.net/en_US/sdk.js#xfbml=1&version=v11.0".to_owned()+ &client_id +"&autoLogAppEvents=1" nonce="#zBU0xm95",
           At::Async=>true,
           At::Defer=>true
        }],


         div![C!("fb.jpeg-login-button"),
             attrs! {At::Type => "data-size", At::Src=>"100"}
        //     At::data-width="100",
          //   data-size="large",
          //   data-button-type="login_with",
         //    data-layout="default",
         //    data-auto-logout-link="false",
         //    data-use-continue-as="true",}
         ]


     ]
//}
*/
