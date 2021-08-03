use crate::prelude::Accounts;
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
