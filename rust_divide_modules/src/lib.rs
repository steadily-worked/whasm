// 별개의 파일로 모듈 분리하기
mod front_of_house;
// 모듈 트리에서 `mod` 선언을 사용해서 파일을 로드하는 것은 한 번만 하면 된다. 그 파일이 프로젝트의 일부란 것을 컴파일러가 파악하면, 프로젝트의 다른 파일들은 선언된 위치의 경로를 사용하여 로드된 파일의 코드를 참조해야 한다.
// 즉, `mod`는 다른 프로그래밍 언어에서 볼 수 있는 '포함하기(include)' 연산이 아니다. 최초 선언된 위치또한 기억하고 있어야 한다.

pub use crate::front_of_house::hosting; // 같은 모듈 트리 내에 있는 sibling 관계이므로 pub mod로 불러오지 않아도 되는 것(?).

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// 러스트에서는 패키지를 여러 크레이트로 나누고, 크레이트를 여러 모듈로 나누어 한 모듈에 정의된 아이템을 다른 모듈에서 참조할 수 있게 해준다. 절대 경로나 상대 경로를 지정하면 이를 수행할 수 있다.
// `use` 구문을 통해 이러한 경로들을 스코프 안으로 가져올 수 있으므로, 해당 스코프에 있는 아이템을 여러 번 사용해야 할 때 더 짧은 경로를 사용할 수 있다.
// 모듈 코드는 기본적으로 비공개지만, `pub` 키워드를 추가해 정의를 공개할 수 있다.