use std::collections::HashMap;

use reqwest::{blocking::Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub key: String,
    #[serde(rename = "IP")]
    pub ip: String,
    pub gm: String,
    pub map: String,
    /// not available in listbeta because of localization
    pub name: Option<String>,
    #[serde(rename = "v")]
    pub version: String,
    /// not available in list, but is in listbeta
    pub attach: Option<bool>,
    pub region: Option<i32>,
    pub time: i32,
    pub bots: Option<i32>,
    pub players: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum SecaResponse {
    #[serde(rename = "ok")]
    Ok { data: String },
    #[serde(rename = "NOT_FOUND")]
    NotFound {},
    #[serde(rename = "INVALID_STEAM")]
    InvalidSteam {},
}

#[derive(Debug, Serialize, Deserialize)]
struct SecaReponseOuter {
    /// String of JSON that contains SECA's response
    response: String,
    activation: String,
}

/// Generic SECA post request
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
struct SecaRequest<'a> {
    #[serde(rename = "apikey")]
    api_key: String,
    v: i32,

    #[serde(flatten)]
    extra: Option<HashMap<&'a str, Value>>,
}

impl<'a> SecaRequest<'a> {
    pub fn new<T>(api_key: String, v: i32, extra: T) -> Self
    where
        T: Into<Option<HashMap<&'a str, Value>>>,
    {
        Self {
            api_key,
            v,
            extra: extra.into(),
        }
    }
}

/// API Client object
pub struct Seca {
    /// Reqwest client
    client: Client,
    /// Seca base url
    base: Url,
    /// Seca API key
    /// As of writing, this is a static key hardcoded into the game.
    api_key: String,
}

impl<'a> Seca {
    /// Create a Seca object from url and api key
    ///
    /// # Errors
    /// This will return [`crate::Error::UrlParseError`] if parsing the url fails.
    pub fn new(url: &str, api_key: &str) -> Result<Self, crate::Error> {
        let client = Client::new();

        Ok(Self {
            client,
            base: Url::parse(url)?,
            api_key: api_key.to_string(),
        })
    }

    /// Create a Seca object for the official SECA instance¨
    ///
    /// # Errors
    /// This will return [`crate::Error::UrlParseError`] if parsing the url fails.
    pub fn from_official() -> Result<Self, crate::Error> {
        Self::new(
            "https://seca.sectorsedge.com/",
            "m7txaFEq5bcVppFWeYthtXGm43EeVdnX",
        )
    }

    /// Generic function for requesting an endpoint
    ///
    /// # Errors
    /// this can return either [`crate::Error::ReqwestError`] if an error occoured in reqwest,
    /// or [`crate::Error::SerdeJsonError`] if an error occurred when serializing/deserializing the response
    pub fn generic_request<T>(&self, endpoint: &str, custom: T) -> Result<String, crate::Error>
    where
        T: Into<Option<HashMap<&'a str, Value>>>,
    {
        let url = self.base.join(endpoint)?;
        let body = SecaRequest::new(self.api_key.clone(), 2, custom.into());
        let req = self
            .client
            .post(url)
            .header("Content-Type", "application/json; charset=utf-8")
            .body(serde_json::to_string(&body)?);

        let outer = req.send()?.json::<SecaReponseOuter>()?;
        match serde_json::from_str(&outer.response) {
            Ok(resp) => match resp {
                SecaResponse::Ok { data, .. } => Ok(data),
                SecaResponse::NotFound { .. } => Err(crate::Error::SecaNotFound()),
                SecaResponse::InvalidSteam { .. } => Err(crate::Error::SecaInvalidSteam()),
            },
            Err(e) => Err(crate::Error::SecaGenericError(e)),
        }
    }

    /// /match/list
    ///
    /// # Errors
    /// this can return either [`crate::Error::ReqwestError`] if an error occoured in reqwest,
    /// or [`crate::Error::SerdeJsonError`] if an error occurred when serializing/deserializing the response
    pub fn get_server_list(&self) -> Result<Vec<Server>, crate::Error> {
        let url = String::from("/match/list");
        let response = self.generic_request(&url, None)?;

        Ok(serde_json::from_str::<Vec<Server>>(&response)?)
    }

    /// /match/listbeta
    ///
    /// # Errors
    /// this can return either [`crate::Error::ReqwestError`] if an error occoured in reqwest,
    /// or [`crate::Error::SerdeJsonError`] if an error occurred when serializing/deserializing the response
    pub fn get_beta_server_list(&self) -> Result<Vec<Server>, crate::Error> {
        let url = String::from("/match/listbeta");
        let response = self.generic_request(&url, None)?;

        Ok(serde_json::from_str::<Vec<Server>>(&response)?)
    }

    /// /api/statRequest
    ///
    /// # Errors
    /// this can return either [`crate::Error::ReqwestError`] if an error occoured in reqwest,
    /// or [`crate::Error::SerdeJsonError`] if an error occurred when serializing/deserializing the response
    pub fn get_stats(&self, auth_ticket: String, beta: bool) -> Result<Stats, crate::Error> {
        let url = String::from("/api/statRequest");
        let response = self.generic_request(
            &url,
            HashMap::from([
                ("attach", json!(beta)),
                ("first", json!(true)),
                ("authTicket", json!(auth_ticket)),
            ]),
        )?;

        Ok(serde_json::from_str::<Stats>(&response)?)
    }
}
