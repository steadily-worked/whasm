use axum::Error;
use serde::Deserialize;
use sqlx::{query_as, SqlitePool};

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NativeDateTime,
    updated_at: NativeDateTime,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    body: String
}

impl CreateTodo {
    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    body: String,
    completed: bool,
}

impl UpdateTodo {
    pub fn body(&self) -> &str {
        self.body.as_str()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

impl Todo {
    // 전체 리스트 반환
    pub async fn list(dbpool: SqlitePool) -> Result<Vec<Todo>, Error> {
        query_as("select * from todos") // todos 테이블에서 모든 todo를 가져옴
        .fetch_all(&dbpool)
        .await
        .map_err(Into::into);
    }

    // id값에 해당하는 하나의 값 반환
    pub async fn read(dbpool: SqlitePool, id: i64) -> Result<Todo, Error> {
        query_as("select * from todos where id = ?") // todos 테이블에서 id가 일치하는 하나의 todo를 가져옴
          .bind(id)
          .fetch_one(&dbpool)
          .await
          .map_err(Into::into)
    }

    pub async fn create(
        dbpool: SqlitePool,
        new_todo: CreateTodo,
    ) -> Result<Todo, Error> {
        query_as("insert into todos (body) values (?) returning *") // 레코드 삽입 후 즉시 검색을 위해 `returning *`을 사용.
          .bind(new_todo.body())
          .fetch_one(&dbpool)
          .await
          .map_err(Into::into)
    }

    pub async fn update(dbpool: SqlitePool, id: i64, update_todo: UpdateTodo) -> Result<Todo, Error> {
        query_as("update todos set body = ?, completed = ?, updated_at = datetime('now') where where id = ? returning *")
        .bind(updated_todo.body()) // "?"를 사용하여, SQL 문 내 선언 순서대로 바인딩됨.
        .bind(updated_todo.completed())
        .bind(id)
        .fetch_one(&dbpool) // 이 쿼리의 실행으로, 하나의 행이 반환될 것으로 기대됨.
        .await
        .map_err(Into::into)
    }

    pub async fn delete(dbpool: SqlitePool, id: i64) -> Result<(), Error> {
        query_as("delete from todos where id = ?")
          .bind(id)
          .execute(&dbpool) // 레코드를 반환하지 않는 쿼리에 사용되는 쿼리를 실행하기 위해 execute()를 사용.
          .await?;

        Ok(()) // 성공 시 유닛 반환.
    }
}
