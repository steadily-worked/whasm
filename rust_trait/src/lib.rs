// 공개 trait 선언
pub trait Summary {
    fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String; 원래는 이렇게 선언만 하는 것이지만, 아래처럼 기본 동작을 구현할 수도 있음.
    fn summarize(&self) -> String {
        // String::from("(Read more...)")
        // 기본 구현을 사용하려면, `impl Summary for NewsArticle {}` 처럼 비어있는 `impl` 블록을 명시하면 됨.

        // 기본 구현 안쪽에서 trait의 다른 메소드를 호출할 수도 있음.
        format!("(Read more from {}...)", self.summarize_author())
        // 이렇게 기본 구현이 있는 경우라면, 이 trait에 대한 impl을 작성할 때 기본 구현이 없는 `summarize_author` 메소드만 정의해주면 된다.
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// NewsArticle 구조체에 대해 헤드라인, 저자, 지역정보를 사용하여 `summarize`의 반환값을 만드는 `Summary` 트레이트를 구현
impl Summary for NewsArticle {
    // 어떤 타입에 대한 trait를 구현하는 것은, 평범한 메소드를 구현하는 것과 비슷하다.
    // 다른점이라면 `impl` 뒤에 구현하고자 하는 trait 이름을 적고, 그다음 `for` 키워드와 trait을 구현할 타입명을 명시한다는 점이다
    // fn summarize(&self) -> String {
    //     // impl 블록 내부에는 trait 정의에서 정의된 메소드 시그니처(여기서는 summarize)를 집어넣되 세미콜론 대신 중괄호를 사용해서 메소드 본문에 원하는 특정 동작을 채워넣는다.
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Tweet 구조체에 대해 사용자명과 해당 트윗의 전체 텍스트를 가져오도록 하는 `summarize`를 구현
impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    // 이제 위의 summarize가 꼭 필요하지 않고, 기본 구현이 없는 summarize_author만 작성해주면 된다.
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
