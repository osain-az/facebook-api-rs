use crate::universal::HttpClient;
use async_trait::async_trait;
use std::borrow::Cow;

use http::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, StatusCode, Version,
};
use seed::log;
use seed::prelude::{Method as SeedMethod, Request};
use std::convert::TryInto;
use std::fmt::Debug;
use std::str::FromStr;

use super::*;

#[cfg(feature = "seed_async")]
use seed::fetch::{
    fetch, form_data, FetchError, FormData as SeedFormData, Header as SeedHeader,
    Headers as SeedHeaders, Request as SeedRequest, Response as SeedResponse,
};
use serde::Serialize;

use crate::universal::errors::ClientErr;

#[derive(Debug, Clone)]
pub struct SeedClient {
    pub headers: HeaderMap,
}
#[async_trait(?Send)]
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
    ) -> Result<http::Response<String>, ClientErr> {
        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let request_body = request.body().clone();

        let version = request.version().clone(); // Todo: i cant get the version from response, so for now we use version from the request
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
        let mut resp: SeedResponse;
        /*  let req_header = request.headers();
        for header in req_header {
            req.clone().header(SeedHeader::custom(
                Cow::from(header.0.to_string()),
                Cow::from(header.0.to_string()),
            ));
        }*/
        if Method::GET == request.method() {
            // if it is a GET method, it will panic if it found body
            resp = fetch(req.clone())
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;
        } else {
            let req_body = req.clone().text(request_body.clone()); // we might have issue with different data type of the body (jason,formdate)
            resp = fetch(req_body)
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;
        }
        println!("this is the status {}", "66666666666");

        let status_code = resp.status().code;
        let headers = resp.headers().clone();
        //  let version = resp.version(); //Todo: version in the response

        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;

        let mut build = http::Response::builder();
        for header in headers {
            build = build.header(header.name(), header.value());
        }
        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{}", e)))
    }
    async fn video_request(
        &self,
        request: http::Request<FormData>,
    ) -> Result<http::Response<String>, ClientErr> {
        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let request_body: FormData = request.body().clone();

        let version = request.version().clone(); // Todo: i cant get the version from response, so for now we use version from the request
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
        /*  let req_header = request.headers();
        for header in req_header {
            req.clone().header(SeedHeader::custom(
                Cow::from(header.0.to_string()),
                Cow::from(header.0.to_string()),
            ));
        }*/
        let test = SeedFormData::from(request_body);
        let req_body = req.clone().form_data(test); // we might have issue with different data type of the body (jason,formdate)

        let resp = fetch(req_body)
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;

        let status_code = resp.status().code;
        let headers = resp.headers().clone();
        //  let version = resp.version(); //Todo: version in the response

        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;

        let mut build = http::Response::builder();
        for header in headers {
            build = build.header(header.name(), header.value());
        }
        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{}", e)))
    }
    /*
    async fn resumable_video_request(
        &self,
        request: http::Request<UploadingData>,
    ) -> Result<http::Response<String>, ClientErr> {
        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let request_body = request.body().clone();

        let version = request.version().clone(); // Todo: i cant get the version from response, so for now we use version from the request
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
        /*  let req_header = request.headers();
        for header in req_header {
            req.clone().header(SeedHeader::custom(
                Cow::from(header.0.to_string()),
                Cow::from(header.0.to_string()),
            ));
        }*/
        //let test = SeedFormData::from(request_body);
        let req_body = req.body(&Default::default()); // we might have issue with different data type of the body (jason,formdate)

        let resp = fetch(req_body)
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;

        let status_code = resp.status().code;
        let headers = resp.headers().clone();
        //  let version = resp.version(); //Todo: version in the response

        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:#?}", e)))?;

        let mut build = http::Response::builder();
        for header in headers {
            build = build.header(header.name(), header.value());
        }
        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{}", e)))
    }*/
}
