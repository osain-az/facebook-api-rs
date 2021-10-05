///The Pages API allows apps to access and update a Facebook Page's settings and content,
/// create and get Posts, get Comments on Page owned content, get Page insights, update actions that
/// Users are able to perform on a Page, and much more.
///
///
use crate::graph::accounts::Accounts;
use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct PagesAPI {
    pub page_access_token: String,
    pub page_id: String,
}

impl PagesAPI {
    pub fn new(accounts: Accounts) -> PagesAPI {
        PagesAPI::default()
            .add_page_access_token(&accounts.get_access_token())
            .add_page_id(&accounts.get_id())
    }

    pub fn add_page_access_token(mut self, page_access_token: &str) -> Self {
        self.page_access_token = page_access_token.to_string();
        self
    }

    pub fn add_page_id(mut self, page_id: &str) -> Self {
        self.page_id = page_id.to_string();
        self
    }

    pub fn get_access_token(&self) -> &String {
        &self.page_access_token
    }

    pub fn get_page_id(&self) -> &String {
        &self.page_id
    }

    pub fn set_page_access_token(&mut self, page_access_token: String) {
        self.page_access_token = page_access_token;
    }

    pub fn set_page_id(&mut self, page_id: String) {
        self.page_id = page_id;
    }
}

#[derive(Deserialize, Serialize, Default)]

pub struct PageSearch {
    pub name: String,
    pub id: String,
    pub location: Location,
    pub link: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub latitude: u64,
    pub longitude: u64,
    pub state: String,
    pub street: String,
    pub zip: String,
}

pub struct PagesSearchAPI {
    pub page_acess_token: String,
    pub base_url: String,
}

impl PagesSearchAPI {
    pub fn new(base_url: String, page_acess_token: String) -> PagesSearchAPI {
        PagesSearchAPI {
            base_url,
            page_acess_token,
        }
    }

    ///
    /// This method is used to search for different facebook pages, which will return the struct as shown
    /// in the PageSearch
    ///
    pub async fn init_search(self) -> seed::fetch::Result<PageSearch> {
        // this method has not be offically tested to be working proper since any attempt to test return error off
        // permission error due to the app still in development mode

        // the note should be a dynamica value that will be pass in
        let url = self.base_url
            + "?q=Oslo"
            + "&fields=id,name,location,link"
            + "&access_token="
            + &self.page_acess_token;

        let request = Request::new(url).method(Method::Get);
        fetch(request).await?.json::<PageSearch>().await
    }
}
