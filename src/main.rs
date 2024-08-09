use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[cfg(test)]
mod test {
    use crate::create_app;
    use crate::User;
    use ::axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn should_return_hello_world() {
        let app = create_app();
        let server = TestServer::new(app).unwrap();
        let resp = server.get("/").await;
        resp.assert_status_ok();
        resp.assert_text("Hello, World!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let app = create_app();
        let server = TestServer::new(app).unwrap();
        let resp = server
            .post("/users")
            .json(&json!({
                "username": "田中 太郎"
            }))
            .await
            .json::<User>();
        assert_eq!("田中 太郎", resp.username);
    }
}
