use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize)]
pub struct Accounts {
    pub access_token: String,
    category: String,
    category_list: Vec<String>,
    name: String,
    id: String,
    tasks: Vec<String>,
}

impl Accounts {
    pub fn add_access_token(mut self, access_token: &str) -> Self {
        self.access_token = access_token.to_string();
        self
    }

    pub fn add_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    pub fn add_category_list(mut self, category_list: &Vec<String>) -> Self {
        self.category_list = category_list.to_vec();
        self
    }

    pub fn add_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn add_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn add_tasks(mut self, tasks: &Vec<String>) -> Self {
        self.tasks = tasks.to_vec();
        self
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
    pub fn get_category(&self) -> &String {
        &self.category
    }
    pub fn get_category_list(&self) -> &Vec<String> {
        &self.category_list
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_tasks(&self) -> &Vec<String> {
        &self.tasks
    }
}
