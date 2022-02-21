use crate::graph::data::Data;
use crate::prelude::utils::ResponseStatus;
use crate::prelude::HttpConnection;
use crate::prelude::errors::ClientErr;

pub struct HashtagAPi{
    access_token: String,
    base_url: String,
}

impl HashtagAPi {
    pub fn new(access_token: String, base_url: String) -> HashtagAPi {
        HashtagAPi {
            access_token,
            base_url,
        }
    }

   pub async fn hashtag(self, hashtag_word:String) -> Result<HashtagIds, ClientErr>{

       let url  = self.base_url
       +"&q="+ &hashtag_word
       +"&access_token=" + &self.access_token;

       let resp = HttpConnection::get::<HashtagIds>(url, "".to_string()).await?;
       Ok(resp)

   }
}


#[derive(Deserialize, Debug, Serialize)]
pub struct HashtagIds{
    ids:Data<ResponseStatus>
}