// 한편, `use` 키워드로 구조체나 이넘 등의 타 아이템을 가져올 때는, 전체 경로를 작성하는 것이 보편적이다.
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 하지만 동일한 이름의 아이템을 여럿 가져오는 경우의 방식은 허용되지 않는다.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // ..
}

fn function2() -> io::Result<()> {
    // ..
}
// 부모 모듈을 명시하여 두 개의 Result 타입을 구별하고 있다.
// `use std::fmt::Result`, `use std::io::Result`로 작성했다면, 동일한 스코프 내에 두개의 `Result` 타입이 존재하므로 Rust는 우리가 어떤 `Result` 타입을 사용했는지 알 수가 없다.
// 물론 이런 경우라고 하더라도, `as OOO`의 형태로 새로운 이름으로 사용하게 해준다면 해결할 수 있다. (React에서도 똑같이 사용할 수 있는 기능)
// `use std::fmt::Result;`, `use std::io::Result as IoResult` 의 형태로 사용하면 된다.
