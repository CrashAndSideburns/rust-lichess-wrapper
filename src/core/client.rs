use hyper::{ Request, Method, Body, StatusCode };
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

use std::time::{Duration, Instant};

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

    /// Send a GET request to the supplied endpoint, then parse the response as
    /// JSON. Requests cannot be made synchonously, and will error if the
    /// server responds with a 429 status code, in which case new requests will
    /// all error for 60 seconds, or until the rate limit is lifted.
    async fn get<T>(&self, endpoint: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        if let Some(rate_limiter) = self.rate_limiter.load() {
            if rate_limiter >= Instant::now() {
                return Err(Error::RateLimited(rate_limiter));
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

        let resp = http_client.request(req).await.unwrap();

        match resp.status() {
            StatusCode::OK => {
                let resp_body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
                Ok(serde_json::from_slice(&resp_body).unwrap())
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let new_rate_limiter = Instant::now() + Duration::from_secs(60);
                self.rate_limiter.store(Some(new_rate_limiter));
                Err(Error::RateLimited(new_rate_limiter))
            }
            other => {
                panic!("Invalid response status code: {}", other);
            }
        }
    }
}

/// An error in client-server communication.
pub enum Error {
    RateLimited(Instant),
}
