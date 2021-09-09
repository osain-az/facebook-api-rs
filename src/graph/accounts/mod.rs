use crate::graph::data::Data;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize)]
pub struct Accounts {
    pub access_token: String,
    category: String,
    category_list: Vec<ListDetails>,
    pub name: String,
    pub id: String,
    tasks: Vec<String>,
}
#[derive(Deserialize, Debug, Default, Serialize)]
pub struct ListDetails {
    id: String,
    name: String,
}

impl Accounts {
    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
    pub fn get_category(&self) -> &String {
        &self.category
    }
    pub fn get_category_list(&self) -> &Vec<ListDetails> {
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

pub struct AccountsAPI {
    url: String,
}

impl AccountsAPI {
    pub fn new(base_url: String) -> AccountsAPI {
        AccountsAPI {
            url: base_url.replace("EDGE", "accounts"),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    pub async fn get(&self) -> seed::fetch::Result<Data<Accounts>> {
        log!(self.url);
        let request = Request::new(&self.url).method(Method::Get);
        fetch(request).await?.json::<Data<Accounts>>().await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_object() {
        let data = r#"{
   "data": [
      {
         "access_token": "Dummy",
         "category": "Musician/Band",
         "category_list": [
            {
               "id": "ID",
               "name": "Musician/Band"
            }
         ],
         "name": "business_name",
         "id": "12345",
         "tasks": [
             
         ]
      }
   ]

}"#;

        let v: Data<Accounts> = serde_json::from_str(data).unwrap();

        println!("{:?}", v);
        assert_eq!(v.data.first().unwrap().name, "business_name".to_string());
    }
}
