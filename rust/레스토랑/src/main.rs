use std::collections::HashMap;
use std::{fmt, io};
// use 키워드로 구조체, 열거형 등 타 아이템을 가져올 때는 전체 경로를 작성하는 게 보편적임.

// use std::fmt::Result;
// use std::io::Result as IoResult;

fn main() {
    let mut 맵 = HashMap::new();
    맵.insert(1, 2);

    // 같은 이름의 타입을 사용할 때는, 어떤 Result를 우리가 사용하고 싶은지 러스트가 알 수 있도록
    // 모듈까지만 불러와서 모듈::타입 의 형태로 작성해줘야 한다.
    // use std::fmt::Result, use std::io::Result; 로 불러오면 안됨.

    fn 함수1() -> fmt::Result {
        Ok(())
    }

    fn 함수2() -> io::Result<()> {
        Ok(())
    }
}
