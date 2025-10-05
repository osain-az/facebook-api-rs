//! The photo API of Facebook Page lets you publish photos and retrieve profile pictures and uploaded photos.
//!
//! # Features
//! - Upload photos by file (web-sys feature) or URL (reqwest feature)
//! - Retrieve all photos by page_id with optional type filter
//! - Support for published and unpublished photos
//! - Temporary photos for scheduled posts
//!
//! # Requirements
//! ## Reading
//! - Page access token with `MODERATE` task permission
//! - `pages_read_engagement` permission
//! - `pages_show_list` permission
//!
//! ## Creating
//! - Page access token with `CREATE_CONTENT` task permission
//! - `pages_read_engagement` permission
//! - `pages_manage_posts` permission
//! - `pages_show_list` permission
//!
//! # Note
//! Updating and deleting are not supported by this endpoint.
//!
//! For more information check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos>.

use crate::graph::data::Data;
use crate::prelude::errors::ClientErr;
use crate::prelude::utils::GetPostResponse;
use crate::prelude::HttpConnection;
use serde::{Deserialize, Serialize};
#[cfg(feature = "web-sys")]
use web_sys::{File, FormData};

/// The photo API of Facebook Page for managing profile pictures and uploaded photos.
///
/// By default, reading from the photos edge returns the current profile picture
/// and previous profile pictures. Use the `type=uploaded` parameter to get photos
/// that a Page has uploaded.
///
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

    /// Posts a photo to the Facebook page using a file upload (multipart/form-data).
    /// This method is only available with the web-sys feature for browser environments.
    ///
    /// The photo is uploaded as multipart/form-data with the parameter name `source`.
    /// Facebook strips all location metadata before publishing and resizes images
    /// to different dimensions for multiple display sizes.
    ///
    /// # Arguments
    /// * `file` - The File object containing the photo to upload
    /// * `photo_params` - Parameters for the photo upload (message, published status, etc.)
    ///
    /// # Returns
    /// Returns a `PhotoResponse` containing the photo ID and post ID on success.
    ///
    /// # Photo Specifications
    /// Facebook automatically processes photos according to these specifications:
    /// - Strips all location metadata before publishing
    /// - Resizes images to different dimensions for optimal rendering
    ///
    /// For more information check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos#creating>.
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

    /// Posts a photo to the Facebook page using a URL.
    /// This method uploads a photo that is already on the internet.
    ///
    /// # Arguments
    /// * `photo_params` - Parameters including the URL of the photo to upload
    ///
    /// # Returns
    /// Returns a `PhotoResponse` containing the photo ID and post ID on success.
    ///
    /// For more information check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos#creating>.
    pub async fn post_by_url(&self, photo_params: PhotoParams) -> Result<PhotoResponse, ClientErr> {
        let base_url = self.base_url.replace("EDGE", "photos");
        let mut url = base_url + "?access_token=" + &self.page_access_token;

        if !photo_params.url.is_empty() {
            url = url + "&url=" + &photo_params.url;
        }

        if !photo_params.message.is_empty() {
            url = url + "&message=" + &photo_params.message;
        }

        if !photo_params.published {
            url = url + "&published=false";
        }

        if photo_params.temporary {
            url = url + "&temporary=true";
        }

        if !photo_params.place.is_empty() {
            url = url + "&place=" + &photo_params.place;
        }

        let resp = HttpConnection::get::<PhotoResponse>(url, "".to_string()).await?;
        Ok(resp)
    }

    /// Retrieves photos from the Facebook page.
    ///
    /// By default, returns the current profile picture and previous profile pictures.
    /// Use `photo_type` parameter with `PhotoType::Uploaded` to get only uploaded photos.
    ///
    /// # Arguments
    /// * `photo_type` - Optional filter for photo type (Profile or Uploaded)
    ///
    /// # Returns
    /// Returns a `Data<GetPostResponse>` containing the list of Photo nodes.
    ///
    /// For more information check <https://developers.facebook.com/docs/graph-api/reference/v23.0/page/photos#reading>.
    pub async fn all_photos(
        &self,
        photo_type: Option<PhotoType>,
    ) -> Result<Data<GetPostResponse>, ClientErr> {
        let mut url = self.base_url.replace("EDGE", "photos");

        if let Some(ptype) = photo_type {
            url = url + "?type=" + ptype.as_str();
            url = url + "&access_token=" + &self.page_access_token;
        } else {
            url = url + "?access_token=" + &self.page_access_token;
        }

        let resp = HttpConnection::get::<Data<GetPostResponse>>(url, "".to_string()).await?;

        Ok(resp)
    }

    /// Creates a FormData object for uploading a photo with parameters.
    /// This is a helper method for constructing the multipart form data.
    /// The parameter name `source` is used historically for photo uploads.
    ///
    /// # Arguments
    /// * `photo_params` - Parameters for the photo upload
    /// * `file` - The File object to upload
    #[cfg(feature = "web-sys")]
    fn form_data(self, photo_params: PhotoParams, file: File) -> FormData {
        let form_data = FormData::new().unwrap();

        // The name "source" is used historically for photo uploads
        form_data.append_with_blob("source", &file).ok();

        if !photo_params.message.is_empty() {
            form_data
                .append_with_str("message", &photo_params.message)
                .ok();
        }

        if !photo_params.published {
            form_data.append_with_str("published", "false").ok();
        }

        if photo_params.temporary {
            form_data.append_with_str("temporary", "true").ok();
        }

        if !photo_params.place.is_empty() {
            form_data.append_with_str("place", &photo_params.place).ok();
        }

        form_data
    }
}

/// Type of photos to retrieve from a Facebook Page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotoType {
    /// Returns profile pictures (current and previous)
    Profile,
    /// Returns only photos that the page has uploaded
    Uploaded,
}

impl PhotoType {
    fn as_str(&self) -> &str {
        match self {
            PhotoType::Profile => "profile",
            PhotoType::Uploaded => "uploaded",
        }
    }
}

/// Parameters for uploading a photo to a Facebook page.
///
/// These parameters allow you to customize the photo post with various options
/// including message, location, publication status, and more.
#[derive(Clone, Serialize, Default)]
pub struct PhotoParams {
    /// The URL of a photo that is already on the internet.
    /// Use this when uploading by URL instead of file upload.
    ///
    /// Example: `"https://example.com/image.png"`
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,

    /// The description of the photo, used as the accompanying status message in
    /// any feed story. The message can contain mentions of Facebook Pages
    /// using the following syntax: `@[page_id]`
    ///
    /// # Example
    /// ```text
    /// "Check out our new product! @[123456789]"
    /// ```
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// Page ID of a place associated with the photo.
    /// The page must have location enabled.
    ///
    /// Example: `"110843418940484"` (a place page ID)
    #[serde(skip_serializing_if = "String::is_empty")]
    pub place: String,

    /// Whether the photo should be published immediately.
    ///
    /// - `true` (default): Publish the photo immediately
    /// - `false`: Upload as unpublished (stored for ~24 hours)
    ///
    /// Unpublished photos can be used in multi-photo posts or scheduled posts.
    /// Photos not published within 24 hours will be automatically deleted.
    #[serde(skip_serializing_if = "is_true")]
    pub published: bool,

    /// Whether the photo is temporary for use in scheduled posts.
    ///
    /// Must be `true` if the photo will be used in a scheduled post.
    /// Only valid when `published=false`.
    #[serde(skip_serializing_if = "is_false")]
    pub temporary: bool,

    /// The local file path of the photo (for reference purposes only, not sent to API).
    #[serde(skip)]
    pub file_path: String,
}

impl PhotoParams {
    /// Creates a new PhotoParams with default values (published=true, temporary=false).
    pub fn new() -> Self {
        PhotoParams {
            url: String::new(),
            message: String::new(),
            place: String::new(),
            published: true,
            temporary: false,
            file_path: String::new(),
        }
    }

    /// Sets the URL of the photo to upload.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Sets the message/caption for the photo.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Sets the place ID associated with the photo.
    pub fn place(mut self, place: impl Into<String>) -> Self {
        self.place = place.into();
        self
    }

    /// Sets whether the photo should be published immediately.
    pub fn published(mut self, published: bool) -> Self {
        self.published = published;
        self
    }

    /// Sets whether the photo is temporary for scheduled posts.
    pub fn temporary(mut self, temporary: bool) -> Self {
        self.temporary = temporary;
        self
    }

    /// Sets the local file path (for reference).
    pub fn file_path(mut self, file_path: impl Into<String>) -> Self {
        self.file_path = file_path.into();
        self
    }
}

// Helper functions for serde skip_serializing_if
fn is_true(b: &bool) -> bool {
    *b
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// Response returned after successfully posting a photo to a Facebook page.
#[derive(Clone, Deserialize)]
pub struct PhotoResponse {
    /// The ID of the photo object
    pub id: String,

    /// The ID of the post that was created with this photo
    pub post_id: String,
}
