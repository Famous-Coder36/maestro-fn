use crate::client::client;
use crate::response::Response;

use futures_util::StreamExt;
use reqwest::header::HeaderMap;
use std::time::Duration;
use url::Url;
use tokio::fs;
use reqwest::header::{HeaderName, HeaderValue};

pub struct GetBuilder {
    path: String,
    headers: HeaderMap,
    timeout: Option<Duration>,
    retries: usize,
    progress: Option<Box<dyn Fn(u64) + Send + Sync>>,
}

pub fn get(path: &str) -> GetBuilder {
    GetBuilder {
        path: path.to_string(),
        headers: HeaderMap::new(),
        timeout: None,
        retries: 0,
        progress: None,
    }
}

impl GetBuilder {

    pub fn header(mut self, key: &str, value: &str) -> Self {
    if let (Ok(name), Ok(val)) = (
        HeaderName::from_bytes(key.as_bytes()),
        HeaderValue::from_str(value),
    ) {
        self.headers.insert(name, val);
    }
    self
}

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Some(Duration::from_secs(secs));
        self
    }

    pub fn retry(mut self, n: usize) -> Self {
        self.retries = n;
        self
    }

    pub fn progress<F>(mut self, f: F) -> Self
    where
        F: Fn(u64) + Send + Sync + 'static,
    {
        self.progress = Some(Box::new(f));
        self
    }

    fn is_remote(&self) -> bool {
        Url::parse(&self.path).is_ok()
    }

    pub async fn send(self) -> Result<Response, Box<dyn std::error::Error>> {

    if !self.is_remote() {
        let data = fs::read(&self.path).await?;
        return Ok(Response { data });
    }

        let mut attempt = 0;

        loop {
            let mut req = client().get(&self.path);

            req = req.headers(self.headers.clone());

            if let Some(t) = self.timeout {
                req = req.timeout(t);
            }

            let res = req.send().await;

            match res {
                Ok(res) => {
                    let total = res.content_length().unwrap_or(0);
                    let mut stream = res.bytes_stream();

                    let mut downloaded = 0u64;
                    let mut data = Vec::new();

                    while let Some(chunk) = stream.next().await {
                        let chunk = chunk?;
                        downloaded += chunk.len() as u64;
                        data.extend_from_slice(&chunk);

                        if let Some(cb) = &self.progress {
                            if total > 0 {
                                cb(downloaded * 100 / total);
                            }
                        }
                    }

                    return Ok(Response { data });
                }

                Err(e) => {
                    if attempt >= self.retries {
                        return Err(Box::new(e));
                    }
                    attempt += 1;
                }
            }
        }
    }
}