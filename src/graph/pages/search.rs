//! The API let you search for different pages on facebook  which  including
//! names, locations, and more. Find Pages to @Mention, Page locations, and tag
//! a Page. For more information check <https://developers.facebook.com/docs/pages/searching>.

use crate::graph::accounts::Accounts;
use crate::prelude::HttpConnection;
use crate::universal::errors::ClientErr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct PagesAPI {
    pub page_access_token: String,
    pub page_id: String,
}

impl PagesAPI {
    pub fn new(accounts: Accounts) -> PagesAPI {
        PagesAPI::default()
            .add_page_access_token(accounts.access_token())
            .add_page_id(accounts.id())
    }

    pub fn add_page_access_token(mut self, page_access_token: &str) -> Self {
        self.page_access_token = page_access_token.to_string();
        self
    }

    pub fn add_page_id(mut self, page_id: &str) -> Self {
        self.page_id = page_id.to_string();
        self
    }

    pub fn access_token(&self) -> &String {
        &self.page_access_token
    }

    pub fn page_id(&self) -> &String {
        &self.page_id
    }

    pub fn set_page_access_token(&mut self, page_access_token: String) {
        self.page_access_token = page_access_token;
    }

    pub fn set_page_id(&mut self, page_id: String) {
        self.page_id = page_id;
    }
}

#[derive(Deserialize, Debug, Serialize, Default)]

pub struct PageSearch {
    pub name: String,
    pub id: String,
    pub location: Location,
    pub link: String,
}

/// this struct represent the data of the location of a page, Note: this data is
/// only available if the page has enabled the location.
#[derive(Deserialize, Serialize, Debug, Default)]
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

    /// This method is used to search for different facebook pages, which will
    /// return the struct as shown in the PageSearch
    pub async fn init_search(self) -> Result<PageSearch, ClientErr> {
        // this method has not be officially tested to be working properly since any
        // attempt to test return error off permission error due to the app
        // still in development mode

        //  note: g should be a dynamic value that will be pass in
        let q = "oslo";
        let url = self.base_url
            + "?q="
            + q
            + "&fields=id,name,location,link"
            + "&access_token="
            + &self.page_acess_token;

        let resp = HttpConnection::get::<PageSearch>(url, "".to_string()).await?;
        Ok(resp)
    }
}
