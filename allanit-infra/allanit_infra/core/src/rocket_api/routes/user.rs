use common::auth;
use common::database::models::user::{
    NewUser, NewUserRequest, UpdateUserRequest, User, UserResponse,
};
use common::database::repositories::UserRepository;
use common::rocket::DbConn;
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, head, post, put, routes, Route};
use rocket_db_pools::Connection;

pub fn routes() -> Vec<Route> {
    routes![
        get_user_by_id,
        create_user,
        delete_user,
        update_user,
        find_user_by_email,
        find_user_by_username,
        search_by_username,
        search_by_email,
        list_users,
        exists_user_by_email,
        exists_user_by_username,
    ]
}

/* ===================== ⚙️ User API Overview =====================

== 🛠️ CRUD ==
• GET     /users/:id                         → Fetch user by ID                       → 200 OK (User)
• POST    /users                             → Create new user (NewUserRequest)      → 201 Created (UserResponse)
• DELETE  /users/:id                         → Delete user by ID                     → 204 No Content
• PUT     /users/:id                         → Update user by ID (UpdateUserRequest) → 200 OK (UserResponse)

== 🔍 Lookup ==
• GET     /users/email/:email                → Find user by email                    → 200 OK (User)
• GET     /users/username/:username          → Find user by username                 → 200 OK (Option<User>)

== 🔍 Search ==
• GET     /users/search/username?<q>         → Search users by username              → 200 OK (Vec<User>)
• GET     /users/search/email?<q>            → Search users by email                 → 200 OK (Vec<User>)

== 📑 Listing ==
• GET     /users?<page>&<limit>             → List all users (paginated)            → 200 OK (Vec<User>)

== ⚡ Existence Checks ==
• HEAD    /users/exists/email/:email         → Exists by email                       → 200 OK / 404 Not Found
• HEAD    /users/exists/username/:username   → Exists by username                    → 200 OK / 404 Not Found

== 🔗 Relational & Aggregation ==
• GET     /users/with-jobs                   → Users with jobs                       → 200 OK (Vec<User>)
• GET     /users/job-counts                  → User job counts                       → 200 OK (Vec<(User, i64)>)

======================================================================== */

// === CRUD ===
#[get("/users/<id>")]
pub async fn get_user_by_id(
    mut conn: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Json<User>, Custom<Json<Value>>> {
    UserRepository::find_by_id(&mut conn, id)
        .await
        .map(Json)
        .map_err(|e| Custom(Status::NotFound, Json(json!({"error": e.to_string()}))))
}

#[post("/users", format = "json", data = "<new_user_req>")]
pub async fn create_user(
    mut conn: Connection<DbConn>,
    new_user_req: Json<NewUserRequest>,
) -> Result<Custom<Json<UserResponse>>, Custom<Json<Value>>> {
    // Validate password
    if !auth::is_password_valid(&new_user_req.password) {
        return Err(Custom(
            Status::BadRequest,
            Json(json!({
                "error": "Password must be 8–128 characters long and contain both letters and digits."
            })),
        ));
    }

    // Hash password
    let hashed = match auth::hash_password(new_user_req.password.clone()) {
        Ok(p) => p,
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                Json(json!({"error": "Password hashing failed"})),
            ))
        }
    };

    // Construct NewUser properly
    let new_user = NewUser {
        username: new_user_req.username.clone(),
        email: new_user_req.email.clone(),
        password_hash: hashed,
    };

    UserRepository::create(&mut conn, new_user)
        .await
        .map(|u| {
            let resp: UserResponse = u.into();
            Custom(Status::Created, Json(resp))
        })
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })
}

#[delete("/users/<id>")]
pub async fn delete_user(
    mut conn: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Json<Value>>> {
    UserRepository::delete(&mut conn, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })
}

#[put("/users/<id>", format = "json", data = "<update_req>")]
pub async fn update_user(
    id: i32,
    update_req: Json<UpdateUserRequest>,
    mut conn: Connection<DbConn>,
) -> Result<Json<UserResponse>, Custom<Json<Value>>> {
    let user = match UserRepository::find_by_id(&mut conn, id).await {
        Ok(u) => u,
        Err(_) => {
            return Err(Custom(
                Status::NotFound,
                Json(json!({"error": "User not found"})),
            ))
        }
    };

    // Hash password if provided
    let password_hash = if let Some(pwd) = &update_req.password {
        if !auth::is_password_valid(pwd) {
            return Err(Custom(
                Status::BadRequest,
                Json(json!({
                    "error": "Password must be 8–128 characters long and contain both letters and digits."
                })),
            ));
        }
        auth::hash_password(pwd.clone().to_string()).map_err(|_| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": "Password hashing failed"})),
            )
        })?
    } else {
        user.password_hash.clone() // preserve old one
    };

    let updated = User {
        id: user.id,
        username: update_req.username.clone(),
        email: update_req.email.clone(),
        password_hash,
        created_at: user.created_at,
    };

    let result = UserRepository::update(&mut conn, user.id, updated)
        .await
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(result.into()))
}

// === Lookup ===
#[get("/users/email/<email>")]
pub async fn find_user_by_email(
    mut conn: Connection<DbConn>,
    email: String,
    _user: User,
) -> Result<Json<User>, Custom<Json<Value>>> {
    let user = UserRepository::find_by_email(&mut conn, &email)
        .await
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(Custom(
            Status::NotFound,
            Json(json!({"error": "User not found"})),
        )),
    }
}

#[get("/users/username/<username>")]
pub async fn find_user_by_username(
    mut conn: Connection<DbConn>,
    username: String,
    _user: User,
) -> Result<Json<Option<User>>, Custom<Json<Value>>> {
    UserRepository::find_by_username(&mut conn, &username)
        .await
        .map(Json)
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })
}

// === Search ===
#[get("/users/search/username?<q>")]
pub async fn search_by_username(
    mut conn: Connection<DbConn>,
    q: String,
    _user: User,
) -> Result<Custom<Json<Vec<User>>>, Custom<Json<Value>>> {
    UserRepository::search_by_username(&mut conn, &q)
        .await
        .map(|data| Custom(Status::Ok, Json(data)))
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })
}

#[get("/users/search/email?<q>")]
pub async fn search_by_email(
    mut conn: Connection<DbConn>,
    q: String,
    _user: User,
) -> Result<Custom<Json<Vec<User>>>, Custom<Json<Value>>> {
    UserRepository::search_by_email(&mut conn, &q)
        .await
        .map(|data| Custom(Status::Ok, Json(data)))
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })
}

// === Listing ===
#[get("/users?<page>&<limit>")]
pub async fn list_users(
    mut conn: Connection<DbConn>,
    page: Option<u32>,
    limit: Option<u32>,
    _user: User,
) -> Result<Custom<Json<Vec<User>>>, Custom<Json<Value>>> {
    let limit = limit.unwrap_or(50);
    let offset = page.unwrap_or(0) * limit;

    UserRepository::list_all(&mut conn, limit as i64, offset as i64)
        .await
        .map(|data| Custom(Status::Ok, Json(data)))
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": e.to_string()})),
            )
        })
}

// === Existence Checks ===
#[head("/users/exists/email/<email>")]
pub async fn exists_user_by_email(
    mut conn: Connection<DbConn>,
    email: &str,
    _user: User,
) -> Result<NoContent, Status> {
    match UserRepository::exists_by_email(&mut conn, email).await {
        Ok(true) => Ok(NoContent),
        Ok(false) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[head("/users/exists/username/<username>")]
pub async fn exists_user_by_username(
    mut conn: Connection<DbConn>,
    username: &str,
    _user: User,
) -> Result<NoContent, Status> {
    match UserRepository::exists_by_username(&mut conn, username).await {
        Ok(true) => Ok(NoContent),
        Ok(false) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
