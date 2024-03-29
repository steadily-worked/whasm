### 외부 패키지 사용하기

-`rand` 패키지를 사용하기 위해 `Cargo.toml`에 다음과 같이 추가했었다.

```
rand = "0.8.5"
```

- `Cargo.toml`에 `rand`를 의존성으로 추가하면, Cargo가 `crates.io`에서 `rand` 패키지를 비롯한 모든 의존성을 다운로드하고, 프로젝트에서 `rand` 패키지를 사용할 수 있게 된다. 아래는 이전에 '임의의 숫자 생성하기'에서 `rand` 패키지를 호출해서 사용했던 예시이다.

```rs
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

- 알아둬야 할 것이 있다면, `std` 표준 라이브러리도 마찬가지로 외부 크레이트라는 것이다. Rust 언어에 포함되어 있으므로 `Cargo.toml`에 추가할 필요는 없지만, 표준 라이브러리에서 우리가 만든 패키지의 스코프로 가져오려면 `use` 문을 작성해야 한다. 예를 들어 `HashMap`을 가져오는 코드는 아래와 같다.

```rs
use std::collections::HashMap;
```

### 중첩 경로를 사용하여 대량의 `use`를 나열하기

```rs
// --생략--
use std::cmp::Ordering;
use std::io;
// --생략--
```

위의 경우 `std` 라이브러리에서 가져오는 것인데, 이렇게 아이템당 한줄씩 코드를 나열하면 수직 방향으로 너무 많은 영역을 차지한다.

```rs
// --생략--
use std::{cmp::Ordering, io};
// --생략--
```

위와 같은 형태로 하나의 라이브러리에서 구조분해 할당을 하는 것처럼 가져와서 사용할 수 있다.

```rs
use std::io;
use std::io::Write;
```

이런 형태로 가져와야 하는 경우, 아래와 같이 작성해주면 된다. 스스로를 넣기 위해서는 `self`로 가져오면 된다.

```rs
use std::io::{self, Write};
```

### 글롭 연산자

경로에 글롭(glob) 연산자 `*`를 붙이면, 경로 안에 정의된 모든 공개 아이템을 가져올 수 있다.

```rs
use std::collections::*;
```

이 `use` 구문은, `std::collections` 내에 정의된 모든 공개 아이템들을 현재 스코프로 가져온다.

하지만 글롭 연산자는 코드에 사용된 어떤 이름이 어느 곳에 정의되어 있는지 파악하기 어렵게 만들 수 있으므로, 사용에 주의해야 한다.
