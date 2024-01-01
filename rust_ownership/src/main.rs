fn main() {
    // ownership: 소유권은 Rust의 가장 독특한 기능 가비지 컬렉터 없이도 메모리의 안전성을 보장하는 방식
    // 일부 언어에는 가비지 컬렉션이 있지만, 다른 언어에서는 프로그래머가 직접 메모리를 명시적으로 할당하고 해제해야 함
    // Rust에서는, 컴파일러가 검사하는 '일련의 규칙'이 있는 소유권 시스템으로 메모리를 관리하며, 하나라도 위반될 경우 컴파일되지 않음

    // 소유권에 관한 규칙
    // 1. Rust에서 각 value는 "owner"라고 칭하는, 소유자가 있다.
    // 2. 한 번에 한 명의 소유자만 있을 수 있다.
    // 3. 소유자가 범위를 벗어나면 해당 값은 삭제된다.
    {
        // s는 아직 선언되지 않았으므로 이 line에선 유효하지 않다.
        let _s = String::from("hello"); // 이 시점부터 s는 유효하다.
    } // 스코프가 여기서 끝나며, s는 더이상 유효하지 않다.
      // 즉, 스코프 내에 들어왔을 때부터 스코프 밖으로 나가기 전까지 유효하다.

    // String 타입을 사용할 경우: 변경 가능하고 크기가 커질 수 있는 텍스트를 지원하기 위해 컴파일 시점에 알 수 없는 양의 메모리를 힙에 할당해서 내용을 저장해야 한다.
    // -> 1. 런타임에 메모리 할당자에게 메모리가 요청되어야 하며, 2. 문자열 작업이 끝난 후 이 메모리를 할당자에게 반환하는 방법이 필요하다.
    // 위의 예시에서, 스코프를 벗어났을 때가 '2번이 실행될 수 있는 자연스러운 시점'이 된다. 이 시점에 Rust는 drop이라는, 메모리를 반환하는 특별한 함수를 호출한다.
    {
        let x = 5;
        let _y = x;
        // 정수의 경우 크기가 알려진 고정된 단순한 값이고, x와 y 각각의 스택에 '5'의 값이 푸시되므로 두개의 값은 모두 5를 가리키게 된다.
    }
    {
        let s1 = String::from("hello");
        let _s2 = s1;
        // 이 경우 String data가 카피돼서, s1 스택에 있는 pointer, length, capacity가 s2의 스택에 복사된다.
        // 힙 데이터는 복사하지 않아서, 두개의 변수가 모두 하나의 힙 데이터를 가리킴.
        // 앞서 말한 drop 함수가 s1, s2에 대해 실행되는 경우 두개가 하나의 힙 데이터를 가리키므로 메모리를 두번 해제하려는 시도가 발생한다. 이 경우 보안 취약점이 발생할 수 있다.
        // 메모리의 안정성을 보장하기 위해 `s2 = s1` 줄 다음에 `s1`이 더이상 유효하지 않은 것으로 간주한다.

        // println!("{s1}");
        // borrow of moved value: `s1`: value borrowed here after move.
        // 이 println! 함수는 에러를 발생시킨다. 메모리에 대한 소유권이 s2에게로 넘어갔기 때문이다.
        // 얕은 복사와 비슷해 보이지만, Rust에서는 's1' 자체를 무효화 해버렸기 때문에 '얕은 복사'라기 보다는 'move(이동)'라고 부른다.
    }
    // 힙 데이터까지 복사(deep copy)하고싶은 경우 `clone` 메소드를 사용한다.
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
        // 이 경우 x는 왜 값을 갖고있을까? 그 이유는 정수처럼 '컴파일 시점에 크기가 알려진 타입'은 스택에 완전히 저장되므로 실제 값의 복사본을 빠르게 만들 수 있기 때문이다.
        // 이 경우 deep copy와 shallow copy의 차이가 없다.
    }
    // let y = x;
    // 위에서 언급한 Drop 함수(스코프를 벗어났을 때)가 실행됐을 때 복사를 하려고 하면 컴파일 에러가 발생한다.
    // cannot find value `x` in this scope. not found in this scope;

    // Copy가 가능한 케이스
    // 1. 모든 정수 타입 (ex: u32)
    // 2. bool
    // 3. 모든 부동소수점 타입 (ex: f64)
    // 4. 문자 타입 (char)
    // 5. 튜플(Copy를 구현하는 타입들만 포함되어있는 튜플). ex) (i32, i32) ok, (i32, String) x

    {
        let s = String::from("hello");
        takes_ownership(s); // s의 값이 takes_ownership 함수 내부로 이동하게 된다. 그리하여 더이상 여기서는 유효하지 않다.
                            // 여기서부터 s는 유효하지 않은 상태이므로, s를 사용하려고 하면 컴파일 에러가 발생한다.

        let x: i32 = 5;
        makes_copy(x) // x가 makes_copy 함수로 이동하게 되지만, i32는 Copy가 가능하기에 x는 여전히 유효하다.
    }

    {
        let _s1 = gives_ownership(); // `gives_ownership()` 함수는 리턴값의 소유권을 `s1`으로 이전시킨다.
        let s2 = String::from("hello"); // `s2`가 스코프에 들어온다.
        let _s3 = takes_and_gives_back(s2); // `s2`가 `takes_and_gives_back` 함수로 이동하게 되며, 이 함수의 리턴값이 `s3`으로 소유권이 이전된다.
                                            // 즉 이 함수는 소유권을 함수로 이전했다가 다시 돌려주는 함수인 것이다.
    }
}

fn takes_ownership(some_string: String) {
    // some_string이 이 스코프로 들어오며
    println!("{}", some_string);
} // 여기서 some_string이 스코프로부터 빠져나가고 `drop` 함수가 실행된다. 메모리가 해제된다.

fn makes_copy(some_integer: i32) {
    // some_integer가 스코프로 들어오며
    println!("{}", some_integer);
} // 여기서 some_integer이 스코프로부터 빠져나가지만 특별한 일이 발생하진 않는다.

fn gives_ownership() -> String {
    // 이 함수는, return value를 함수 내부로 옮기고 실행시킨다.
    let some_string = String::from("yours"); // `some_string`이 스코프에 들어온다.
    some_string // `some_string`이 리턴되며 이 함수를 호출한 함수로 빠져나가게 된다.
}

// String을 파라미터로 받아와서 하나를 반환한다.
fn takes_and_gives_back(a_string: String) -> String {
    // `a_string`이 스코프에 들어온다.
    a_string // a_string이 리턴되며, 이 함수를 호출한 함수로 빠져나가게 된다.
}
