//!  HTTP client from https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
use std::str::FromStr;

use http::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_LENGTH, SERVER},
    Method, StatusCode, Version,
};

use crate::universal::HttpClient;

use seed::fetch::{Headers as SeedHeaders, Header as SeedHeader, Request as SeedRequest, Method as SeedMethod, fetch};
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
        request: http::Request<T>,
    ) -> Result<http::Response<T>, ClientErr> {
        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let request_body = request.body();
        log::trace!("{:?}({:?}): {} ", method, url, request_body);

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

        let req = self.headers.iter().fold(req, |req, (k, v)| {
            req.header(
                SeedHeader::custom(k.as_str(),v.to_str())
            )
        });
        let req = request.headers().iter().fold(req, |req, (k, v)| {
            req.header(
                SeedHeader::custom(k.as_str(),v.to_str())
            )
        });

        let req_body = req
            .body(request_body);// we might have issue with different data type of the body (jason,formdate)
         let mut resp = fetch(req_body)
             .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let status = u16::from(status_code);
        let headers = resp.headers().clone();
        let version = resp.version();
        let content = resp
            .body()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        let mut resp =
            http::response::Builder::from(build).status(StatusCode::from_u16(status).unwrap());
        if version.is_some() {
            resp = resp.version(http_version.unwrap());
        }
        resp.body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }
}
