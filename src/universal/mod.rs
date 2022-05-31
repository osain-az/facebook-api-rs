use crate::prelude::utils::UploadingData;
use crate::prelude::video::VideoParams;
use crate::universal::errors::ClientErr;

#[cfg(any(feature = "reqwst"))]
use ::reqwest::multipart::Form;

use async_trait::async_trait;
use http::{HeaderMap, Request, Response};

use url::Url;
#[cfg(any(feature = "web_sis"))]
use web_sys::FormData;

#[cfg(all(feature = "reqwst", feature = "web_sis"))]
compile_error!(
    r#"feature "reqwest_async" and "surf_async" cannot be set at the same time.
If what you want is "seed_async", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);

#[cfg(all(feature = "reqwst", feature = "web_sis"))]
compile_error!(r#"only one of features "reqwest_async", "seed_async" and "..." can be"#);

pub mod client;
pub mod errors;
#[cfg(any(feature = "reqwst"))]
pub mod form_data;
#[cfg(any(feature = "reqwst"))]
pub mod reqwest;
pub mod response;

#[cfg(any(feature = "web_sis"))]
pub mod web_sys_client;

#[async_trait(?Send)]
pub trait HttpClient: Sync + Clone {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientErr>
    where
        Self: Sized;

    #[inline]
    async fn get<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::get(url.to_string())
                .body(request_body.into())
                .unwrap(),
        )
        .await
    }

    #[inline]
    async fn post<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::post(url.to_string())
                .body(request_body.into())
                .unwrap(),
        )
        .await
    }

    #[cfg(any(feature = "web_sis"))]
    #[inline]
    async fn video_post(
        &self,
        url: Url,
        request_body: FormData,
    ) -> Result<Response<String>, ClientErr> {
        self.video_request(Request::post(url.to_string()).body(request_body).unwrap())
            .await
    }

    #[cfg(any(feature = "reqwst"))]
    #[inline]
    async fn video_post(
        &self,
        url: Url,
        request_body: VideoParams,
    ) -> Result<Response<String>, ClientErr> {
        self.video_request(Request::post(url.to_string()).body(request_body).unwrap())
            .await
    }

    #[cfg(any(feature = "reqwst"))]
    #[inline]
    async fn resumable_video_post(
        &self,
        url: Url,
        request_body: UploadingData,
    ) -> Result<Response<String>, ClientErr> {
        self.resumable_video_request(Request::post(url.to_string()).body(request_body).unwrap())
            .await
    }

    #[cfg(any(feature = "reqwst"))]
    #[inline]
    async fn upload_by_form_data(
        &self,
        url: Url,
        request_body: (Vec<u8>, VideoParams),
    ) -> Result<Response<String>, ClientErr> {
        self.upload_by_form_data_request(Request::post(url.to_string()).body(request_body).unwrap())
            .await
    }

    #[inline]
    async fn put<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::put(url.to_string())
                .body(request_body.into())
                .unwrap(),
        )
        .await
    }

    #[inline]
    async fn delete<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::delete(url.to_string())
                .body(request_body.into())
                .unwrap(),
        )
        .await
    }

    #[inline]
    async fn patch<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::patch(url.to_string())
                .body(request_body.into())
                .unwrap(),
        )
        .await
    }

    #[inline]
    async fn head<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::head(url.to_string())
                .body(request_body.into())
                .unwrap(),
        )
        .await
    }

    #[inline]
    async fn options<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::options(url.to_string())
                .body(request_body.into())
                .unwrap(),
        )
        .await
    }

    async fn request(&self, request: Request<String>) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "reqwst"))]
    async fn video_request(
        &self,
        //  request: Request<FormData>,
        request: Request<VideoParams>,
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "web_sis"))]
    async fn video_request(
        &self,
        request: Request<FormData>,
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "reqwst"))]
    async fn resumable_video_request(
        &self,
        //  request: Request<FormData>,
        request: Request<UploadingData>,
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "reqwst"))]
    async fn upload_by_form_data_request(
        &self,
        //  request: Request<FormData>,
        request: Request<(Vec<u8>, VideoParams)>,
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;
}
