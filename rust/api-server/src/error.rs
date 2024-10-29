use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Sqlx(StatusCode, String), // Sqlx에서 오는 에러를 HTTP 상태 코드와 메시지로 변환
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Error {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound, // 일치하는 값 없을 때 404 리턴
            _ => Error::Sqlx(
                StatusCode::INTERNAL_SERVER_ERROR, // 반대 케이스는 500 리턴하고, 에러 메시지 전달
                err.to_string(),
            ),
        }
    }
}

impl IntoResponse for Error { // 에러를 응답으로 반환하기 위해 IntoResponse 구현
    fn into_response(self) -> Response {
        match self {
            Error::Sqlx(code, body) => (code, body).into_response(),
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}