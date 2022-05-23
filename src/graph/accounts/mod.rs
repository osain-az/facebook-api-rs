//! This mod represent strucs and API needed for getting different accounts
//! (facebook page) a user have access to. accounts API is called, it returns
//! the  Facebook Pages that a user owns or is able to perform tasks on.
//! This endpoint let you create facebook page or get different pages
//!
//! the response is the list of pages and the taskes, information and action the
//! user can perform on those pages as shown in the struct. the actual response
//! is varies with pages but this struct data will be present in every page For
//! more details check facebook official documentation   https://developers.facebook.com/docs/graph-api/reference/user/accounts/

use crate::graph::data::Data;
use crate::prelude::errors::ClientErr;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};

/// This struct represent some of the data that will be returned when the
/// account end point is called, the response is the list of pages and the
/// taskes, information and action the user can perform on those pages as shown
/// in the struct. the actual response is varies with pages but this struct data
/// will be present in every page For more details check facebook official
/// documentation   <https://developers.facebook.com/docs/graph-api/reference/user/accounts/>
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
pub struct Accounts {
    /// The access token of this given page, which used to make operation that
    /// requires permission on this page example post and get request.
    pub access_token: String,

    /// The category shows the name of the major category the pages belog to
    pub category: String,

    /// this is this list of categories  with their names and id  { name:"
    /// category_name", id: ""1223333
    pub category_list: Vec<ListDetails>,

    /// The facebook page name
    pub name: String,
    pub id: String,

    /// this is the list of operation/task the user can perform on this page
    pub tasks: Vec<String>,
}

/// This is the struct of name and id of category that page belong to

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct ListDetails {
    id: String,
    name: String,
}

impl Accounts {
    /// This will return the page access token
    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    pub fn category(&self) -> &String {
        &self.category
    }

    /// This will return the list of category the page  belong
    pub fn category_list(&self) -> &Vec<ListDetails> {
        &self.category_list
    }

    /// This will return the page name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// This will return the page id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// This will returned the list of permission and tasks the user is
    /// permitted to perform on the page
    pub fn tasks(&self) -> &Vec<String> {
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

    /// This request  will get the list of  Facebook Pages that a person owns or
    /// is able to perform tasks on.
    ///
    /// The response data can be seen in the Accounts struct,   the struct is
    /// constructed to only take data that will be returned on every page
    /// since the actual reponse from facebook varies with pages
    ///
    /// To check the possible data that is possible to be in the response which
    /// varies with pages check facebook documentation    
    /// <https://developers.facebook.com/docs/graph-api/reference/user/accounts/>

    pub async fn get(&self) -> Result<Data<Accounts>, ClientErr> {
        let resp =
            HttpConnection::get::<Data<Accounts>>(self.url.to_string(), "".to_string()).await?;
        Ok(resp)
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

        assert_eq!(v.data.first().unwrap().name, "business_name".to_string());
    }
}
