## 에러 처리

러스트는 에러를 복구 가능한(recoverable) 에러와 복구 불가능한(unrecoverable) 에러의 두가지 범주로 묶는다.

- 복구 가능한 에러: 사용자에게 문제를 보고하고 명령을 재시도하길 원함 (ex: `파일을 찾을 수 없음`)
- 복구 불가능한 에러: 프로그램을 즉시 멈춤 (ex: `배열 끝을 넘어선 인덱스에 접근하는 경우`)

러스트에는 예외 처리 기능은 없지만, 복구 가능한 에러를 위한 `Result<T, E>` 타입과 복구 불가능한 에러가 발생했을 때 프로그램을 종료하는 `panic!` 매크로가 있다.

### 1. `panic!`으로 복구 불가능한 에러 처리하기

Rust에서 패닉을 일으키는 방법은 두 가지가 있다:

1. 코드가 패닉을 일으킬 동작들을 하는 것
2. `panic!` 매크로를 명시적으로 호출하는 것 (자바스크립트의 `throw new Error`가 생각난다.)

`panic!`이 발생하면 프로그램은 되감기(unwinding)를 시작하는데, 이는 Rust가 패닉을 발생시킨 함수로부터 스택을 거꾸로 훑어가면서 데이터를 청소한다는 뜻이다. 하지만 이 작업이 간단한 작업이 아니기 때문에, Rust에서는 프로그램이 데이터 정리 작업 없이 즉각 종료되는 대안인 그만두기(aborting)를 선택할 수도 있다. 프로그램이 사용하던 메모리는 운영체제가 청소해 줘야 한다. `Cargo.toml` 내의 적합한 `[profile]` 섹션에 아래와 같이 입력함으로써 되감기를 그만두기로 바꿀 수 있다.

```toml
[profile.release]
panic = 'abort'
```

### 2. `Result`로 복구 가능한 에러 처리하기

```rs
enum Result<T, E> {
    OK(T),
    Err(E)
}
```

```rs
let _greeting_file_result = File::open("hello.txt");
```

파일을 여는 코드. `File::open`의 반환 타입은 `Result<T, E>`이다. 제네릭 매개변수 T는 `File::open`의 구현부에 성공 값인 파일 핸들 `std::fs::File`로 채워져 있다. 에러 값에 사용된 E의 타입은 `std::io::Error`이다. 이 `File::open` 함수는 예를 들어 해당 파일이 존재하지 않는 경우에는 실패할 수도 있다. 이렇게, 가능성을 알려주면서도 파일 핸들 혹은 에러 정보를 제공할 방법이 필요한데, 이 일을 `Result` 이넘이 한다.

`File::open`이 성공하는 경우에는 `greeting_file_result` 변수의 값이 파일 핸들을 갖고 있는 `Ok` 인스턴스가 될 것이다. 실패한 경우 `greeting_file_result`는 발생한 에러의 종류에 관한 더 자세한 정보가 담긴 `Err` 인스턴스가 될 것이다. 이러한 핸들링을 `match` 메소드를 사용해 추가해보면 아래와 같을 것이다:

```rs
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}",error)
    };
}
```

...후략

### 3. 에러 발생 시 패닉을 위한 shortcut: `unwrap`과 `expect`

`unwrap` 메소드는 `match` 구문과 비슷한 구현을 하는 shortcut 메소드이다. 만일 Result 값이 Ok라면 `unwrap`은 `Ok`의 값을 반환할 것이고, Err 라면 `unwrap`은 `panic!` 매크로를 호출해 줄 것이다.

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

`hello.txt`가 없는 상태에서 위 코드를 실행시키면 `unwrap` 메소드에 의해 호출된 `panic!` 으로부터 아래와 같은 메시지를 보게 될 것이다.

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

이와 비슷한 `expect`는 `panic!` 에러 메시지도 선택할 수 있도록 해준다. `unwrap` 대신 `expect`를 이용하고 좋은 에러 메시지를 제공하면 개발자의 의도를 전달하면서 패닉의 근원을 추적하는 것을 쉽게 해준다.

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

`unwrap`은 `panic!`의 기본 메시지가 출력되지만 `expect`는 매개변수로 전달한 여러 메시지를 출력한다. 다음과 같은 형태로 나타난다.

```text
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

### 4. 에러 전파하기

함수의 구현체에서 실패할 수도 있는 무언가를 호출할 때, 이 함수에서 에러를 처리하는 대신 이 함수를 호출하는 코드 쪽으로 에러를 반환하여 그쪽에서 수행할 작업을 결정하도록 할 수 있다. 이를 에러 전파하기(`propagating`)라고 하며 호출하는 코드 쪽에 더 많은 제어권을 주는 것인데, 호출하는 코드 쪽에는 에러를 어떻게 처리해야 하는지 결정하는 정보와 로직이 코드 컨텍스트 내에서 활용할 수 있는 것보다 더 많이 있을 수도 있기 때문이다.

### `panic!`이냐 `panic!`이 아니냐, 그것이 문제로다

언제 `panic!`을 쓰고 언제 `Result`를 반환해야 할지는 어떻게 결정할 수 있을까? 코드가 패닉을 일으킬 때는 복구할 방법이 없다.

1. `panic!`을 에러 상황에 대해 호출할 수 있지만, 그렇게 되면 호출하는 코드를 대신해서 **현 상황은 복구 불가능한 것**이라고 결정을 내리는 꼴이 된다.
2. `Result` 값을 반환하는 선택을 한다면, 호출하는 쪽에게 옵션을 제공하는 것이다. 호출하는 코드 쪽에서는 상황에 적합한 방식으로 복구를 시도할 수도 있고, 혹은 현재 상황의 `Err`은 복구 불가능하다고 결론을 내리고 `panic!`을 호출하여 복구 가능한 에러를 복구 불가능한 것으로 바꿔놓을 수도 있다.

그러므로 실패할 지도 모르는 함수를 정의할 때는 기본적으로 `Result`를 반환하는 것이 좋은 선택이다.

그러나 **예제, 프로토타입, 테스트**같은 상황에서는 `Result` 보다 패닉을 일으키는 코드가 더 적절하다. 왜인지에 대해서 알아보자.

1. 어떤 개념을 묘사하기 위한 예제를 작성 중이라면, 견고한 에러 처리 코드를 포함시키는 것이 오히려 예제의 명확성을 떨어트릴 수도 있다. 예제 코드 내에서는 `panic!`을 일으킬 수 있는 `unwrap`같은 메소드의 호출이 어플리케이션의 에러 처리가 필요한 곳을 뜻하는 방식으로 해석될 수 있는데, 이러한 에러 처리는 코드의 나머지 부분이 하는 일에 따라 달라질 수 있다.

2. 비슷한 상황으로 에러를 어떻게 처리할지 결정할 준비가 되기 전이라면, `unwrap`과 `expect` 메소드가 프로토타이핑할 때 매우 편리하다. 이 함수들은 코드를 더 견고하게 만들 준비가 되었을 때를 위해서 명확한 표시를 남겨 둔다. 만일 테스트 내에서 메소드 호출이 실패한다면, 해당 메소드가 테스트중인 기능이 아니더라도 전체 테스트를 실패시키도록 한다. `panic!`이 테스트의 실패를 표시하는 방식이므로, `unwrap`이나 `expect`의 호출이 정확히 그렇게 만들어준다.

3. `Result`가 `Ok` 값을 갖고 있을 것이라 확신할 만한 논리적 근거가 있지만 컴파일러가 그 논리를 이해할 수 없는 경우라면 `unwrap` 혹은 `expect`를 호출하는 것이 적절할 수 있다. 어떤 연산이든 간에 특정한 상황에서는 논리적으로 불가능할지라도 기본적으로는 실패할 가능성을 갖고 있는 코드를 호출하는 것이므로, 처리가 필요한 `Result` 값이 나오게 된다. 손수 코드를 조사하여 `Err` 배리언트가 나올 리가 없음을 확신할 수 있다면 `unwrap`을 호출해도 아무런 문제가 없으며, `expect`의 문구에 `Err` 배리언트가 있으면 안될 이유를 적어주는 것이 더 좋을 것이다.

```rs
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

**`"127.0.0.1"`이 유효한 문자열이라는 사실**이 `parse`메소드의 반환 타입을 변경해 주진 않는다: 여전히 `Result` 값이 나오고, 컴파일러는 마치 `Err` 배리언트가 나올 가능성이 여전히 있는 것처럼 `Result`를 처리하도록 요청할 것인데, 그 이유는 이 문자열이 항상 유효한 IP 주소라는 사실을 알 수 있을 만큼 컴파일러가 똑똑하지 않기 때문이다.

심지어 하드코딩된 값에 대해서도 그런데, 만약 이 IP 주소가 사용자로부터 입력된 값이라면? 실패할 가능성이 생기게 된다. 그렇다면 더 견고한 방식으로 `Result` 를 처리할 필요가 분명히 있다. `expect`에 이 IP 주소가 하드코딩 되었다는 가정을 언급하는 것은, 향후에 IP 주소가 다른 곳으로부터 가져올 필요가 생길 경우 `expect`를 더 나은 에러 처리 코드로 수정하도록 재촉할 것이다.

코드가 유효하지 않은 값에 대해 호출되면 사용자를 위험에 빠뜨릴 수 있는 연산을 수행할 때, 그 코드는 해당 값이 유효한 지를 먼저 검사하고 만일 그렇지 않다면 `panic!`을 호출해야 한다. 이는 보안상의 이유이다: 유효하지 않은 데이터에 어떤 연산을 시도하는 것은 코드를 취약점에 노출시킬 수 있다. 이것이 범위를 벗어난 메모리 접근을 시도했을 경우 표준 라이브러리가 `panic!`을 호출하는 주된 이유이다.

### 정리

Rust의 에러 처리 기능은 더 견고한 코드를 작성하는 데 도움을 주도록 설계되었다. `panic!` 매크로는 프로그램이 처리 불가능한 상태에 놓여 있음에 대한 신호를 주고, 유효하지 않거나 잘못된 값으로 계속 진행을 시도하는 대신 실행을 멈추게끔 해준다. `Result` 열거형은 Rust의 타입 시스템을 이용하여 복구할 수 있는 방법으로 코드의 연산이 실패할 수도 있음을 알려준다. 또한 `Result`를 이용하면 **본인이 작성한 코드를 호출하는 코드에게 잠재적인 성공이나 실패를 처리해야 할 필요가 있음**을 알려줄 수 있다. `panic!`과 `Result`를 적절한 상황에서 사용하는 것은 코드가 불가피한 문제에 직면했을 때도 더 신뢰할 수 있도록 해줄 것이다.
