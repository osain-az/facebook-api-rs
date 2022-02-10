use crate::prelude::utils::UploadingData;
use crate::prelude::video::VideoParams;
use crate::universal::errors::ClientErr;

use async_trait::async_trait;
use http::{HeaderMap, Request, Response};
use url::Url;
#[cfg(any(feature = "seed_async"))]
use web_sys::FormData;

#[cfg(all(feature = "reqwest_async", feature = "seed_async"))]
compile_error!(
    r#"feature "reqwest_async" and "surf_async" cannot be set at the same time.
If what you want is "seed_async", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);

#[cfg(all(feature = "reqwest_async", feature = "seed_async"))]
compile_error!(r#"only one of features "reqwest_async", "seed_async" and "..." can be"#);

pub mod errors;
//pub mod form_data;
#[cfg(any(feature = "reqwest_async"))]
pub mod reqwest;
pub mod response;

pub mod client;
#[cfg(any(feature = "seed_async"))]
pub mod seed_client;
//mod testing;

//#[derive(Deserialize, Debug)]
//#[maybe_async::maybe_async]

//#[async_trait::async_trait]
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
    #[cfg(any(feature = "seed_async"))]
    #[inline]
    async fn video_post(
        &self,
        url: Url,
        //request_body: FormData,
        request_body: FormData,
    ) -> Result<Response<String>, ClientErr> {
        self.video_request(Request::post(url.to_string()).body(request_body).unwrap())
            .await
    }

    #[cfg(any(feature = "reqwest_async"))]
    #[inline]
    async fn video_post(
        &self,
        url: Url,
        //request_body: FormData,
        request_body: VideoParams,
    ) -> Result<Response<String>, ClientErr> {
        self.video_request(Request::post(url.to_string()).body(request_body).unwrap())
            .await
    }
    #[cfg(any(feature = "reqwest_async"))]
    #[inline]
    async fn resumable_video_post(
        &self,
        url: Url,
        //request_body: FormData,
        request_body: UploadingData,
    ) -> Result<Response<String>, ClientErr> {
        self.resumable_video_request(Request::post(url.to_string()).body(request_body).unwrap())
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

    #[cfg(any(feature = "reqwest_async"))]
    async fn video_request(
        &self,
        //  request: Request<FormData>,
        request: Request<VideoParams>,
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "seed_async"))]
    async fn video_request(
        &self,
        //  request: Request<FormData>,
        request: Request<FormData>,
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "reqwest_async"))]
    async fn resumable_video_request(
        &self,
        //  request: Request<FormData>,
        request: Request<UploadingData>,
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;
}
