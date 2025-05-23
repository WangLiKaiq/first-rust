use reqwest::Response;
use serde::Serialize;

use crate::config::http::HttpClientConfig;

use super::ClientBuilder;

pub type HttpClient = reqwest::Client;

pub trait HttpClientExt: ClientBuilder {
    fn post_request<T: Serialize + ?Sized + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> impl std::future::Future<Output = Result<Response, reqwest::Error>>;
    fn put_request<T: Serialize + ?Sized + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> impl std::future::Future<Output = Result<Response, reqwest::Error>>;
    fn delete_request(
        &self,
        url: &str,
    ) -> impl std::future::Future<Output = Result<Response, reqwest::Error>>;
    fn get_request(
        &self,
        url: &str,
    ) -> impl std::future::Future<Output = Result<Response, reqwest::Error>>;
}

impl ClientBuilder for HttpClient {
    fn build_from_config(config: &HttpClientConfig) -> Result<Self, anyhow::Error> {
        Ok(reqwest::Client::builder().timeout(config.timeout).build()?)
    }
}

impl HttpClientExt for HttpClient {
    async fn post_request<T: Serialize + ?Sized + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<Response, reqwest::Error> {
        self.post(url).json(body).send().await
    }

    async fn put_request<T: Serialize + ?Sized + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<Response, reqwest::Error> {
        self.put(url).json(body).send().await
    }

    async fn delete_request(&self, url: &str) -> Result<Response, reqwest::Error> {
        self.delete(url).send().await
    }

    async fn get_request(&self, url: &str) -> Result<Response, reqwest::Error> {
        self.get(url).send().await
    }
}
