use anyhow::{anyhow, ensure, Context};
use chrono::{NaiveDateTime, Utc};
use diesel_async::AsyncPgConnection;
use reqwest::{header, Client, ClientBuilder, StatusCode};
use serde_json::json;
use uuid::Uuid;

use common::commands;
use common::database::models::user::UserResponse;

// ===== UTILITIES =====
pub fn get_ndt_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

// ========== Constants ==========
pub const APP_HOST: &str = "http://localhost:8000";
pub const TEST_PASSWORD: &str = "ADMINPASSWORD123";

// ========== HTTP Client Setup ==========
pub fn http_client() -> Client {
    Client::new()
}

// ========== User Utilities ==========

pub fn generate_unique_username() -> String {
    format!("test_admin_{}", Uuid::new_v4())
}

/// Create a user via the HTTP API
pub async fn create_user_via_api(
    client: &Client,
    username: &str,
) -> anyhow::Result<UserResponse, anyhow::Error> {
    let email = format!("{}@example.com", username);
    let res = client
        .post(&format!("{}/users", APP_HOST))
        .json(&json!({
            "username": username,
            "email": email,
            "password": TEST_PASSWORD
        }))
        .send()
        .await
        .context("Failed to send create-user request")?;

    assert!(
        res.status().is_success(),
        "User creation failed (status={}): {:?}",
        res.status(),
        res.text().await.unwrap_or_default()
    );

    let user_resp = res
        .json::<UserResponse>()
        .await
        .context("Failed to deserialize User from create response")?;

    Ok(user_resp)
}

/// Attempt login via HTTP API, returning the response
pub async fn login_user(
    client: &Client,
    username: &str,
    password: &str,
) -> anyhow::Result<reqwest::Response, anyhow::Error> {
    let resp = client
        .post(&format!("{}/login", APP_HOST))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .context("Login request failed")?;

    Ok(resp)
}

/// Returns a logged-in client and the corresponding user
pub async fn build_client_with_logged_in_user(
) -> anyhow::Result<(Client, UserResponse), anyhow::Error> {
    let client = http_client();

    let username = generate_unique_username();
    let user = create_user_via_api(&client, &username)
        .await
        .context("Failed to create user with api")?;

    let login_resp = login_user(&client, &username, TEST_PASSWORD).await?;

    ensure!(login_resp.status() == StatusCode::OK,);

    let token = login_resp.json::<serde_json::Value>().await.unwrap()["token"]
        .as_str()
        .ok_or_else(|| anyhow!("could not extract login token"))?
        .to_string();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).context("invalid tokens")?,
    );

    let authorized_client = ClientBuilder::new().default_headers(headers).build()?;

    Ok((authorized_client, user))
}

/// Delete user via API by ID
pub async fn delete_user_via_api(client: &Client, id: i32) -> anyhow::Result<(), anyhow::Error> {
    let res = client
        .delete(&format!("{}/users/{}", APP_HOST, id))
        .send()
        .await
        .context("Delete-user request failed")?;

    assert!(
        res.status().is_success() || res.status() == StatusCode::NO_CONTENT,
        "Delete-user failed: status={} body={}",
        res.status(),
        res.text().await.unwrap_or_default()
    );

    Ok(())
}

/// Delete multiple users
pub async fn delete_users_via_api(
    client: &Client,
    user_ids: &[i32],
) -> anyhow::Result<(), anyhow::Error> {
    for &id in user_ids {
        delete_user_via_api(client, id)
            .await
            .context("Failed to delete user via api")?;
    }

    Ok(())
}
