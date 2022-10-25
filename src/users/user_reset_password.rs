//! Trigger password reset.
use reqwest::Method;
use serde::{Serialize};

use crate::{Auth0Client, Auth0Request, Auth0Result};

/// Trigger password reset.
#[derive(Serialize)]
pub struct UserResetPassword<'a> {
    #[serde(skip_serializing)]
    client: &'a Auth0Client,

    email: String,
    connection: String,
    client_id: String,
}

impl<'a> UserResetPassword<'a> {
    /// Create reset password request.
    pub fn new(client: &'a Auth0Client, email: &str, connection: &str, client_id: &str) -> Self {
        Self {
            client,

            email: email.to_owned(),
            connection: connection.to_owned(),
            client_id: client_id.to_owned(),
        }
    }
}

impl<'a> UserResetPassword<'a> {
    /// Send
    pub async fn send(&self) -> Auth0Result<String>
    {
        self
            .client
            .send(self.client.begin(Method::POST, "dbconnections/change_password").json(self))
            .await
    }
}
