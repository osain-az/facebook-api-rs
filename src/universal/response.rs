use http::StatusCode;
use std::ops::Deref;

use crate::prelude::BatchResponseBody;
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use serde_json::value::Value;

use crate::universal::errors::{ClientErr, FacebookAPiError};

// @todo Review the response methods and refactor  the response method to have just one method

pub fn deserialize_response_handler<T>(
    response: Result<http::Response<String>, ClientErr>,
) -> Result<T, ClientErr>
where
    T: DeserializeOwned,
{
    match response {
        Ok(result) => {
            match result.status() {
                StatusCode::OK => {
                    let result = deserialize_response::<T>(result.body())?;
                    Ok(result)
                }
                StatusCode::BAD_REQUEST => Err(client_error_deserialize_response(result.body())?),
                _ => {
                    //  let result = deserialize_response::<T>(result.body())?;
                    //@Todo:  handle the possible response error
                    Err(ClientErr::CustomError(format!(
                        "unhandled facebook response error.  response: {}",
                        result.body().to_string(),
                    )))
                }
            }
        }
        Err(err) => {
            //  let result = deserialize_response::<T>(result.body())?;
            Err(err)
        }
    }
}

pub fn deserialize_batch_handler<T>(
    response: Result<http::Response<String>, ClientErr>,
) -> Result<T, ClientErr>
where
    T: DeserializeOwned,
{
    match response {
        Ok(result) => {
            match result.status() {
                StatusCode::OK => {
                    let result = deserialize_response_batch::<T>(result.body())?;
                    Ok(result)
                }
                StatusCode::BAD_REQUEST => Err(client_error_deserialize_response(result.body())?),
                _ => {
                    //  let result = deserialize_response::<T>(result.body())?;
                    //@Todo:  handle the possible response error
                    Err(ClientErr::CustomError(format!(
                        "unhandled facebook response error.  response: {}",
                        result.body().to_string(),
                    )))
                }
            }
        }
        Err(err) => Err(err),
    }
}

fn deserialize_response<T>(text: &str) -> Result<T, ClientErr>
where
    T: DeserializeOwned,
{
    let response: Response<T> = serde_json::from_str(text)?;

    Ok(Into::<Result<T, FacebookAPiError>>::into(response)?)
}

fn deserialize_response_batch<T>(text: &str) -> Result<T, ClientErr>
where
    T: DeserializeOwned,
{
    let response: T = serde_json::from_str(text.clone())?;
    Ok(response)
}

fn client_error_deserialize_response(text: &str) -> Result<ClientErr, ClientErr> {
    let error: FacebookAPiError = serde_json::from_str(text)?;
    Err(ClientErr::Facebook(error))
}

#[derive(Debug)]
pub(crate) enum Response<T> {
    Ok(T),
    Err(FacebookAPiError),
}

impl<T> Into<Result<T, FacebookAPiError>> for Response<T> {
    fn into(self) -> Result<T, FacebookAPiError> {
        match self {
            Response::Ok(success) => Ok(success),
            Response::Err(err) => Err(err),
        }
    }
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;
        let error = map
            .get("error")
            .map_or_else(|| Ok(false), Deserialize::deserialize)
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if error {
            FacebookAPiError::deserialize(rest)
                .map(Response::Err)
                .map_err(de::Error::custom)
        } else {
            T::deserialize(rest)
                .map(Response::Ok)
                .map_err(de::Error::custom)
        }
    }
}
