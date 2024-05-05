use std::fmt::{Debug, Display};

// 기본 구현
pub trait Summary {
    fn 작가_요약(&self) -> String;
    fn 요약(&self) -> String {
        format!("(Read more from {}...)", self.작가_요약())
    }
    // 원래 트레이트 내 모든 메소드를 impl 시 구현해야 하지만, 내부에서 참조하는 경우에는 그렇게 하지 않아도 된다.
}

pub struct 뉴스기사 {
    pub 제목: String,
    pub 소속: String,
    pub 저자: String,
    pub 본문: String,
}

// 어떤 타입에 대한 트레이트를 구현하는 것은, 평범한 메소드를 구현하는 것과 비슷하다. 다른 점이라면 impl 뒤에 구현하고자 하는 트레이트 이름(Summary)을 적고 그다음 for 키워드와 트레이트를 구현할 타입명(뉴스기사)을 적는다는 점이다.
impl Summary for 뉴스기사 {
    fn 작가_요약(&self) -> String {
        format!("@{}", self.저자)
    }
}

pub struct 트윗 {
    pub 사용자: String,
    pub 내용: String,
    pub 댓글: bool,
    pub 리트윗: bool,
}

impl Summary for 트윗 {
    fn 작가_요약(&self) -> String {
        format!("@{}", self.사용자)
    }
}

// 매개변수로서의 트레이트
pub fn 알림(아이템: &impl Summary) {
    println!("긴급 속보! {}", 아이템.요약());
    // 아이템의 구체적 타입을 명시하는 대신 `impl` 키워드와 트레이트 이름을 명시했다.
    // 이 매개변수에는, 지정된 트레이트를 구현하는 타입이라면 어떤 타입이든 전달받을 수 있다.
}

// 트레이트 바운드 문법
pub fn 알림_2<T: Summary>(아이템: &T) {
    println!("긴급 속보! {}", 아이템.요약());
    // 트레이트 바운드는, 꺾쇠괄호 안의 제네릭 타입 매개변수 선언에 붙은 : 뒤에 위치한다.
    // 더 복잡한 상황을 표현할 때 좋다. 예를 들어, 여러 개의 트레이트를 구현한 타입을 받아들이는 경우.
}

// pub fn 알림(아이템1: &impl Summary, 아이템2: &impl Summary) {}
// 트레이트 바운드 사용시: pub fn 알림<T: Summary>(아이템1: &T, 아이템2: &T) {

// 트레이트 바운드는 여러개 지정될 수도 있음. 예를 들어 출력 포매팅까지 사용하고 싶을 경우.
pub fn 알림_3(아이템: &(impl Summary + Display)) {
    println!("긴급 속보! {}", 아이템.요약());
}
pub fn 알림_4<T: Summary + Display>(아이템: &T) {
    println!("긴급 속보! {}", 아이템.요약());
}
// 알림_3과 알림_4는 같은 역할을 함.

// 트레이트 바운드가 너무 많아지면 여러 제네릭 타입 파라미터를 사용하는 함수는, 함수명과 파라미터 사이에 너무 많은 트레이트 바운드 정보를 담게 될 가능성이 있다.
// 이때 where 절을 사용하면, 함수 시그니처를 더 깔끔하게 만들어 가독성을 높일 수 있다.
pub fn 어떤_함수<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    5
}
// 위처럼 하는 대신 아래처럼 하면 트레이트 바운드로 도배되지 않고, 평범한 함수처럼 "함수명", "파라미터 목록", "리턴 타입"이 붙어있으니, 함수 시그니처를 읽기 쉬워진다.
pub fn where_를_쓰는_어떤_함수<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

// 트레이트를 구현하는 타입을 반환하기
// 아래처럼, impl 트레이트를 리턴 타입에 써서 Summary 트레이트를 구현하는 타입을 반환하는 함수를 작성할 수 있다.
fn 요약_생성() -> impl Summary {
    트윗 {
        사용자: String::from("horse_ebooks"),
        내용: String::from("of course, as you probably already know, people"),
        댓글: false,
        리트윗: false,
    }
}
// 이 경우 함수를 호출하는 쪽에서는 구체적인 타입을 알 필요가 없다. 그러나 아래와 같이 상황에 따라서 여러 타입을 리턴할 수 없다.
// fn 요약_생성_2(조건: bool) -> impl Summary {
//     if 조건 {
//         트윗 {
//             사용자: String::from("horse_ebooks"),
//             내용: String::from("of course, as you probably already know, people"),
//             댓글: false,
//             리트윗: false,
//         }
//     } else {
//         뉴스기사 {
//             제목: String::from("프로그래밍 언어 Rust의 특징"),
//             소속: String::from("Rust 프로그래밍 언어"),
//             저자: String::from("Rustacean"),
//             본문: String::from("Rust는 안전하고 빠르며, 실수를 줄이는 프로그래밍 언어입니다."),
//         }
//     }
// }
// 트윗 혹은 뉴스기사 중 하나를 반환하는 행위는, `impl Trait` 문법이 컴파일러 내에 구현된 방식으로 인한 제약 때문에 허용되지 않는다.

// 트레이트 바운드를 사용해 조건부로 메서드 구현하기
// 제네릭 타입 파라미터를 사용하는 impl 블록에 트레이트 바운드를 이용하면, 지정된 트레이트를 구현하는 타입에 대해서만 메소드를 구현할 수도 있다.
// 아래 예시는, T 타입이 Display 트레이트를 구현하는 경우에만 요약_표시 메소드를 구현한다.

struct 쌍<T> {
    x: T,
    y: T,
}

// 쌍<T> 타입은 언제나 새로운 쌍<T> 인스턴스를 반환하는 new 함수를 구현한다.
impl<T> 쌍<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 하지만 여기서는, 어떤 T 타입이 비교를 가능하게 해주는 PartialOrd 트레이트와, 출력을 가능하게 만드는 Display 트레이트를 모두 구현한 타입인 경우에 대해서만 "cmp_display" 메소드를 구현한다.
impl<T: Display + PartialOrd> 쌍<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("가장 큰 멤버는 x: {}", self.x);
        } else {
            println!("가장 큰 멤버는 y: {}", self.y);
        }
    }
}

// 타입이 특정 트레이트를 구현하는 경우에만, 해당 타입에 트레이트를 구현할 수도 있다.
// 트레이트 바운드를 만족하는 모든 타입에 대해 트레이트를 구현하는 것을 포괄 구현(blanket implementations)이라고 하고, 러스트 표준 라이브러리에서 광범위하게 사용된다.
// ex) 표준 라이브러리는 Display 트레이트를 구현하는 모든 타입에 ToString 트레이트도 구현한다.
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        todo!()
    }
}
// Display 트레이트가 구현된 모든 타입에서 to_string() 메소드를 호출할 수 있는 건, 표준 라이브러리의 포괄 구현 덕분이다.
// 예를들어 정수는 Display를 구현하므로, to_string() 메소드를 호출하여 String 값으로 변환할 수 있다.
// let s = 3.to_string();
