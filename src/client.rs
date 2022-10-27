//! Auth0 request client.
use reqwest::{Client, Method, RequestBuilder, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::rate::{RateLimit, RateLimitResponse};
use crate::token::TokenManager;
use crate::Auth0Error;
use crate::{Auth0ErrorResponse, Auth0Result};

/// Auth0 management client.
#[derive(Debug)]
pub struct Auth0Client {
  rate: RateLimit,
  token: TokenManager,
  client: Client,
  domain: String,
}

// Object for auth0 response for when only a string is returned
#[derive(Serialize, Deserialize)]
pub struct Auth0MessageResponse { pub message: String}

impl Auth0Client {
  /// Create Auth0 client
  pub fn new(rate: RateLimit, token: TokenManager, client: Client, domain: &str) -> Self {
    Self {
      rate,
      token,
      client,
      domain: domain.to_owned(),
    }
  }

  /// Send request with auth0 client.
  pub async fn send<R>(&self, req: RequestBuilder) -> Auth0Result<R>
  where
    R: DeserializeOwned,
  {
    let token = self.token.get_token().await?;
    let res = req //
      .bearer_auth(&token)
      .send()
      .await?;

    if res.status().is_success() {
      let headers = res.headers().to_owned();
      let res_is_json = headers.contains_key(CONTENT_TYPE) && headers[CONTENT_TYPE] == "application/json; charset=utf-8";
      let body = res.rate_limit(&self.rate)?.bytes().await?;
      let body = body.to_vec();
      let body = std::str::from_utf8(&body).unwrap();

      let body: &str = if body.is_empty() { "null" } else if res_is_json { body } else { &serde_json::to_string(body)? };

      Ok(serde_json::from_str::<R>(body)?)
    } else {
      let body = res.bytes().await?;
      let body = body.to_vec();
      let body = std::str::from_utf8(&body).unwrap();

      let err = serde_json::from_str::<Auth0ErrorResponse>(body);
      if let Ok(err) = err {
        Err(Auth0Error::from(err))
      } else {
        Err(Auth0Error::Auth0(body.to_owned()))
      }
    }
  }

  /// Create auth0 request builder.
  /// # Arguments
  /// * `method` = The HTTP request method.
  /// * `path` - The HTTP request path.
  pub fn begin(&self, method: Method, path: &str) -> RequestBuilder {
    self
      .client
      .request(method, &format!("https://{}/{}", self.domain, path))
  }
}
