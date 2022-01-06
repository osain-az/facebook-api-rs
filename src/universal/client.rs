use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::universal::errors::ClientErr;
//use crate::universal::reqwest::ReqwestClient;
use crate::universal::response::{deserialize_response, ClientResult};
use crate::universal::seed_client::SeedClient;
use crate::universal::HttpClient;
use async_trait::async_trait;
use log::{debug, trace};
//#[async_trait(?Send)]
use crate::graph::me::Me;
use crate::prelude::account::{InstaAccountIds, InstagramAccount};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

//#[cfg((feature = "reqwest_async"))]
//pub type HttpConnection = GenericClientConnection<ReqwestClient>;

#[cfg(feature = "seed_async")]
pub type HttpConnection = GenericClientConnection<SeedClient>;

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

    pub async fn user_details(build_url: String) -> Result<Me, ClientErr> {
        //  let client  = HttpClient::new(None)?
        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), "").await?;
        let result: ClientResult<Me> = deserialize_response(resp.body())?;
        Ok(result.unwrap())
    }

    //for in instgram
    pub async fn instagram_account_details(
        build_url: String,
    ) -> Result<InstagramAccount, ClientErr> {
        //  let client  = HttpClient::new(None)?
        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), "").await?;
        let result: ClientResult<InstagramAccount> = deserialize_response(resp.body())?;
        Ok(result.unwrap())
    }

    pub async fn instagram_account_id(build_url: String) -> Result<InstaAccountIds, ClientErr> {
        //  let client  = HttpClient::new(None)?
        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), "").await?;
        let result: ClientResult<InstaAccountIds> = deserialize_response(resp.body())?;
        Ok(result.unwrap())
    }
    pub async fn testing(build_url: String, body: String) -> Result<(), ClientErr> {
        //  let client  = HttpClient::new(None)?

        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), body).await?;
        let result: ClientResult<()> = deserialize_response(resp.body())?;
        Ok(result.unwrap())
    }
}
