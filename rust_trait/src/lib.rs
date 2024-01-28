// 공개 trait 선언
pub trait Summary {
    fn summarize(&self) -> String;
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
    fn summarize(&self) -> String {
        // impl 블록 내부에는 trait 정의에서 정의된 메소드 시그니처(여기서는 summarize)를 집어넣되 세미콜론 대신 중괄호를 사용해서 메소드 본문에 원하는 특정 동작을 채워넣는다.
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
