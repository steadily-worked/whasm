use axum::{routing::get, Router};
use sqlx::{Pool, Sqlite};

pub async fn create_router(dbpool: Pool<Sqlite>) -> Router {
    use crate::api::{ping, todo_create, todo_delete, todo_list, todo_read, todo_update};
    use tower_http::cors::{Any, CorsLayer};
    use tower_http::trace::TraceLayer;

    Router::new()
        .route("/alive", get(|| async { "ok" }))
        // 현재 서버가 살아있는지 확인. 무조건 ok만 반환함
        .route("/ready", get(ping))
        // 현재 서버가 준비되었는지 확인. 데이터베이스 연결 확인
        .nest(
            "/v1",
            Router::new()
                .route("/todos", get(todo_list).post(todo_create))
                .route(
                    "/todos/:id",
                    get(todo_read).put(todo_update).delete(todo_delete),
                ),
        )
        .with_state(dbpool)
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
