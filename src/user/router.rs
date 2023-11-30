use super::models::{CreateUser, User};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

pub fn router() -> Router {
    // `POST /users`
    Router::new()
        .route("/:id", get(get_user))
        .route("/", post(create_user))
}

pub async fn get_user(
    Path(raw_id): Path<String>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let id = raw_id.parse::<u64>();
    if id.is_err() {
        return Err((StatusCode::BAD_REQUEST, String::from("Invalid ID")));
    }

    let user = User {
        id: id.unwrap(),
        username: "jill".to_string(),
    };

    Ok((StatusCode::OK, Json(user)))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
