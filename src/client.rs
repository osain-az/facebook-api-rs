pub use crate::{
    code::*, data::*, image::*,image::*,redirect_url::*,redirect_url::*,token::*,
    extract_query_fragments::*,
};
use seed::{prelude::*, *};
use async_trait::async_trait;


//Seed Client Struct for making calls to Facebook Graph

#[derive(Debug)]
pub struct Client {
    graph: String,
    node: String,
    edge: String,
    fields: Vec<String>,
    access_token: String,
}

impl Default for Client {
    fn default() -> Self {
        let graph = "https://graph.facebook.com/".to_string();

        Self {
            graph,
            node: "".to_string(),
            edge:"".to_string(),
            fields: Vec::new(),
            access_token: "".to_string(),
        }
    }
}

/*


impl Client {
   async fn new_request (&self) -> seed::browser::fetch::Result<Response> {
        fetch(
            self.graph.to_string() + &*self.node.to_string() + &*self.fields.iter().cloned().collect::<String>() + &*self.access_token.to_string()
        ).await

    }
}

async fn start_login_flow (redirect_url: RedirectURL) -> seed::browser::fetch::Result<Response> {
    fetch(
        redirect_url.build_redirect_url_as_string()
    ).await
}


pub fn client_test() {

    let mut client_image = Client {
        graph: "https://graph.facebook.com/v11.0/".to_string(),
        node: "me/".to_string(),
        edge: "picture?".to_string(),
        fields: vec!["redirect=false".to_string()],
        access_token:"EAAHNQH0awn4BAFyuWUdO8gZCxNn4eF3nkZAoWZC7nKlzxS6YJGlnNSjHyZBmCVzrmR2e8YUmqYyaECogUksYDyMXt8vfN3YAqSgQTe6ocT8HucbftYSh3HZBnKZAR0UIwWcssFFoR24Jykconv5UZBuUdKpZAPl5IPwERC8Ranykw8gJfa9Gg795kmDJejtl6dzZCkZBc4rzGHhcTWQKsKuGSWkP1e83O2Il8yqNgf3FzRqaRG3dcO5ZATr".to_string()
    };

    let login = start_login_flow();

    let response = client_image.new_request();

    let token = get_token();



}









    fn test_graph() {

        let mut client = Client {
            graph: "https://graph.facebook.com/v11.0/".to_string(),
            node: "me/".to_string(),
            edge: "picture?".to_string(),
            fields: vec!["redirect=false".to_string()],
            access_token:"EAAHNQH0awn4BAFyuWUdO8gZCxNn4eF3nkZAoWZC7nKlzxS6YJGlnNSjHyZBmCVzrmR2e8YUmqYyaECogUksYDyMXt8vfN3YAqSgQTe6ocT8HucbftYSh3HZBnKZAR0UIwWcssFFoR24Jykconv5UZBuUdKpZAPl5IPwERC8Ranykw8gJfa9Gg795kmDJejtl6dzZCkZBc4rzGHhcTWQKsKuGSWkP1e83O2Il8yqNgf3FzRqaRG3dcO5ZATr".to_string()
        };


        let response =client.new_request();


        pub struct Image {
            height:u16,
            width:u16,
            is_silhouette:bool,
            pub url:String,
        }
        let image = Image {
            height: 0,
            width: 0,
            is_silhouette: false,
            url: "this is supposed to be an url".to_string()
        };


        }






*/
