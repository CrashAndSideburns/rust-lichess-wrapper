use crate::core::ndjson::ndjson::NDJsonStream;

use hyper::{ Request, Method, Body, StatusCode };
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

use std::time::{Duration, Instant};
use std::error::Error;
use std::fmt::Display;

use crossbeam::atomic::AtomicCell;
use tokio::sync::Mutex;

use serde::de::DeserializeOwned;

/// A client connected to the Lichess API. This client is not authenticated.
pub struct Client {
    http_client: Mutex<hyper::Client<HttpsConnector<HttpConnector>>>,
    rate_limiter: AtomicCell<Option<Instant>>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            http_client: Mutex::new(
                hyper::Client::builder().build(HttpsConnector::new()),
            ),
            rate_limiter: AtomicCell::new(None),
        }
    }

    /// Send a GET request to the supplied endpoint, then return the response
    /// body. Requests cannot be made synchonously, and will error if the
    /// server responds with a 429 status code, in which case new requests will
    /// all error for 60 seconds, or until the rate limit is lifted.
    async fn get(&self, endpoint: &str) -> Result<Body, Box<dyn Error>> {
        if let Some(rate_limiter) = self.rate_limiter.load() {
            if rate_limiter >= Instant::now() {
                return Err(Box::new(ClientError::RateLimited(rate_limiter)));
            } else {
                self.rate_limiter.store(None);
            }
        }

        let req = Request::builder()
            .method(Method::GET)
            .uri(endpoint)
            .body(Body::empty())
            .unwrap();

        let http_client = self.http_client.lock().await;

        let resp = http_client.request(req).await?;

        match resp.status() {
            StatusCode::OK => {
                Ok(resp.into_body())
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let new_rate_limiter = Instant::now() + Duration::from_secs(60);
                self.rate_limiter.store(Some(new_rate_limiter));
                Err(Box::new(ClientError::RateLimited(new_rate_limiter)))
            }
            other => {
                panic!("Invalid response status code: {}", other);
            }
        }
    }

    /// Make a GET request to the API, and deserialize the response as a JSON
    /// object.
    async fn get_json<T>(&self, endpoint: &str) -> Result<T, Box<dyn Error>>
    where T: DeserializeOwned
    {
        let bytes = hyper::body::to_bytes(self.get(endpoint).await?).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Make a GET request to the API, and deserialize the response as an
    /// NDJSON stream.
    async fn get_ndjson<T>(&self, endpoint: &str) -> Result<NDJsonStream<T>, Box<dyn Error>> {
        Ok(NDJsonStream::new(self.get(endpoint).await?))
    }
}

/// An error in client-server communication.
#[derive(Debug)]
pub enum ClientError {
    RateLimited(Instant),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::RateLimited(rate_limit) => {
                write!(f, "request was denied due to rate limit in effect until {:?}", rate_limit)
            }
        }
    }
}

impl Error for ClientError {}
