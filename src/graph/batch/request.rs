use crate::prelude::errors::{ClientErr, FacebookAPiError};
use crate::prelude::HttpConnection;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Debug;

pub struct BatchApi {
    pub base_url: String,
    pub page_access_token: String,
    pub page_id: String,
}

impl BatchApi {
    pub fn new(base_url: String, page_access_token: String, page_id: String) -> Self {
        BatchApi {
            base_url,
            page_access_token,
            page_id,
        }
    }

    pub async fn request(
        self,
        batch_items: BatchItems,
    ) -> Result<Vec<BatchResponseBody>, ClientErr> {
        let batch = serde_json::to_string(&batch_items.data).unwrap();

        let url = self.base_url
            + "?batch="
            + &batch
            + "&access_token="
            + &self.page_access_token
            + "&include_headers=false";

        let resp =
            HttpConnection::batch_post::<Vec<BatchResponseBody>, String>(url, "".to_string())
                .await?;
        Ok(resp)
    }

    pub async fn request_with_type<T>(
        self,
        batch_items: BatchItems,
    ) -> Result<Vec<BatchResponseBodyWithType<T>>, ClientErr>
    where
        T: DeserializeOwned,
    {
        let batch = serde_json::to_string(&batch_items.data).unwrap();

        let url = self.base_url
            + "?batch="
            + &batch
            + "&access_token="
            + &self.page_access_token
            + "&include_headers=false";

        let resp: Vec<BatchResponseBody> =
            HttpConnection::batch_post::<Vec<BatchResponseBody>, String>(url, "".to_string())
                .await?;

        let mut resp_body: Vec<BatchResponseBodyWithType<T>> = Vec::new();

        for body in resp {
            if let Some(data) = body.body {
                if body.code == 200 {
                    let data: T = serde_json::from_str(&data.as_str())?;

                    resp_body.push(BatchResponseBodyWithType {
                        code: body.code,
                        body: Some(data),
                        error: None,
                    })
                } else {
                    let data: FacebookAPiError = serde_json::from_str(&data.as_str())?;

                    resp_body.push(BatchResponseBodyWithType {
                        code: body.code,
                        body: None,
                        error: Some(data),
                    })
                }
            } else {
                resp_body.push(BatchResponseBodyWithType {
                    code: body.code,
                    body: None,
                    error: None,
                })
            }
        }
        Ok(resp_body)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BatchItems {
    pub data: Vec<Value>,
}

impl BatchItems {
    pub fn add_batch_item_with_body(
        mut self,
        method: &str,
        body: String,
        relative_url: &str,
    ) -> Self {
        let data = json!(
            {
                "method": method.to_uppercase(),
               "relative_url":relative_url,
                "body": body
            }
        );

        self.data.push(data);
        self
    }

    pub fn add_batch_item_with_body_access_token(
        mut self,
        method: &str,
        body: String,
        relative_url: &str,
        access_token: String,
    ) -> Self {
        let data = json!(
            {
                "method": method.to_uppercase(),
               "relative_url":relative_url.to_owned() + "?access_token=" + &access_token,
                "body": body
            }
        );
        self.data.push(data);
        self
    }

    pub fn add_batch_item_without_body(mut self, method: &str, relative_url: &str) -> Self {
        let data = json!(
            {
                "method": method.to_uppercase(),
               "relative_url":relative_url,
            }
        );
        self.data.push(data);
        self
    }

    pub fn add_batch_item_with_access_token(
        mut self,
        method: &str,
        relative_url: &str,
        access_token: String,
    ) -> Self {
        let data = json!(
            {
                "method": method.to_uppercase(),
               "relative_url":relative_url.to_owned() + "?access_token=" + &access_token,
            }
        );
        self.data.push(data);
        self
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BatchResponseBody {
    pub code: u64,
    pub body: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BatchResponseBodyWithType<T> {
    pub code: u64,
    pub body: Option<T>,
    pub error: Option<FacebookAPiError>,
}
