//! Reqwest HTTP client
use std::convert::TryInto;

use crate::universal::errors::ClientErr;
//use crate::universal::utils::generic_req;
use crate::universal::HttpClient;
#[cfg(feature = "reqwest_async")]
use ::reqwest::Client;
use async_trait::async_trait;
use http::header::HeaderMap;
use http::Method;
//use reqwest::multipart;
use crate::prelude::video::VideoParams;
use crate::universal::form_data::create_form_data;
use reqwest::multipart::Form;
use reqwest::{Request, RequestBuilder};

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub client: Client,
    pub headers: HeaderMap,
}

#[async_trait(?Send)]
impl HttpClient for ReqwestClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientErr> {
        let client = Client::builder().gzip(true);
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };

        client
            .default_headers(headers.clone())
            .build()
            .map(|c| ReqwestClient { client: c, headers })
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }

    async fn request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientErr> {
        let req = request.try_into().unwrap();

        let resp = self
            .client
            .execute(req)
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = resp.version();
        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }

    async fn video_request(
        &self,
        request: http::Request<VideoParams>,
    ) -> Result<http::Response<String>, ClientErr> {
        //et req: Request = request.try_into().unwrap();
        // let req: RequestBuilder = generic_req::<FormData>(request).unwrap();
        let url = request.uri().to_string();
        let method = request.method().clone();
        let body = request.body().to_owned();

        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            Method::PUT => Client::new().put(url),
            Method::DELETE => Client::new().delete(url),
            Method::PATCH => Client::new().patch(url),
            // Method::CONNECT => Client::new().connect(url),
            Method::HEAD => Client::new().head(url),
            //   Method::OPTIONS => Client::new().option(url),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };
        let resp = req
            .multipart(create_form_data(body))
            .send()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = resp.version();
        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }
}
