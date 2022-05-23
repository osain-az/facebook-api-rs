//! Reqwest HTTP client
use crate::universal::errors::ClientErr;
use std::convert::TryInto;
use std::io::Read;
// use crate::universal::utils::generic_req;
use crate::universal::HttpClient;
use ::reqwest::Body;
#[cfg(feature = "reqwest_async")]
use ::reqwest::Client;
use async_trait::async_trait;
use http::header::HeaderMap;
use http::Method;

// use reqwest::multipart;
use crate::prelude::video::{ContentCategory, VideoParams};
use crate::universal::form_data::{create_form_data, form_data_with_bytes};
use reqwest::multipart::Form;
use reqwest::{multipart, Request, RequestBuilder};

use crate::prelude::utils::UploadingData;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub client: Client,
    pub headers: HeaderMap,
}

#[async_trait(?Send)]
impl HttpClient for ReqwestClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientErr> {
        let client = Client::builder();
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
        // No version on the response when using from client but works when using from
        // server (backend)
        let version = request.version().clone();
        let req = request.try_into().unwrap();

        let resp = self
            .client
            .execute(req)
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        // No version on the response when using from client but works when using from
        // server (backend) let version = resp.version();
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
        // et req: Request = request.try_into().unwrap();
        // let req: RequestBuilder = generic_req::<FormData>(request).unwrap();
        let url = request.uri().to_string();
        let method = request.method().clone();
        let body = request.body().to_owned();

        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            Method::PUT => Client::new().put(url),
            Method::DELETE => Client::new().delete(url),

            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };
        let resp = req
            .multipart(create_form_data(body, Vec::new()))
            .send()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        // No version on the response when using from client but works when using from
        // server (backend)
        let version = request.version();
        let status_code = resp.status();
        let headers = resp.headers().clone();
        // let version = resp.version();

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

    async fn resumable_video_request(
        &self,
        request: http::Request<UploadingData>,
    ) -> Result<http::Response<String>, ClientErr> {
        // et req: Request = request.try_into().unwrap();
        // let req: RequestBuilder = generic_req::<FormData>(request).unwrap();
        let url = request.uri().to_string();
        let method = request.method().clone();
        let body = request.body().to_owned();
        //  let test = Form::from(request.body().as_ref());
        // let ne_test: Form = request.into_body();

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

        // tsting

        let resp = if body.upload_phase == "start".to_string() {
            let response = req
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
            response
        } else if body.upload_phase == "transfer".to_string() {
            use futures_util::stream::StreamExt;
            use futures_util::{future, stream};
            use std::io::Cursor;
            // use tokio::fs::File;
            use bytes::{Buf, BufMut, Bytes, BytesMut};
            use std::fs::File;

            let mut file = File::open(body.file_path.clone()).unwrap();
            let mut buffer = [0; 1048576];
            //  let stream = FramedRead::new(file, BytesCodec::new());

            file.read_exact(&mut buffer).unwrap();
            // let stream_body = FramedRead::new(Cursor::new(buffer), BytesCodec::new());
            //  let stream =
            // reqwest::Body::wrap_stream(stream::once(future::ready(Ok::<_,
            // reqwest::Error>( "part1 part2".to_owned(),
            // ))));
            //   let stream = reqwest::Body::wrap_stream(stream_body);
            let bytes = Bytes::from(buffer.to_vec());

            // let mut reader = BufReader::new(&*test.chun).buffer();
            // let stream_part = Part::bytes(buffer.to_vec());
            let part = reqwest::multipart::Part::stream(bytes);

            let form = reqwest::multipart::Form::new().part("video_file_chunk", part);
            // println!("form data  :{:?}", form);
            let response = req
                .multipart(form)
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
            response
        } else if body.upload_phase == "finish".to_string() {
            let response = req
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
            response
        } else if body.upload_phase == "cancel".to_string() {
            // Todo: impliment this
            let response = req
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
            response
        } else {
            let response = req
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
            response
        };

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = request.version();
        // println!("raw resp.{:?}", resp);
        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        println!("facebook api {}", content.clone());

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

    async fn upload_by_form_data_request(
        &self,
        request: http::Request<(Vec<u8>, VideoParams)>,
    ) -> Result<http::Response<String>, ClientErr> {
        let url = request.uri().to_string();
        let method = request.method().clone();
        let (buffer, params) = request.body().clone();

        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            Method::PUT => Client::new().put(url),
            Method::DELETE => Client::new().delete(url),
            Method::PATCH => Client::new().patch(url),
            Method::HEAD => Client::new().head(url),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };

        let part = reqwest::multipart::Part::bytes(buffer);

        let form = reqwest::multipart::Form::new()
            .part("source", part)
            .text("description", params.clone().description)
            .text("description", params.clone().title);

        let resp = req
            //.form(&form)
            .multipart( form)
            .send()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = request.version();

        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        println!("facebook api {}", content.clone());

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
