use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;

use crate::api::users::User;
use crate::request::Auth0Request;

/// Provides data used to request user from user_id field.
pub struct GetUser<AppMetadata, UserMetadata> {
  id: String,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> GetUser<AppMetadata, UserMetadata> {
  /// Create get user request.
  /// # Arguments
  /// * `id` - The ID of the user to retrieve.
  pub fn new(id: &str) -> Self {
    Self {
      id: id.to_owned(),
      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata: DeserializeOwned, UserMetadata: DeserializeOwned> Auth0Request
  for GetUser<AppMetadata, UserMetadata>
{
  type Response = User<AppMetadata, UserMetadata>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, &format!("api/v2/users/{}", self.id))
  }
}

/// Provides data used to request user from email field.
pub struct GetUserByEmail<AppMetadata, UserMetadata> {
  email: String,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> GetUserByEmail<AppMetadata, UserMetadata> {
  /// Create get user request.
  /// # Arguments
  /// * `email` - The email address of the user to retrieve.
  pub fn new(email: &str) -> Self {
    Self {
      email: email.to_owned(),
      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata: DeserializeOwned, UserMetadata: DeserializeOwned> Auth0Request
  for GetUserByEmail<AppMetadata, UserMetadata>
{
  type Response = User<AppMetadata, UserMetadata>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "api/v2/users/users-by-email")
      .query(&[("email", self.email.to_owned())])
  }
}
