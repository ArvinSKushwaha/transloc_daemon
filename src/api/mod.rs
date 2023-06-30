#![allow(dead_code)]

use reqwest::{Client, header::{HeaderMap, HeaderValue}};

use crate::API_URL;

use self::request::EndpointRequest;

pub(crate) mod response;
pub(crate) mod request;
pub(crate) mod helper_types;

pub(crate) struct TranslocApiManager {
    client: Client,
}

impl TranslocApiManager {
    pub fn new(api_key: &str) -> anyhow::Result<Self> {
        let mut request_header_map = HeaderMap::new();
        request_header_map.insert("X-RapidAPI-Key", HeaderValue::from_str(api_key.trim_end())?);
        request_header_map.insert("X-RapidAPI-Host", HeaderValue::from_static(API_URL));

        Ok(Self {
            client: Client::builder()
                .default_headers(request_header_map)
                .build()?,
        })
    }

    pub async fn request(&self, request: EndpointRequest) -> Result<reqwest::Response, reqwest::Error> {
        let mut query = vec![("format".to_string(), "json".to_string())];
        request.fill_query(&mut query);

        self.client.get(request.to_url()).query(&query).send().await
    }
}
