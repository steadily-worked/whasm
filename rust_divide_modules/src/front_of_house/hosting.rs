pub fn add_to_waitlist() {}
// `hosting.rs`를 `src/front_of_house` 대신 `src` 디렉토리에 넣을 경우, 컴파일러는 `hosting.rs` 코드가 `front_of_house` 모듈의 하위에 선언되지 않고 크레이트 루트에 선언된 `hosting` 모듈에 있을 것으로 예상한다.
// 어떤 파일에서 어떤 모듈의 코드를 확인할 지에 대한 컴파일러의 규칙은, 디렉토리와 파일이 모듈 트리와 더 밀접하게 일치한다는 것을 의미한다.
