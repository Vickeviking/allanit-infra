///! Mostly used within commanddeck, that's why some functions have a lot of prints, for the UI
use anyhow::Context;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::auth;
use crate::database::models::user::{NewUser, User};
use crate::database::repositories::UserRepository;

/// Load database connection,
/// Error if DB not up
#[allow(clippy::expect_used)]
pub async fn load_db_connection() -> anyhow::Result<AsyncPgConnection, anyhow::Error> {
    let database_url =
        std::env::var("DATABASE_URL").context("Cannot load DB url from environment")?;
    let conn = AsyncPgConnection::establish(&database_url)
        .await
        .context("Cannot connect to Postgres")?;
    Ok(conn)
}

/// Create a new user
/// # Arguments
/// * `username` - The username of the user
/// * `email` - The email of the user
/// * `password` - The password of the user, unhashed
/// # Returns
/// * `anyhow::Result<(), anyhow::Error>`
/// # Example
/// ```
/// use swarm_core::commands::create_user;
///
/// let result = create_user("username", "email", "password").await;
/// assert!(result.is_ok());
/// ```
/// # Errors
/// * Hashing failed
/// * UserRepository create failed
pub async fn create_user(
    username: String,
    email: String,
    password: String,
) -> anyhow::Result<(), anyhow::Error> {
    //load in db connection
    let mut c = load_db_connection().await?;

    // hash pashword
    let password_hash = auth::hash_password(password).context("Hashing failed")?;
    // create insertable user struct
    let new_user = NewUser {
        username,
        email,
        password_hash,
    };

    // create user with user repository func
    let created = UserRepository::create(&mut c, new_user)
        .await
        .context("UserRepository create failed")?;
    println!("✅ Created user: {} ({})", created.username, created.email);
    Ok(())
}

/// Get a user by id,
/// # Arguments
/// * `id` - The id of the user
/// # Returns
/// * `anyhow::Result<User, anyhow::Error>`
/// # Example
/// ```
/// use swarm_core::commands::get_user_by_id;
///
/// let user = get_user_by_id(1).await;
/// assert!(user.is_ok());
/// ```
/// # Errors
/// * UserRepository find_by_id failed
/// * DB connection failed
pub async fn get_user_by_id(id: i32) -> Result<User, anyhow::Error> {
    let mut c = load_db_connection().await?;
    let user = UserRepository::find_by_id(&mut c, id).await?;
    Ok(user)
}

/// Print all users, DOES NOT RETURN USERS
/// # Arguments
/// * `limit` - The limit of users to list
/// * `offset` - The offset of users to list
/// # Returns
/// * `anyhow::Result<(), anyhow::Error>`
pub async fn list_users(limit: i64, offset: i64) -> anyhow::Result<(), anyhow::Error> {
    // load db connection
    let mut c = load_db_connection().await?;
    // fetch relevant users
    let users = UserRepository::list_all(&mut c, limit, offset)
        .await
        .context("list all failed from UserRepository")?;

    // Print users
    println!("📄 Listing users (limit: {}, offset: {}):", limit, offset);
    for user in users {
        println!("({})- {} <{}>", user.id, user.username, user.email);
    }
    Ok(())
}

/// update user
/// # Arguments
/// * `id` - The id of the user
/// * `username` - The username of the user
/// * `email` - The email of the user
/// * `password` - The password of the user (unhashed)
/// # Returns
/// * `anyhow::Result<(), anyhow::Error>`
/// # Example
/// ```
/// use swarm_core::commands::update_user;
///
/// let result = update_user(1, "username", "email", "password").await;
/// assert!(result.is_ok());
/// ```
/// # Errors
/// * UserRepository update failed
/// * DB connection failed
/// * Hashing failed
/// * non unique username or email
/// * non existing user_id
pub async fn update_user(
    id: i32,
    username: String,
    email: String,
    password: String,
) -> anyhow::Result<(), anyhow::Error> {
    // load db
    let mut c = load_db_connection().await?;

    // created_at not updated but needed , so we make a dummy
    let d = NaiveDate::from_ymd_opt(2004, 1, 9).context("invalid time placeholder")?;
    let t = NaiveTime::from_hms_opt(0, 0, 0).context("invalid time placeholder")?;
    let dt = NaiveDateTime::new(d, t);

    // lets hash the password
    let password_hash =
        auth::hash_password(password).context("password hashing failed with argon2")?;

    // create the user
    let user = User {
        id,
        username,
        email,
        password_hash,
        created_at: dt,
    };

    // update the user
    let updated = UserRepository::update(&mut c, id, user)
        .await
        .context("User update failed in UserRepository")?;

    // print message
    println!(
        "✏️ Updated user {} -> {} ({})",
        id, updated.username, updated.email
    );
    Ok(())
}

/// Delete a user by id
/// # Arguments
/// * `id` - The id of the user
/// # Returns
/// * `anyhow::Result<(), anyhow::Error>`
/// # Example
/// ```
/// use swarm_core::commands::delete_user;
///
/// let result = delete_user(1).await;
/// assert!(result.is_ok());
/// ```
/// # Errors
/// * UserRepository delete failed
/// * DB connection failed
/// * non existing user
pub async fn delete_user(id: i32) -> anyhow::Result<(), anyhow::Error> {
    // load db conn
    let mut c = load_db_connection().await?;

    // fire delete funciton from within repo
    let deleted = UserRepository::delete(&mut c, id)
        .await
        .context("UserRepository delete failed")?;

    // print relevant messages.
    if deleted > 0 {
        println!("🗑️ Deleted user with id {}", id);
    } else {
        println!("⚠️ No user found with id {}", id);
    }
    Ok(())
}

/// Delete a range of users, inclusive
/// # Arguments
/// * `start` - The start of the range
/// * `end` - The end of the range
/// # Returns
/// * `anyhow::Result<(), anyhow::Error>`
/// # Example
/// ```
/// use swarm_core::commands::delete_many_users;
///
/// // deletes users {1, 2, ..., 10}
/// let result = delete_many_users(1, 10).await;
/// assert!(result.is_ok());
/// ```
/// # Errors
/// * UserRepository delete failed
pub async fn delete_many_users(start: i32, end: i32) -> anyhow::Result<(), anyhow::Error> {
    let mut c = load_db_connection().await?;

    for id in start..=end {
        match UserRepository::delete(&mut c, id).await {
            Ok(0) => println!("⚠️ No user found with id {}", id),
            Ok(1) => println!("🗑️ Deleted user with id {}", id),
            Ok(n) => println!("🗑️ Deleted {} users with id {} (unexpected)", n, id),
            Err(e) => println!("❌ Error deleting user {}: {}", id, e),
        }
    }

    Ok(())
}
