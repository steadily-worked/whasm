use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

use aggregator::{NewsArticle, Summary, Tweet};
// lib.rs에서 정의한 메소드들에 대해 `Cargo.toml`에서 package의 name을 aggregator로 지정해 준 이후부터 위와같이 불러와서 사용할 수 있게 된다.

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // 1 new tweet: horse_ebooks: of course, as you probably already know, people
    // 1 new tweet: (Read more from @horse_ebooks...) summarize_author를 구현한 경우 이와 같이 바뀌게 된다.
    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
        // New article available! (Read more...)
        // impl Summary for NewsArticle {} 로 정의했을 때를 기준으로 한다.
    }
    {
        // '어떤' 타입의 `item` 파라미터에서 summarize 메소드를 호출하는 notify 함수
        // pub fn notify(item: &impl Summary) {
        // impl Trait 문법은 사실 trait bound로 일러진, 더 긴 형식의 syntactic sugar이다. trait bound는 아래와 같다.
        pub fn notify<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }
        // item 파라미터의 구체적 타입을 명시하는 대신 impl 키워드와 trait 이름을 명시했다. 이 파라미터에는 지정된 trait을 구현하는 타입이라면 어떤 타입이든 전달받을 수 있다.
        // notify 본문 내에서는 item에서 summarize와 같은 Summary trait의 모든 메소드를 호출할 수 있다. notify는 NewsArticle 인스턴스로도, Tweet 인스턴스로도 호출할 수 있다.
        // 만약 Summary trait를 구현하지 않는 String, i32 등의 타입으로 notify 함수를 호출하는 코드를 작성한다면 컴파일 에러가 발생한다.

        // impl trait이 단순한 상황에서는 편리하고 코드를 더 간결하게 해주지만 trait bound 문법은 더 복잡한 상황을 표현할 수 있다.
        pub fn _notify(item1: &impl Summary, item2: &impl Summary) {}
        pub fn _notify2<T: Summary>(item1: &T, item2: &T) {}
        // 만약 두 매개변수가 같은 타입으로 강제되어야 한다면, 위와 같이 trait bound를 사용해야 한다.
    }
    {
        // trait bound는 여러개 지정될 수도 있다.
        pub fn _notify(item: &(impl Summary + Display)) {
            // ...
            // notify 정의를 할 때 item이 Display, Summary를 모두 구현해야 하도록 지정해야 할 경우 위와 같이 선언하면 된다.
        }
        pub fn _notify2<T: Summary + Display>(item: &T) {
            // ...
        }

        // trait bound가 너무 많아지는 것을 방지하기 위해 Rust는 trait bound를 함수 시그니처 뒤의 `where` 조항에 명시하는 대안을 제공한다.
        fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
            // ...
        }
        // 위와 같이 명시하는 대신에 아래와 같이 where 조항을 사용할 수 있다.
        fn _some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            // ...
        }
        // trait bound로 도배되지 않고 평범한 함수처럼 파라미터 목록, 반환 타입이 붙어있으니 함수 시그니처를 읽기 쉬워진다.
        {
            fn returns_summarizable() -> impl Summary {
                Tweet {
                    username: String::from("horse_ebooks"),
                    content: String::from("of course, as you probably already know, people"),
                    reply: false,
                    retweet: false,
                }
            }
            // 반환 타입에 구체적 타입명이 아닌 `impl Summary`를 작성하여, `returns_summarizable` 함수는 `Summary` trait을 구현하는 타입을 반환한다고 명시했다.
            // 위의 경우 `returns_summarizable`는 `Tweet`를 반환하지만 이 함수를 호출하는 쪽의 코드에서는 구체적인 타입을 알 필요가 없다.

            // 하지만, `impl Trait` 문법을 쓴다고 해서 다양한 타입을 반환할 수는 없다.
            fn _returns_summarizable(switch: bool) -> impl Summary {
                if switch {
                    NewsArticle {
                        headline: String::from("Penguins win the Stanley Cup Championship!"),
                        location: String::from("Pittsburgh, PA, USA"),
                        author: String::from("Iceburgh"),
                        content: String::from(
                            "The Pittsburgh Penguins once again are the best \
                             hockey team in the NHL.",
                        ),
                    }
                } else {
                    Tweet {
                        username: String::from("horse_ebooks"),
                        content: String::from("of course, as you probably already know, people"),
                        reply: false,
                        retweet: false,
                    }
                }
            }
            // NewsArticle, Tweet 중 하나를 반환하는 행위는 impl Trait 문법이 컴파일러 내에 구현된 방식으로 인한 제약 때문에 허용되지 않는다.
            // 함수가 이런 식으로 동작하도록 만드는 방법은 이후에 알아볼 예정이다.
        }
    }
}

// aggregator 크레이트에 의존적인 다른 크레이트들 또한 Summary 크레이트를 스코프로 가져와서 자신들의 타입에 대해 Summary를 구현할 수 있다.
// trait 구현에는 한가지 제약사항이 있다: trait이나 trait을 구현할 타입 둘 중 하나는 반드시 자신의 크레이트 것이어야 해당 타입에 대한 trait를 구현할 수 있다는 점이다.
// ex1) `aggregator` 크레이트의 일부 기능으로 `Tweet` 타입에 표준 라이브러리 trait인 `Display` 등을 구현할 수 있다. `Tweet` 타입이 우리가 만든 `aggregator` 크레이트의 타입이기 때문이다.
// ex2) `aggregator` 크레이트에서 `Vec<T>` 타입에 `Summary` trait을 구현할 수도 있다. 마찬가지로 `Summary` trait가 우리가 만든 `aggregator` 크레이트의 trait이기 때문이다.
// 하지만 외부 타입에 외부 크레이트를 구현할 수는 없다.
// ex) `aggregator` 크레이트에서는 `Vec<T>`에 대한 `Display` trait을 구현할 수 없다. `Vec<T>`, `Display` 모두 직접 만든 크레이트가 아닌, 표준 라이브러리에 정의되어 있는 것들이기 때문이다.
