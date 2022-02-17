use crate::universal::HttpClient;
use async_trait::async_trait;
use http::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, StatusCode, Version,
};

use web_sys::{
    FormData, Request as Web_sys_Request, RequestInit, RequestMode, Response, Window,
    XmlHttpRequest,
};

use super::*;

use crate::universal::errors::ClientErr;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Web_sysClient {
    pub headers: HeaderMap,
}
#[async_trait(?Send)]
impl HttpClient for Web_sysClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientErr> {
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };
        Ok(Web_sysClient { headers })
    }

    async fn request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientErr> {
        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let request_body = request.body().clone();

        let version = request.version().clone(); //
        let mut req_init = XmlHttpRequest::new().unwrap();

        let req = match method {
            Method::GET => req_init.open_with_async("GET", url.as_str(), false),
            Method::POST => req_init.open_with_async("POST", url.as_str(), false),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };

        req_init.send().unwrap();
        let status_code = req_init.status().unwrap();
        let headers = req_init.get_all_response_headers().unwrap();
        let content_opt = req_init.response_text().unwrap();

        let mut build = http::Response::builder();

        if let Some(content) = content_opt.clone() {
            build
                .status(status_code)
                .version(version)
                .body(content_opt.unwrap())
                .map_err(|e| ClientErr::HttpClient(format!("{}", e)))
        } else {
            Err(ClientErr::FacebookError("can`t unwrap result".to_string()))
        }
    }

    async fn video_request(
        &self,
        request: http::Request<FormData>,
    ) -> Result<http::Response<String>, ClientErr> {
        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let request_body = request.body().clone();

        let version = request.version().clone();

        let mut req_init = XmlHttpRequest::new().unwrap();
        let req = match method {
            Method::GET => req_init.open_with_async("GET", url.as_str(), false),
            Method::POST => req_init.open_with_async("POST", url.as_str(), false),
            Method::DELETE => req_init.open_with_async("DELETE", url.as_str(), false),

            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };

        req_init
            .send_with_opt_form_data(Some(&request_body))
            .unwrap();
        let status_code = req_init.status().unwrap();
        let headers = req_init.get_all_response_headers().unwrap();
        let content_opt = req_init.response_text().unwrap();

        let mut build = http::Response::builder();

        if let Some(content) = content_opt {
            build
                .status(status_code)
                .version(version)
                .body(content)
                .map_err(|e| ClientErr::HttpClient(format!("{}", e)))
        } else {
            Err(ClientErr::FacebookError("can`t unwrap result".to_string()))
        }
    }
}
