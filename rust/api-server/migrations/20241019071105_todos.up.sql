-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos { 
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, -- 자동 증가(AUTOINCREMENT)에 SQLite 기능 사용
    body TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
}
-- 어떤 필드도 NULL일 수 없고, 지정되지 않은 값을 가질 수 없다.
-- body를 제외한 모든 필드의 기본값이 있으므로, body만 받아도 생성 가능하다.
