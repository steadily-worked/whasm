use oop_blog::Post;

fn main() {
    상태패턴();
}

fn 상태패턴() {
    println!(
        "
    상태 패턴에서, 상태는 상태객체(state object)의 집합으로 표현되며 값의 동작은 상태에 기반하여 변경됨.
    상태 패턴을 사용하면, 프로그램의 비즈니스적 요구사항들이 변경될 때 상태를 보유한 값의 코드, 그 값을 사용하는 코드는 변경될 필요가 없다는 이점이 있음
    "
    );

    // let mut post = Post::new();

    // post.add_text("점심으로 샐러드 먹었음");
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content()); // 검토중일때 빈컨텐츠반환

    // post.approve();
    // assert_eq!("점심으로 샐러드 먹었음", post.content());
    // // blog 크레이트에 원하는 요구 동작을 보여주는 코드. Post 타입과만 상호작용하고있음.

    // 두번째
    // let mut post2 = Post::new();

    // post2.add_text("점심으로 샐러드 먹었음");
    // assert_eq!("", post2.content());

    // 마지막
    let mut post = Post::new();

    post.add_text("점심으로 샐러드 먹었음");

    let post = post.request_review();

    let post = post.approve();
    // 더이상 객체지향 상태패턴을 따르지 않지만(상태 간의 변환이 Post 구현체 내에 모두 캡슐화되지 않게 됨), 타입 시스템과 컴파일에 일어나는 타입검사로인해 유효하지 않은 상태가 사라지게 됨.
    // 즉, 객체지향 외에도 다른 패턴을 사용할 수 있다는 것.

    assert_eq!("점심으로 샐러드 먹었음", post.content());
}
