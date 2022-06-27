use std::ops::Deref;

use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use serde_json::value::Value;

use crate::universal::errors::{ClientErr, FacebookAPiError};

///
pub(crate) fn deserialize_response<T>(text: &str) -> Result<T, ClientErr>
where
    T: DeserializeOwned,
{
    let response: Response<T> = serde_json::from_str(text)?;
    Ok(Into::<Result<T, FacebookAPiError>>::into(response)?)
}

pub(crate) fn client_error_deserialize_response(text: &str) -> Result<ClientErr, ClientErr> {
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
            Response::Ok(success) => {
                println!("the response :''");
                Ok(success)
            }
            Response::Err(err) => {
                println!("the  error response :'{err:#?}'");
                Err(err)
            }
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
            println!("This is the errr ................ :''");

            FacebookAPiError::deserialize(rest)
                .map(Response::Err)
                .map_err(de::Error::custom)
        } else {
            println!("This is the response ................ :''");

            T::deserialize(rest)
                .map(Response::Ok)
                .map_err(de::Error::custom)
        }
    }
}
// #[derive(Deserialize, Debug)]
// pub(crate) struct ClientResult<T> {
// #[serde(rename = "result")]
// result: T,
// }
//
// impl<T> ClientResult<T> {
// pub fn unwrap(self) -> T {
// self.result
// }
// }
//
// impl<T> Deref for ClientResult<T> {
// type Target = T;
//
// fn deref(&self) -> &Self::Target {
// &self.result
// }
// }
