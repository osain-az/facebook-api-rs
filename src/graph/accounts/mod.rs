//! Facebook accounts API gives access to facebook pages own by a user or
//! account a user have access to perform operations.
//!
//! This endpoint let you create facebook page or get different pages
//!
//! [facebook account doc] (https://developers.facebook.com/docs/graph-api/reference/user/accounts/)

use crate::prelude::errors::ClientErr;
use crate::prelude::search::Location;
use crate::prelude::structs::Engagement;
use crate::prelude::{HttpConnection, InstagramAccount};
use serde::{Deserialize, Serialize};

/// This struct represent the data that will be return from facebook page
///
/// For more information on page data check  [facebook account doc](https://developers.facebook.com/docs/graph-api/reference/page/)
#[derive(Deserialize, Debug, Clone, Default)]
pub struct PageAccount {
    /// The access token of this given page, which used to make operation that
    /// requires permission on this page example post and get request.
    access_token: String,
    /// The category shows the name of the major category the pages belog to
    category: String,
    /// this is this list of categories  with their names and id  { name:"
    /// category_name", id: ""1223333
    category_list: Vec<ListDetails>,
    /// The facebook page name
    name: String,
    /// Information about the Page. Can be read with Page Public Content Access
    /// or Page Public Metadata Access. This value maps to the Description
    /// setting in the Edit Page Info user interface. Limit of 100 characters
    #[serde(default)]
    about: String,
    /// The ID representing a Facebook Page.
    id: String,
    /// The Business associated with this Page. Requires business_management
    /// permissions, and a page or user access token. The person requesting
    /// the access token must be an admin of the page.
    business: String,
    can_post: bool,
    cover: CoverPage,
    emails: Vec<String>,
    engagement: Engagement,
    /// The number of users who like the Page. For Global Pages this is the
    /// count for all Pages across the brand. Can be read with Page Public
    /// Content Access or Page Public Metadata Access. For New Page
    /// Experience Pages, this field will return followers_count.
    fan_count: u32,
    /// Number of page followers
    followers_count: u32,
    /// Linked page backed instagram account for this page
    connected_page_backed_instagram_account: InstagramAccount,
    /// Instagram account linked to page during Instagram business conversion
    /// flow
    instagram_business_account: InstagramAccount,
    pub location: Location,
    phone: String,
    /// Number of ratings for the Page (limited to ratings that are publicly
    /// accessible). Can be read with Page Public Content Access or Page Public
    /// Metadata Access.
    rating_count: u32,
    /// this is the list of operation/task the user can perform on this page
    tasks: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Account {
    /// The access token of this given page, which used to make operation that
    /// requires permission on this page example post and get request.
    access_token: String,
    /// The category shows the name of the major category the pages belog to
    category: String,
    /// this is this list of categories  with their names and id  { name:"
    /// category_name", id: ""1223333
    category_list: Vec<ListDetails>,
    /// The facebook page name
    name: String,
    /// The ID representing a Facebook Page.
    id: String,
    /// The Business associated with this Page. Requires business_management
    /// permissions, and a page or user access token. The person requesting
    /// the access token must be an admin of the page.

    /// this is the list of operation/task the user can perform on this page
    tasks: Vec<String>,
}

/// This is the struct of name and id of category that page belongs to
#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct ListDetails {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Accounts {
    pub data: Vec<Account>,
}

impl Account {
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

impl PageAccount {
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

    pub fn about(&self) -> &str {
        &self.about
    }

    pub fn business(&self) -> &str {
        &self.business
    }

    pub fn can_post(&self) -> bool {
        self.can_post
    }

    pub fn cover(&self) -> &CoverPage {
        &self.cover
    }

    pub fn emails(&self) -> &Vec<String> {
        &self.emails
    }

    pub fn engagement(&self) -> &Engagement {
        &self.engagement
    }

    pub fn fan_count(&self) -> u32 {
        self.fan_count
    }

    pub fn followers_count(&self) -> u32 {
        self.followers_count
    }

    pub fn connected_page_backed_instagram_account(&self) -> &InstagramAccount {
        &self.connected_page_backed_instagram_account
    }

    pub fn instagram_business_account(&self) -> &InstagramAccount {
        &self.instagram_business_account
    }

    pub fn phone(&self) -> &str {
        &self.phone
    }

    pub fn rating_count(&self) -> u32 {
        self.rating_count
    }
}

/// Facebook accounts API gives access to facebook pages own by a user or
/// account a user have access to perform operations.
///
/// This endpoint let you create facebook page or get different pages
///
/// [facebook account doc] (https://developers.facebook.com/docs/graph-api/reference/user/accounts/)
///
/// # Example for getting pages
///
/// ```
/// use facebook_api_rs::prelude::{Accounts, Client, TokenLiveType, UserToken};
/// use facebook_api_rs::prelude::errors::ClientErr;
/// // The UserToken struct obtained from login
/// let token = UserToken::default();
///  // for long live to token.
///    let page_access_token_type = TokenLiveType::LONGLIVE;
///  let page :Result<Accounts, ClientErr>  = Client::new(token, "".to_owned()).accounts(page_access_token_type).get().await;
/// ```
pub struct AccountsAPI {
    url: String,
}

impl AccountsAPI {
    pub fn new(base_url: String) -> AccountsAPI {
        AccountsAPI {
            url: base_url.replace("EDGE", "accounts"),
        }
    }

    /// This request  will get the list of Facebook Pages that a person owns or
    /// have access to perform tasks
    ///
    /// [facebook account doc] (https://developers.facebook.com/docs/graph-api/reference/user/accounts/)
    ///
    /// # Example for getting pages
    ///
    /// ```
    /// use facebook_api_rs::prelude::{Accounts, Client, TokenLiveType, UserToken};
    /// use facebook_api_rs::prelude::errors::ClientErr;
    /// // The UserToken struct obtained from login
    /// let token = UserToken::default();
    ///  // for long live to token.
    ///    let page_access_token_type = TokenLiveType::LONGLIVE;
    ///  let page :Result<Accounts, ClientErr>  = Client::new(token, "".to_owned()).accounts(page_access_token_type).get().await;
    /// ```
    pub async fn get(&self) -> Result<Accounts, ClientErr> {
        let resp = HttpConnection::get::<Accounts>(self.url.to_string(), "".to_string()).await?;
        Ok(resp)
    }
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct CoverPage {
    id: String,
    source: String,
}

impl CoverPage {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn source(&self) -> &str {
        &self.source
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

        let v: Accounts = serde_json::from_str(data).unwrap();

        assert_eq!(v.data.first().unwrap().name, "business_name".to_string());
    }
}
