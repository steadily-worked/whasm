fn main() {
    // 앞서 `takes_and_gives_back` 함수는 소유권을 함수에 이전했다가 다시 돌려받는 과정이 있다고 했다.
    // 근데 굳이 '소유권을 이전했다가 다시 돌려받는' 과정이 필요할까?
    // Rust에는 소유권을 이전하지 않고 값을 사용할 수 있는 기능인 참조(Reference)가 있다.

    // 포인터와 달리 참조는 해당 참조의 수명동안 특정 유형의 유효한 값을 가리키도록 보장된다.
    // 참조를 사용하려면 함수의 파라미터로 보내는 값의 prefix로 `&`를 넣으면 된다.
    // 참조를 생성하는 행위를 borrowing이라고 한다.
    let mut s1 = String::from("hello");
    change(&mut s1);
    // s1을 mutable한 값으로 변경하고, 함수에 변경 가능한 `&mut s1` 참조를 만들고, `change` 함수에서 변경이 가능함을 명시하는 `&mut String` 처리를 하면 된다.
    // 다만 이경우, 값에 대한 변경 가능한 참조가 있다면 해당 값에 대한 다른 참조를 가질 수가 없다.
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);
    // 이 경우 println 함수는 에러를 리턴한다.
    println!("{s1}");
    // 이러한 Rust의 형태는, 컴파일 시 데이터의 경합을 방지할 수 있다는 장점이 있다.
    // 데이터 경합(data race)의 3가지 조건:
    // 1. 두개 이상의 포인터가 동시에 동일한 데이터에 액세스하는 경우
    // 2. 최소 하나 이상의 포인터가 데이터를 write하는 데 쓰이고 있는 경우
    // 3. 데이터에 대한 액세스를 동기화하는 데 사용되는 매커니즘이 없는 경우
    // 데이터 경합이 발생하는 경우, 정의되지 않은 동작(undefined behavior)을 유발하고 런타임에 이를 추적하려고 할 때 문제를 진단하고 수정하기가 어려울 수 있다.
    // Rust에서는 이러한 데이터 경합이 있는 코드를 컴파일하지 않음으로써 이러한 문제들을 방지한다.

    // 아래와 같이 중괄호를 추가하여 특정 범위를 가진 스코프를 생성하면 에러가 발생하지 않는다.
    {
        let _r1 = &mut s1;
    }
    let _r2 = &mut s1;

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    {
        let s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        let r3 = &s; // &mut s로 작성할 경우 에러 발생(s가 mutable한 변수일 경우)
        println!("{}, {} and {}", r1, r2, r3);
        // 동일한 값에 대한 불변 참조(`&s`)가 있는 동안에는, 변경 가능한 참조(`&mut s`)를 가질 수가 없다.
        // 불변 참조를 사용하는 사용자는, 아래에서 갑자기 값이 바뀔 것을 당연히 기대하지 않는다. 그렇기에 에러가 발생하는 것.
        // 그렇지만 불변 참조를 여러개 작성하는 것은 문제가 되지 않는다. 데이터를 읽는 행위 자체는 다른 사람의 데이터 읽기에 영향을 줄 수가 없기 때문.
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // 이 지점 이후부터 r1과 r2는 더이상 사용되지 않는다. 불변 참조 r1과 r2의 범위가 끝난다.

        let r3 = &mut s;
        println!("{}", r3); // r1); // 여기서 다시 r1을 사용하게 되면 똑같은 에러가 발생한다.

        // 반면 이 코드들은 문제가 없다. 왜그럴까?
        // 그것은, 참조의 스코프는 참조가 도입된 곳(let mut s = ~)에서 시작하여 해당 참조가 마지막으로 사용된 시점까지 계속되기 때문이다.
        // 즉, 이 코드의 경우 변경 가능한 참조(`&mut s`)가 도입되기 이전에 불변 참조(`&s`)의 마지막 사용인 `println!`이 발생하므로 컴파일이 되는 것이다.
    }

    let _reference_to_nothing = dangle();
    // 포인터를 사용하는 언어에서는, 해당 메모리에 대한 포인터를 유지하면서 일부 메모리를 해제함으로써 "dangling pointer"(일부 메모리를 해제하여 다른 사람에게 제공되었을 수 있는 메모리의 위치를 참조하는 포인터)를 실수로 만들기 쉽다.
    // 반면 Rust에서는 컴파일러 참조가 dangling 참조가 되지 않도록 보장한다. 어떤 데이터에 대한 참조가 있는 경우 컴파일러는 데이터에 대한 참조가 범위를 벗어나기 이전에 해당 데이터가 범위를 벗어나지 않도록 보장한다.
}

fn calculate_length(s: &String) -> usize {
    // s는 String 타입에 대한 불변 참조이다.
    s.len()
    // s는 참조이기 때문에 `.push_str()` 등의, 값의 변경을 수반하는 메소드들을 사용할 수 없다.
} // 여기서, s는 스코프 밖으로 나오게 된다. 그렇지만 소유권이 없는(참조) 상태이기 때문에, drop되지는 않는다.

fn change(some_string: &mut String) {
    // 타입이 &String일 경우 `cannot borrow `*some_string` as mutable, as it is behind a `&` reference `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable`. 라는 에러 발생.
    // 요약하면 참조값이므로 mutable하지 않다는 것인데, 이 경우엔 &String을 &mut String으로 가변 참조 처리해주면 된다.
    // (물론 기본적으로 이 함수의 파라미터로 들어가는 `s1`의 값 또한 mutable해야 하며(`let mut s1 = ~`), 파라미터에도 &mut 선언을 해줘야 한다.)
    // 이 경우 `change` 함수가 빌려온 값을 변경한다는 것을 매우 명확히 알 수 있다.
    some_string.push_str(", world");
}

fn dangle() -> String {
    // &String 타입에서 "this function's return type contains a borrowed value, but there is no value for it to be borrowed from" 라는 에러가 발생하게 된다.
    // dangle 함수는 String에 대한 참조를 리턴하고,
    let s = String::from("hello"); // s는 새로운 String이다.

    // &s 그리고 여기서 s(String)에 대한 '참조'를 리턴하게 된다.
    s
} // 여기서 s는 스코프 밖으로 나오게 되고, drop된다. 그 결과 할당된 메모리가 사라지게 되며, 문제가 생긴다.
  // 이 문제를 해결하려면, 새로운 메모리를 함수에서 할당한 만큼 '참조'를 리턴하지 않고 본래 String값을 리턴하면 된다. 메모리가 할당된 상태로 리턴되므로 값을 사용할 수 있다.

// 참조에 대한 정리
// 1. 언제든지 하나의 가변 참조(`&mut ~`) 또는 여러개의 불변 참조(`&~`)를 가질 수 있다.
// 2. 참조는 항상 유효해야 한다. (스코프 밖으로 나갔을 때 메모리 할당이 삭제되면 안된다.)
