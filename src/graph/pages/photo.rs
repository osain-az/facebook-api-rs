

//! This methods lets you perform operation to facebook pages photos API.
//! It support  publishing and  get all photos in by page_id.
//! Note:: it does not allow  getting a single post, updating nor deleting. Use other method for that.
//! For information on different opertaions on facebook page  check  <https://developers.facebook.com/docs/graph-api/reference/v13.0/page/photos>

use crate::prelude::errors::ClientErr;
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
use crate::prelude::utils::{form_data_seed, resumable_form_data_seed};
use crate::prelude::HttpConnection;
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
use web_sys::{Blob, File, FormData};
use crate::graph::data::Data;
use crate::prelude::utils::GetPostResponse;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Clone, Serialize)]
pub struct PhotoApi {
    base_url: String,
    page_access_token: String,
}

impl PhotoApi {
    pub fn new(base_url: String, page_access_token: String) -> PhotoApi {
        PhotoApi {
            base_url,
            page_access_token,
        }
    }



    #[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
    pub async fn post_by_file(
        &self,
        file: File,
        photo_params: PhotoParams,
    ) -> Result<PhotoResponse, ClientErr> {
        let base_url = self.base_url.clone();
        let page_token = self.page_access_token.clone();
        let _file = file.clone();
        use crate::prelude::FileResult;
        //let file_result = FileResult::file_analyze(file);

        let form_data = self.form_data(photo_params, _file);
        let base_url = self.base_url.replace("EDGE", "photos");
        let url = base_url + "?access_token=" + &self.page_access_token;
        let resp = HttpConnection::video_post::<PhotoResponse>(url, form_data).await?;
        Ok(resp)

    }
    ///Posting a photo by url
    pub async fn all_photos(
        &self
    ) -> Result<Data<GetPostResponse>, ClientErr> {

       let url = self.base_url.replace("EDGE", "photos")
            + "&access_token="
            + &self.page_access_token;

        let resp = HttpConnection::get::<Data<GetPostResponse>>(url, "".to_string()).await?;

        Ok(resp)
    }

    #[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
    fn form_data(self, photo_params: PhotoParams, file: File) -> FormData {
        #[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
        let mut form_data = FormData::new().unwrap();

        form_data.append_with_blob("source", &file); // appped  the  current chunked file   to the form
        form_data.append_with_str("message", &photo_params.message);
        form_data
    }
}

/// Possible parameters used when uploading a photo
///
#[derive(Clone, Deserialize, Serialize)]
pub struct PhotoParams {
    /// The description of the photo, used as the accompanying status message in any feed story.
    /// The message can contain mentions of Facebook Pages using the following syntax:
    ///
    /// example
    ///
    /// Test message @[page_id]
    /// ```
    pub message: String,

    ///Page ID of a place associated with the Photo
    pub place: String,
    pub file_path: String,
}
pub struct PhotoResponse {
    pub id: String,
    post_id: String,
}
