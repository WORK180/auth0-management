//! Retrieve details of users.
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{Auth0Client, Auth0Result, User};
use crate::{Page, Sort};

/// Retrieve details of users.
#[derive(Serialize)]
pub struct UsersFind<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  #[serde(flatten)]
  page: Page,
  #[serde(skip_serializing_if = "Sort::is_empty")]
  sort: Sort,

  /// Query in Lucene query string syntax. Some query types cannot be used on metadata fields, for details see Searchable Fields.
  #[serde(skip_serializing_if = "String::is_empty")]
  q: String,

  #[serde(skip_serializing_if = "String::is_empty")]
  search_engine: String,
}

impl<'a> UsersFind<'a> {
  /// Create find users request.
  pub fn new(client: &'a Auth0Client, q: Option<String>) -> Self {
    Self {
      client,

      page: Default::default(),
      sort: Default::default(),
      q: q.unwrap_or_default(),
      search_engine: "v3".to_string(),
    }
  }
}

impl<'a> AsMut<Page> for UsersFind<'a> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl<'a> AsMut<Sort> for UsersFind<'a> {
  fn as_mut(&mut self) -> &mut Sort {
    &mut self.sort
  }
}

impl<'a> UsersFind<'a> {
  /// Send
  pub async fn send<AOut, UOut>(&self) -> Auth0Result<Vec<User<AOut, UOut>>>
  where
    AOut: DeserializeOwned,
    UOut: DeserializeOwned,
  {
    self
      .client
      .send(self.client.begin(Method::GET, "api/v2/users").query(self))
      .await
  }
}
