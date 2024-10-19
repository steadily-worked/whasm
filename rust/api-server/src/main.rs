#[tokio::main]
async fn main() {
    init_tracing(); // 서비스 및 해당 라이브러리에 대한 추적 & 로깅 초기화
    let dbpool = init_dbpool().await.expect("couldn't initialize DB pool"); // DB 풀 초기화

    let router = create_router(dbpool).await; // 라우터 생성

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string()); // BIND_ADDR 환경변수에서 주소를 가져오거나, 없으면 127.0.0.1:3000을 기본으로 설정

    axum::Server::bind(&bind_addr.parse().unwrap()).serve(router.into_make_service()).await.expect("unable to start server")
    // BIND_ADDR를 소켓 주소로 파싱. 서비스 생성 후 HTTP 서버 시작
}

fn init_tracing() { // 추적 초기화 함수
    use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());
    // RUST_LOG 환경변수를 가져오고, 없으면 기본값을 제공함.
    // tower_http::*에 대한 디버그 수준 메시지, sqlx::*에 대한 정보 수준 메시지를 포함함.

    tracing_subscriber::registry() // 기본 전역 저장소 반환
        .with(fmt::layer()) // 텍스트 형식으로 로그 출력
        .with(
            EnvFilter::builder() // 환경변수 기반 필터 생성. 기본 로그 수준을 info로 설정하고 그렇지 않으면 RUST_LOG에서 제공하는 값을 사용함.
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log)
        )
        .init();
}

// 연결 pool을 사용해서 DB에 대한 연결을 얻음. connection pool 사용 시 각 요청에 대한 새로운 연결을 생성할 필요가 없이 
// DB에 대한 기존 연결을 얻어 재사용할 수 있음.
async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> { 
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.sqlite".to_string());
    // DATABASE_URL 환경변수에서 데이터베이스 연결 문자열을 가져오거나, 없으면 sqlite:db.sqlite를 기본으로 설정
    // 이 경우 현재 디렉토리 내 `db.sqlite` 라는 파일이 열림.

    let dbpool = SqlitePoolOptions::new().connect_with(
        SqliteConnectOptions::from_str(&db_connection_str)?
        .create_if_missing(true)) // DB가 없을 때 드라이버에 DB를 생성하도록 요청
        .await
        .expect("can't connect to DB");

    sqlx::migrate!()// DB 연결 후 마이그레이션 실행
        .run(&dbpool) // 새로 생성한 dbpool을 SQLx에 직접 전달
        .await
        .expect("can't run migrations");
    // 데이터베이스 마이그레이션 실행  

    Ok(dbpool)
}

