use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize, Serialize,
};
use std::borrow::BorrowMut;
use std::convert::TryInto;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::universal::errors::ClientErr;
//use crate::universal::reqwest::ReqwestClient;
#[cfg(any(feature = "reqwest_async"))]
use crate::universal::reqwest::ReqwestClient;
use crate::universal::response::{deserialize_response, ClientResult};
#[cfg(any(feature = "seed_async"))]
use crate::universal::seed_client::SeedClient;
#[cfg(any(feature = "web_sys_async"))]
use crate::universal::web_sys_client::Web_sysClient;

use crate::universal::HttpClient;
use async_trait::async_trait;
use log::{debug, trace};

#[cfg(any(feature = "reqwest_async"))]
use reqwest::multipart::Form;
//use reqwest::multipart::Form;
//#[async_trait(?Send)]
use crate::graph::me::Me;
use crate::prelude::account::{InstaAccountIds, InstagramAccount};
use crate::prelude::video::VideoParams;
use crate::prelude::{Accounts, Data};
use serde_json::Value;
use url::Url;

use crate::prelude::utils::UploadingData;
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
use web_sys::FormData;

#[cfg(any(feature = "reqwest_async"))]
pub type HttpConnection = GenericClientConnection<ReqwestClient>;
#[cfg(any(feature = "seed_async"))]
pub type HttpConnection = GenericClientConnection<SeedClient>;

#[cfg(any(feature = "web_sys_async"))]
pub type HttpConnection = GenericClientConnection<Web_sysClient>;

#[derive(Debug, Clone)]
pub struct GenericClientConnection<HttpC: HttpClient> {
    http_client: Arc<HttpC>,
    url: Url,
}
impl<HttpC: HttpClient> GenericClientConnection<HttpC> {
    pub async fn login(build_url: String, body: String) -> Result<(), ClientErr> {
        //  let client  = HttpClient::new(None)?
        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), body).await?;
        let result: ClientResult<()> = deserialize_response(resp.body())?;
        Ok(result.unwrap())
    }

    pub async fn get<T>(build_url: String, body: String) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<T>(resp.body())?;
        Ok(result)
    }

    pub async fn post<R, T>(build_url: String, body: T) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
        T: Into<String> + Send,
    {
        let client = HttpC::new(None)?;
        let resp = client.post(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }

    #[cfg(any(feature = "reqwest_async"))]
    pub async fn video_post<R>(build_url: String, body: VideoParams) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
                             //T: Send + DeserializeOwned,
    {
        let client = HttpC::new(None)?;
        let resp = client.video_post(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }

    #[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
    pub async fn video_post<R>(build_url: String, body: FormData) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
                             //T: Send + DeserializeOwned,
    {
        let client = HttpC::new(None)?;
        let resp = client.video_post(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }
    // this will be used for rqwest_async feature
    #[cfg(any(feature = "reqwest_async"))]
    pub async fn resumable_video_post<R>(
        build_url: String,
        body: UploadingData,
    ) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
                             //T: Send + DeserializeOwned,
    {
        let client = HttpC::new(None)?;
        let resp = client
            .resumable_video_post(build_url.parse().unwrap(), body)
            .await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }
}
