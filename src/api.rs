use std::collections::HashMap;

use reqwest::{blocking::Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(from="String")]
pub enum ServerGamemode {
    Breakthrough,
    ControlShift,
    CaptureTheFlag,
    Escort,
    Unknown(String),
}

impl From<String> for ServerGamemode {
    fn from(input: String) -> Self {
        match input.as_str() {
            "brk" => ServerGamemode::Breakthrough,
            "cs" => ServerGamemode::ControlShift,
            "ctf" => ServerGamemode::CaptureTheFlag,
            "esc" => ServerGamemode::Escort,
            _ => ServerGamemode::Unknown(input),
        }
    }
}

/// Server Lobby information (might be incomplete!!!)
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerLobby {
    /// Allowed gamemodes
    pub gamemodes: Vec<ServerGamemode>,
    /// Allowed maps
    pub maps: Vec<String>,
    /// Max level allowed
    pub max_level: i32,
    /// Server region
    pub region: String,
    /// Are spectators allowed?
    pub spectators_allowed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub attach: bool,
    pub gm: ServerGamemode,
    #[serde(rename = "IP")]
    pub ip: String,
    pub key: String,
    pub map: String,
    pub name: String,
    pub number: i32,
    pub region: i32,
    pub time: i32,
    #[serde(rename = "v")]
    pub version: String,
    /// Players are not included in the data if there are none.
    pub players: Option<i32>,
    /// Bots are not included in the data if there are none.
    pub bots: Option<i32>,
    /// unsure of what this actually is... seems to be not included, 1, or 2.
    pub po: Option<i32>,
    /// Lobby information if the server has been hijacked for a private lobby.
    pub lobby: Option<ServerLobby>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
enum SecaResponse {
    #[serde(rename = "ok")]
    Ok { data: Value },
    #[serde(rename = "NOT_FOUND")]
    NotFound {},
    #[serde(rename = "INVALID_STEAM")]
    InvalidSteam {},
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
    pub fn generic_request<T>(&self, endpoint: &str, custom: T) -> Result<Value, crate::Error>
    where
        T: Into<Option<HashMap<&'a str, Value>>>,
    {
        let url = self.base.join(endpoint)?;
        let body = SecaRequest::new(self.api_key.clone(), 3, custom.into());
        let req = self
            .client
            .post(url)
            .header("Content-Type", "application/json; charset=utf-8")
            .body(serde_json::to_string(&body)?);

        match serde_json::from_str::<SecaResponse>(&req.send()?.text()?) {
            Ok(resp) => match resp {
                SecaResponse::Ok { data, .. } => Ok(data),
                SecaResponse::NotFound { .. } => Err(crate::Error::SecaNotFound()),
                SecaResponse::InvalidSteam { .. } => Err(crate::Error::SecaInvalidSteam()),
            },
            Err(e) => Err(crate::Error::SecaGenericError(e)),
        }
    }

    /// /match/listbeta
    ///
    /// # Errors
    /// this can return either [`crate::Error::ReqwestError`] if an error occoured in reqwest,
    /// or [`crate::Error::SerdeJsonError`] if an error occurred when serializing/deserializing the response
    pub fn get_server_list(&self) -> Result<Vec<Server>, crate::Error> {
        let url = String::from("/match/listbeta");
        let response = self.generic_request(&url, None)?;

        Ok(serde_json::from_value::<Vec<Server>>(response)?)
    }
}
