//! The Facebook Video API allows you to publish Videos on Pages and Groups.
//! Publishing on Users is not supported.
//!
//! The process for publishing Videos involves choosing an upload protocol and
//! sending a POST request to the targeted Page or Group's  to end point /videos
//! edge. The API  supports  postting video by video_url, Resumable and
//! Non-Resumableupload protocols.
//! Facebook recommend that you use the Resumable Upload protocol
//! as it is more  versatile and can gracefully handle connection interruptions.
//! To post to either page or group pass either of the id.  (/event-id/videos,
//! /page-id/videos /group-id/videos
//!
//! For other information check on facebook documentation  for
//! For information on different opertaions on facebook page  check  <https://developers.facebook.com/docs/graph-api/reference/v13.0/page/photos>

use crate::prelude::errors::ClientErr;
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
use crate::prelude::utils::{form_data_seed, resumable_form_data_seed};
use crate::prelude::HttpConnection;
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
use web_sys::{Blob, File, FormData};

#[derive(Deserialize, Clone, Serialize)]
pub struct PhotoApi {
    base_url: String,
    page_access_token: String,
}

impl PhotoApi {
    pub fn new(base_url: String, page_access_token: String) -> PhotoApi {
        VideoApi {
            base_url,
            page_access_token,
        }
    }

    /// facebook recommend that you upload files using the Resumable Upload
    /// method because it handles connection interruptions more efficiently
    /// and supports larger files. However, if you prefer to upload files
    /// using the Non-Resumable Upload method.
    ///
    /// This method is expecting a video file less than 1 gb, and a video
    /// parameter struct,  if the video file is within this range it post
    /// the video but if the video is not within the range , the post will
    /// not be made but a Fetcherror will be gerated.

    #[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
    pub async fn upload_file(
        &self,
        file: File,
        photo_params: PhotoParams,
    ) -> Result<PhotoResponse, ClientErr> {
        let base_url = self.base_url.clone();
        let page_token = self.page_access_token.clone();
        let _file = file.clone();
        use crate::prelude::FileResult;
        let file_result = FileResult::file_analyze(file);

        // check if the uploading method
        if file_result.uploading_method() == "non_resumable" {
            let form_data = self.form_data(photo_params, _file);
            let base_url = self.base_url.replace("EDGE", "photos");
            let url = base_url + "?access_token=" + &self.page_access_token;
            let resp = HttpConnection::video_post::<PhotoResponse>(url, form_data).await?;
            Ok(resp)
        } else {
            Err(ClientErr::FacebookError(
                "The uplaoded file is above 1 gb, use Resumable method ".to_string(),
            )) // try to generate a customer
               // error
        }
    }
    pub async fn post_by_link(
        &self,
        file_url: &str,
        message: &str,
    ) -> Result<PhotoResponse, ClientErr> {
        let mut url = "".to_string();

        url = self.base_url.replace("EDGE", "photos")
            + "?url="
            + file_url
            + "&access_token="
            + &self.page_access_token
            + &"message="
            + description;

        let resp = HttpConnection::post::<PhotoResponse, String>(url, "".to_string()).await?;
        if resp.id.is_empty() {
            Err(ClientErr::FacebookError(
                "The video posting by url was not suceessfull ".to_string(),
            )) // try to generate a customer
        } else {
            Ok(resp)
        }
    }

    fn form_data(self, photo_params: PhotoParams, file: File) -> FormData {
        #[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
        let mut form_data = FormData::new().unwrap();

        form_data.append_with_blob("source", &file); // appped  the  current chunked file   to the form

        form_data.append_with_str("message", &photo_params.message);

        form_data
    }
}

/// Facebook video api accepts different parameters that could be passed to the
/// post request while uploading the video. this struck will have the possible
/// parameters that a user might need to pass along the video while publishing.
/// Note : video_title, file_name, and title will not appear in your feed. use
/// "description" to describe your video  which will appear at the top of the
/// post.
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
