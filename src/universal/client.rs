use serde::de::DeserializeOwned;

use std::sync::Arc;

use crate::prelude::video::VideoParams;
use crate::universal::errors::ClientErr;
#[cfg(any(feature = "reqwest"))]
use crate::universal::reqwest::ReqwestClient;
#[cfg(any(feature = "web-sys"))]
use crate::universal::web_sys_client::Web_sysClient;
use crate::universal::HttpClient;
use url::Url;

use crate::prelude::response::{deserialize_batch_handler, deserialize_response_handler};
use crate::prelude::utils::UploadingData;
#[cfg(any(feature = "web-sys"))]
use web_sys::FormData;

#[cfg(any(feature = "reqwest"))]
pub type HttpConnection = GenericClientConnection<ReqwestClient>;

#[cfg(any(feature = "web-sys"))]
pub type HttpConnection = GenericClientConnection<Web_sysClient>;

#[derive(Debug, Clone)]
pub struct GenericClientConnection<HttpC: HttpClient> {
    http_client: Arc<HttpC>,
    url: Url,
}

impl<HttpC: HttpClient> GenericClientConnection<HttpC> {
    pub async fn get<T>(build_url: String, body: String) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let response = client.get(build_url.parse().unwrap(), body).await;
        Ok(deserialize_response_handler::<T>(response)?)
    }

    pub async fn post<R, T>(build_url: String, body: T) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
        T: Into<String> + Send,
    {
        let client = HttpC::new(None)?;
        let response = client.post(build_url.parse().unwrap(), body).await;
        Ok(deserialize_response_handler::<R>(response)?)
    }

    pub async fn batch_post<R, T>(build_url: String, body: T) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
        T: Into<String> + Send,
    {
        let client = HttpC::new(None)?;
        let response = client.post(build_url.parse().unwrap(), body).await;
        Ok(deserialize_batch_handler::<R>(response)?)
    }

    pub async fn delete<T>(build_url: String, body: String) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let response = client.delete(build_url.parse().unwrap(), body).await;
        Ok(deserialize_response_handler::<T>(response)?)
    }

    #[cfg(any(feature = "reqwest"))]
    pub async fn video_post<T>(build_url: String, body: VideoParams) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, /* response Type
                              * T: Send + DeserializeOwned, */
    {
        let client = HttpC::new(None)?;
        let response = client.video_post(build_url.parse().unwrap(), body).await;
        Ok(deserialize_response_handler::<T>(response)?)
    }

    #[cfg(any(feature = "web-sys"))]
    pub async fn video_post<T>(build_url: String, body: FormData) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, /* response Type
                              * T: Send + DeserializeOwned, */
    {
        let client = HttpC::new(None)?;
        let response = client.video_post(build_url.parse().unwrap(), body).await;
        Ok(deserialize_response_handler::<T>(response)?)
    }

    #[cfg(any(feature = "reqwest"))]
    pub async fn resumable_video_post<T>(
        build_url: String,
        body: UploadingData,
    ) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, /* response Type
                              * T: Send + DeserializeOwned, */
    {
        let client = HttpC::new(None)?;
        let response = client
            .resumable_video_post(build_url.parse().unwrap(), body)
            .await;
        Ok(deserialize_response_handler::<T>(response)?)
    }

    // this will be used for rqwest_async feature
    #[cfg(any(feature = "reqwest"))]
    pub async fn request_by_bytes_and_params<T>(
        build_url: String,
        body: (Vec<u8>, VideoParams),
    ) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, /* response Type
                              * T: Send + DeserializeOwned, */
    {
        let client = HttpC::new(None)?;
        let response = client
            .upload_by_form_data(build_url.parse().unwrap(), body)
            .await;
        Ok(deserialize_response_handler::<T>(response)?)
    }

    /*  // this will be used for rqwest_async feature
    #[cfg(any(feature = "reqwest"))]
    pub async fn request_by_bytes<T>(build_url: String, body: Vec<u8>) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, /* response Type
                              * T: Send + DeserializeOwned, */
    {
        let client = HttpC::new(None)?;
        let response = client
            .video_post_by_bytes(build_url.parse().unwrap(), body)
            .await;
        if response.as_ref().is_ok() {
            println!("{:?}", response.as_ref().unwrap());
        } else {
            println!("{:?}", response.as_ref().err());
        }

        Ok(deserialize_response_handler::<T>(response)?)
    }*/
}
