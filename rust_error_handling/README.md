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
