fn main() {
    // if let: if와 let을 조합하여 하나의 패턴만 매칭시키고 나머지 경우는 무시하도록 값을 처리하는 간결한 방법을 제공한다.
    // config_max 변수의 어떤 Option<u8> 값을 매칭하지만 그 값이 Some variant일 경우에만 코드를 실행시키고 싶어하는 함수
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // 하지만 이 함수는 결국, 이 값이 Some인지 아닌지만 판단하면 되는 건데 이를 위해 match 패턴을 사용하는 것은 불필요해보인다.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // 이런 식으로 `if let`을 사용하여 Some 값이 config_max인지 판단하고 맞을 경우 아래 코드를 진행시키는 형태로 리팩토링할 수 있다.
    // if let은 match와 동일한 방식으로 작동한다. 위 if let 구문의 경우 패턴이 Some(max)이고 max는 Some 내에 있는 값에 바인딩된다.
    // 이렇게 될 경우 `match`의 갈래 안에서 `max`를 사용했던 것과 같은 방식으로 `if let` 본문 블록 내에서 `max`를 사용할 수 있다.
    // if let을 사용하게 되면 타이핑과 들여쓰기, 보일러플레이트 코드가 줄게 되지만 match가 강제하는 철저한 검사는 하지 않게 된다.
    // 상황에 따라 적절한 것을 사용하면 된다. if let은 한 패턴에 매칭될 때만 코드를 실행하는 경우에만 match 대신 사용할 수 있는 좋은 방법이다.
}

// 열거형 값에 데이터가 있는 경우, 처리해야 하는 경우의 수에 따라 match나 if let을 사용하여 해당 값을 추출하여 사용할 수 있다.
