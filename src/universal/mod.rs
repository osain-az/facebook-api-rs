//! HTTP client
//!
//! Feature gates are used to conditionally enable specific http ecosystem.
//! Currently reqwest( async) and .......
//! supported out of the box.
//!
//! But it's possible to incorporate custom ecosystem. See
//! `examples/custom_client.rs`.
use http::{HeaderMap, Request, Response};
use url::Url;

use crate::universal::errors::ClientErr;

#[cfg(all(feature = "reqwest_async"))]
/*compile_error!(
    r#"feature "reqwest_async" and "reqwest_blocking" cannot be set at the same time.
If what you want is "reqwest_blocking", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);*/

#[cfg(all(feature = "reqwest_async", feature = "seed_async"))]
compile_error!(
    r#"feature "reqwest_async" and "surf_async" cannot be set at the same time.
If what you want is "seed_async", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);

#[cfg(all(
feature = "reqwest_async",
feature = "seed_async"
))]
compile_error!(
    r#"only one of features "reqwest_async", "seed_async" and "..." can be"#
);

#[cfg(any(feature = "reqwest_async"))]
pub mod reqwest;

//mod client_request;

pub(crate) mod errors;
//mod web_sys;

#[cfg(any(feature = "seed_async"))]
pub mod seed_client;
pub mod response;

//#[derive(Deserialize, Debug)]
#[maybe_async::maybe_async]
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
        self.request(Request::get(url.to_string()).body(request_body.into()).unwrap())
            .await
    }
    #[inline]
    async fn post<T>(&self, url: Url, request_body: T) -> Result<Response<String>, ClientErr>
        where
            Self: Sized,
            T: Into<String> + Send,
    {
        self.request(Request::post(url.to_string()).body(request_body.into()).unwrap())
            .await
    }
    #[inline]
    async fn put<T>(&self, url: Url,  request_body: T) -> Result<Response<String>, ClientErr>
        where
            Self: Sized,
            T: Into<String> + Send,
    {
        self.request(Request::put(url.to_string()).body( request_body.into()).unwrap())
            .await
    }
    #[inline]
    async fn delete<T>(&self, url: Url,  request_body: T) -> Result<Response<String>, ClientErr>
        where
            Self: Sized,
            T: Into<String> + Send,
    {
        self.request(Request::delete(url.to_string()).body( request_body.into()).unwrap())
            .await
    }
    #[inline]
    async fn patch<T>(&self, url: Url,  request_body: T) -> Result<Response<String>, ClientErr>
        where
            Self: Sized,
            T: Into<String> + Send,
    {
        self.request(Request::patch(url.to_string()).body( request_body.into()).unwrap())
            .await
    }

    #[inline]
    async fn head<T>(&self, url: Url,  request_body: T) -> Result<Response<String>, ClientErr>
        where
            Self: Sized,
            T: Into<String> + Send,
    {
        self.request(Request::head(url.to_string()).body(request_body.into()).unwrap())
            .await
    }

    #[inline]
    async fn options<T>(&self, url: Url,  request_body: T) -> Result<Response<String>, ClientErr>
        where
            Self: Sized,
            T: Into<String> + Send,
    {
        self.request(Request::options(url.to_string()).body( request_body.into()).unwrap())
            .await
    }

    async fn request(&self, request: Request<String>) -> Result<Response<String>, ClientErr>
        where
            Self: Sized;
}
