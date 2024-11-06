use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::Error;
use crate::todo::{CreateTodo, Todo, UpdateTodo};

// 데이터베이스 연결이 정상적으로 작동하는지 확인하는 헬스체크 엔드포인트.
// dbpool에서 연결을 가져와서 데이터베이스에 ping을 보내고, 성공하면 "ok" 문자열을 반환
// 실패하면 Error를 반환
pub async fn ping(State(dbpool): State<SqlitePool>) -> Result<String, Error> {
    use sqlx::Connection;

    let mut conn = dbpool.acquire().await?;
    conn.ping()
        .await
        .map(|_| "ok".to_string())
        .map_err(Into::into)
}

pub async fn todo_list(State(dbpool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, Error> {
    Todo::list(dbpool).await.map(Json::from)
}

pub async fn todo_read(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, Error> {
    Todo::read(dbpool, id).await.map(Json::from)
}

pub async fn todo_create(
    State(dbpool): State<SqlitePool>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<Json<Todo>, Error> {
    Todo::create(dbpool, new_todo).await.map(Json::from)
}

pub async fn todo_update(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, Error> {
    Todo::update(dbpool, id, updated_todo).await.map(Json::from)
}

pub async fn todo_delete(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), Error> {
    Todo::delete(dbpool, id).await
}
