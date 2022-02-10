//! Reqwest HTTP client
use std::convert::TryInto;
use std::io::Read;

use crate::universal::errors::ClientErr;
//use crate::universal::utils::generic_req;
use crate::universal::HttpClient;
use ::reqwest::Body;
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

use crate::prelude::form_data::extract_bytes;
use crate::prelude::utils::UploadingData;

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
        println!("thi sis the eeror from  facebook api {}", content.clone());

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
        //et req: Request = request.try_into().unwrap();
        // let req: RequestBuilder = generic_req::<FormData>(request).unwrap();
        let url = request.uri().to_string();
        let method = request.method().clone();
        let body = request.body().to_owned();
        //  let test = Form::from(request.body().as_ref());
        // let ne_test: Form = request.into_body();
        println!("data {:?}", body);

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

        //tsting

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
            //use tokio::fs::File;
            use bytes::{Buf, BufMut, Bytes, BytesMut};
            use std::fs::File;
            use tokio_util::codec::{BytesCodec, FramedRead};

            let mut file = File::open(body.file_path.clone()).unwrap();
            let mut buffer = [0; 1048576];
            //  let stream = FramedRead::new(file, BytesCodec::new());

            file.read_exact(&mut buffer).unwrap();
            //let stream_body = FramedRead::new(Cursor::new(buffer), BytesCodec::new());
            /*  let stream =
            reqwest::Body::wrap_stream(stream::once(future::ready(Ok::<_, reqwest::Error>(
                  "part1 part2".to_owned(),
              ))));*/
            //   let stream = reqwest::Body::wrap_stream(stream_body);
            let bytes = Bytes::from(buffer.to_vec());
            println!("workign upto here after ");

            //let mut reader = BufReader::new(&*test.chun).buffer();
            // let stream_part = Part::bytes(buffer.to_vec());
            let part = reqwest::multipart::Part::stream(bytes);

            let form = reqwest::multipart::Form::new().part("video_file_chunk", part);
            //println!("form data  :{:?}", form);
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
        /*
        let form_data = Form::new()
            .text("file_size", "7962162")
            .text("upload_phase", "start");
        let testing = request.body();
        let resp = req
           //.form( &params)
            .multipart(form_data)
            .send()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;*/

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = resp.version();
        //println!("raw resp.{:?}", resp);
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
