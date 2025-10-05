//! The photo API of Facebook Page lets you publish photos and retrieve all photos from a page.
//! This API supports uploading photos by file (web-sys feature) and retrieving all photos by page_id.
//! Note: It does not allow getting a single photo, updating, or deleting. Use other methods for those operations.
//! For more information on different operations on Facebook pages check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos>.

use crate::graph::data::Data;
use crate::prelude::errors::ClientErr;
use crate::prelude::utils::GetPostResponse;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "web_sys", feature = "seed_async"))]
use web_sys::{Blob, File, FormData};

/// The photo API of Facebook Page lets you publish photos and retrieve all photos from a page.
/// This API supports uploading photos by file (web-sys feature) and retrieving all photos by page_id.
/// For more information check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos>.
#[derive(Clone)]
pub struct PhotoApi {
    base_url: String,
    page_access_token: String,
}

impl PhotoApi {
    /// Creates a new instance of PhotoApi.
    /// This is a static method used to create an instance of the PhotoApi.
    /// Note: This method is called inside the Client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the API endpoint
    /// * `page_access_token` - The page access token for authentication
    pub fn new(base_url: String, page_access_token: String) -> PhotoApi {
        PhotoApi {
            base_url,
            page_access_token,
        }
    }

    /// Posts a photo to the Facebook page using a file upload.
    /// This method is only available with the web-sys feature for browser environments.
    ///
    /// # Arguments
    /// * `file` - The File object containing the photo to upload
    /// * `photo_params` - Parameters for the photo upload (message, place, etc.)
    ///
    /// # Returns
    /// Returns a `PhotoResponse` containing the photo ID and post ID on success.
    ///
    /// For more information check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos#publish>.
    #[cfg(feature = "web-sys")]
    pub async fn post_by_file(
        &self,
        file: File,
        photo_params: PhotoParams,
    ) -> Result<PhotoResponse, ClientErr> {
        let form_data = self.clone().form_data(photo_params, file);
        let base_url = self.base_url.replace("EDGE", "photos");
        let url = base_url + "?access_token=" + &self.page_access_token;
        let resp = HttpConnection::video_post::<PhotoResponse>(url, form_data).await?;
        Ok(resp)
    }

    /// Retrieves all photos from the Facebook page.
    ///
    /// # Returns
    /// Returns a `Data<GetPostResponse>` containing the list of photos on success.
    ///
    /// For more information check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos#reading>.
    pub async fn all_photos(&self) -> Result<Data<GetPostResponse>, ClientErr> {
        let url =
            self.base_url.replace("EDGE", "photos") + "&access_token=" + &self.page_access_token;

        let resp = HttpConnection::get::<Data<GetPostResponse>>(url, "".to_string()).await?;

        Ok(resp)
    }

    /// Creates a FormData object for uploading a photo with parameters.
    /// This is a helper method for constructing the multipart form data.
    ///
    /// # Arguments
    /// * `photo_params` - Parameters for the photo upload
    /// * `file` - The File object to upload
    #[cfg(feature = "web-sys")]
    fn form_data(self, photo_params: PhotoParams, file: File) -> FormData {
        let form_data = FormData::new().unwrap();

        form_data.append_with_blob("source", &file).ok();
        form_data
            .append_with_str("message", &photo_params.message)
            .ok();
        form_data
    }
}

/// Possible parameters used when uploading a photo to a Facebook page.
/// These parameters allow you to customize the photo post with a message and location.
#[derive(Clone, Serialize)]
pub struct PhotoParams {
    /// The description of the photo, used as the accompanying status message in
    /// any feed story. The message can contain mentions of Facebook Pages
    /// using the following syntax: `@[page_id]`
    ///
    /// # Example
    /// ```text
    /// "Test message @[123456789]"
    /// ```
    pub message: String,

    /// Page ID of a place associated with the photo.
    /// The page must have location enabled.
    pub place: String,

    /// The file path of the photo to upload (for reference purposes).
    pub file_path: String,
}

/// Response returned after successfully posting a photo to a Facebook page.
#[derive(Clone, Deserialize)]
pub struct PhotoResponse {
    /// The ID of the photo object
    pub id: String,

    /// The ID of the post that was created with this photo
    pub post_id: String,
}
