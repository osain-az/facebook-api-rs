//!  HTTP client from https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
use std::fmt::Debug;
use std::convert::TryInto;
use std::str::FromStr;
use seed::prelude::At::From;
use seed::prelude::{Request,Method as SeedMethod};
use http::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, StatusCode, Version,
};
use seed::{content, TryFutureExt};

use crate::universal::HttpClient;

use seed::fetch::{Headers as SeedHeaders, Header as SeedHeader, Request as SeedRequest, fetch, FetchError};

use crate::universal::errors::ClientErr;

#[derive(Debug, Clone)]
pub struct SeedClient {
    pub headers: HeaderMap,
}

#[async_trait::async_trait]
impl HttpClient for SeedClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientErr> {
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };
        Ok(SeedClient { headers })
    }

    async fn request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientErr>  {
        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let request_body = request.body();

        let version = request.version(); // Todo: i cant get the version from response, so for now we use version from the request
        let req = match method {

            Method::GET => SeedRequest::new(url).method(SeedMethod::Get),
            Method::POST => SeedRequest::new(url).method(SeedMethod::Post),
            Method::PUT => SeedRequest::new(url).method(SeedMethod::Put),
            Method::DELETE => SeedRequest::new(url).method(SeedMethod::Delete),
            Method::PATCH => SeedRequest::new(url).method(SeedMethod::Patch),
            Method::CONNECT => SeedRequest::new(url).method(SeedMethod::Connect),
            Method::HEAD => SeedRequest::new(url).method(SeedMethod::Head),
            Method::OPTIONS => SeedRequest::new(url).method(SeedMethod::Options),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };

        let req_body = req
            .text(request_body.clone());// we might have issue with different data type of the body (jason,formdate)
         let mut resp = fetch(req_body)
             .await
             .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;

        let status_code = resp.status().code;
        let headers = resp.headers().clone();
      //  let version = resp.version(); //Todo: version in the response
        let mut content : String = "".to_string();
            content   = resp.text().await?.map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;


        let mut build = http::Response::builder();
        for header in headers {
            build = build.header(header.name(), header.value());
        }
        build
            .status( status_code)
            .version(version)
            .body(content)
      .map_err(|e| ClientErr::HttpClient(format!("{}", e.to_string())))

    }
}