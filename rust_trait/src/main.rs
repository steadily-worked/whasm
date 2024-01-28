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
}

// aggregator 크레이트에 의존적인 다른 크레이트들 또한 Summary 크레이트를 스코프로 가져와서 자신들의 타입에 대해 Summary를 구현할 수 있다.
// trait 구현에는 한가지 제약사항이 있다: trait이나 trait을 구현할 타입 둘 중 하나는 반드시 자신의 크레이트 것이어야 해당 타입에 대한 trait를 구현할 수 있다는 점이다.
// ex1) `aggregator` 크레이트의 일부 기능으로 `Tweet` 타입에 표준 라이브러리 trait인 `Display` 등을 구현할 수 있다. `Tweet` 타입이 우리가 만든 `aggregator` 크레이트의 타입이기 때문이다.
// ex2) `aggregator` 크레이트에서 `Vec<T>` 타입에 `Summary` trait을 구현할 수도 있다. 마찬가지로 `Summary` trait가 우리가 만든 `aggregator` 크레이트의 trait이기 때문이다.
// 하지만 외부 타입에 외부 크레이트를 구현할 수는 없다.
// ex) `aggregator` 크레이트에서는 `Vec<T>`에 대한 `Display` trait을 구현할 수 없다. `Vec<T>`, `Display` 모두 직접 만든 크레이트가 아닌, 표준 라이브러리에 정의되어 있는 것들이기 때문이다.
