fn main() {
    // 앞서 `takes_and_gives_back` 함수는 소유권을 함수에 이전했다가 다시 돌려받는 과정이 있다고 했다.
    // 근데 굳이 '소유권을 이전했다가 다시 돌려받는' 과정이 필요할까?
    // Rust에는 소유권을 이전하지 않고 값을 사용할 수 있는 기능인 참조(Reference)가 있다.

    // 포인터와 달리 참조는 해당 참조의 수명동안 특정 유형의 유효한 값을 가리키도록 보장된다.
    // 참조를 사용하려면 함수의 파라미터로 보내는 값의 prefix로 `&`를 넣으면 된다.
    // 참조를 생성하는 행위를 borrowing이라고 한다.
    let mut s1 = String::from("hello");
    change(&mut s1);
    // s1를 mutable한 값으로 변경하고, 함수에 변경 가능한 `&mut s1` 참조를 만들고, `change` 함수에서 변경이 가능함을 명시하는 `&mut String 처리를 하면 된다.
    // 다만 이경우, 값에 대한 변경 가능한 참조가 있다면 해당 값에 대한 다른 참조를 가질 수가 없다.
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);
    // 이 경우 println 함수는 에러를 리턴한다.

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s는 String 타입에 대한 참조이다.
    s.len()
    // s는 참조이기 때문에 `.push_str()` 등의, 값의 변경을 수반하는 메소드들을 사용할 수 없다.
} // 여기서, s는 스코프 밖으로 나오게 된다. 그렇지만 소유권이 없는(참조) 상태이기 때문에, drop되지는 않는다.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
