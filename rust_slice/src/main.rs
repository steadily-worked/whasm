// Slice Type
fn main() {
    let mut s: String = String::from("hello world");
    // let _word = first_word(&s);
    let word_with_slice = first_word_with_slice(&s); // slice를 사용해서 변수를 만들 경우, 9행의 `s.clear()`는 에러를 발생시킨다.
                                                     // word_with_slice의 값은 s를 슬라이스한 값이므로 문자열과 관련이 있는 값이 된다.
                                                     //  그렇기 때문에 `s.clear()`를 시도할 경우 에러가 발생하는 것이다.
                                                     // 에러는, `clear` 함수는 문자열을 잘라내야 되므로 mutable한 참조를 가져와야 하는데, clear 뒤의 println!에서 여전히 단어의 참조를 사용하므로 그 시점에서 immutable한 참조가 활성 상태여야 하는 것과 관련이 있다.
                                                     // Rust는 `clear` 함수의 mutable한 참조와 word_with_slice의 immutable한 참조가 동시에 존재하는 것을 허용하지 않으므로 컴파일에 실패한다.
    s.clear(); // String을 ""로 만든다.
               // 이 경우, _word는 여전히 5라는 값을 갖고 있지만 정작 '5'라는 값을 의미있게 사용할 수 있는 문자열이 더이상 없다.
               // 'hello world'의 값을 가진 &s를 기반으로 5라는 값을 가진 _word를 생성했는데, 그 이후 'hello world'라는 값을 더이상 `s`가 갖지 않게 되었으므로 이것은 버그가 된다.
    println!("the first word is: {}", word_with_slice);

    // 이러한 문제를 해결하는 방법으로 slice가 있다.
    {
        let s = String::from("hello world");

        let _hello = &s[0..5]; // _hello는 문자열 전체에 대한 참조가 아니라 [0..5] 비트에 대한 참조이다.
        let _world = &s[6..11];

        // slice는 [starting_index..ending_index]의 형태를 취한다.
        let _slice = &s[0..2];
        let _slice = &s[..2];
        // 이 두개는 같은 값이다.

        // 같은 맥락으로, ending_index에 대해서도 마지막 값까지 가는 경우 생략이 된다.
        let _slice = &s[3..s.len()];
        let _slice = &s[3..];
        // 이 두개는 같은 값이다.

        // 두개를 모두 생략할 경우, 모든 값이 된다.
        let _slice = &s[0..s.len()];
        let _slice = &s[..];
        // 이 두개는 같은 값이다.
    }
}

// first_word 함수: 공백으로 구분된 단어의 문자열을 받아, 해당 문자열에서 찾은 첫번째 단어를 반환하는 함수. 함수가 문자열에서 공백을 찾지 못하면, 전체 문자열이 한 단어여야 하므로 전체 문자열을 반환해야 함.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // 문자열을 바이트 배열로 반환

    for (i, &item) in bytes.iter().enumerate() {
        // iter: 컬렉션의 각 element를 반환, enumerate: 각 element를 튜플의 일부로 반환
        if item == b' ' {
            return i;
            // 공백을 나타내는 바이트를 찾으면 그 위치를 반환하고,
        }
    }

    s.len() // 그렇지 않을 경우(공백 없는 문자일 경우) 문자열의 길이를 반환한다.
}

// 그렇지만 위 first_word 함수에서 반환하는 usize의 경우, &String의 컨텍스트에서만 의미가 있는 숫자일 뿐이다.
// 즉, 문자열과는 별개의 값이기 때문에 향후에도 여전히 유효하다는 보장이 없다. (이후 4행의 주석을 보기)

// slice를 사용해서 다시 작성한 first_word 함수. slice는 &str의 타입을 가짐.
fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
