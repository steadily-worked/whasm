러스트에는 코드 조직화에 필요한 기능들이 여러가지가 있다. 이를 통틀어 모듈 시스템이라고 한다.

- 패키지: 크레이트 빌드, 테스트, 공유에 사용하는 Cargo 기능.
- 크레이트: 라이브러리나 실행 가능한 모듈로 구성된 트리 구조
- 모듈과 use: 구조, 스코프를 제어하고 조직 세부 경로를 감추는 데 사용.
- 경로: 구조체, 함수, 모듈 등의 이름을 지정.

## 크레이트(crate)

- 러스트가 컴파일 한 차례에 고려하는 가장 작은 코드 단위.
- 여러 모듈을 담을 수 있고, 모듈은 이 크레이트와 함께 컴파일되는 다른 파일들에 정의되어 있을 수도 있음.
- 라이브러리일 수도 있고, 바이너리일 수도 있음.
  - 라이브러리: React에서의 라이브러리처럼 필요한 기능을 가져다 쓰기 위한, 공용될 의도로 만들어진 기능들이 정의되어있음.
  - 바이너리: 우리가 지금까지 만든 모든 크레이트들. 실행 가능한 실행 파일로 컴파일할 수 있는 프로그램.
- 크레이트 루트: 러스트 컴파일러가 컴파일을 시작하는 소스 파일. 크레이트의 루트 모듈을 구성.

## 패키지

- 일련의 기능을 제공하는 하나 이상의 크레이트로 구성된 번들. `Cargo.toml` 파일이 포함되어있음.
- 패키지에는 바이너리 크레이트가 여러개 포함될 수 있으나, **라이브러리 크레이트는 하나만 넣을 수 있음.** -> 왜? `lib` 파일 하나만 가질 수 있다는 뜻인가?

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

- `Cargo.toml`이 패키지를 만들어 준다. `Cargo.toml` 내부에는 `src/main.rs`가 따로 적시되어 있지는 않지만, 패키지명과 같은 이름의 바이너리는 `src/main.rs`가 루트라는 관례를 준수한다.
- 마찬가지로 `src/lib.rs`는 라이브러리의 루트.

## 모듈 치트시트

- 크레이트를 컴파일할 때, 컴파일러는 우선 크레이트 루트 파일(`src/main.rs`(바이너리), `src/lib.rs`(라이브러리)) 파일을 본다.
- 크레이트 루트 파일(`src/main.rs`(바이너리), `src/lib.rs`(라이브러리))에는 새로운 모듈을 선언할 수 있다. 예를 들면 `mod garden;`의 형태로 garden 모듈을 선언할 수 있다.
- 크레이트 루트가 아닌 파일에서는 서브모듈(submodule)을 선언할 수 있다. 예를 들어 `src/garden.rs` 안에 `mod vegetables;`를 선언할 수 있다. 아래 장소들에서 서브모듈의 코드가 있는지 살펴볼 것이다.
  - `mod vegetables` 뒤 세미콜론이 아닌, 중괄호를 쓴 경우 그 내부
  - `src/garden/vegetables.rs` 파일 내부
  - `src/garden/vegetables/mod.rs` 파일 내부
- 모듈이 크레이트의 일부로서 구성되면, 공개 규칙이 허용하는 한도 내에서라면 해당 코드의 경로를 사용하여 동일한 크레이트의 어디에서든 이 모듈의 코드를 참조할 수 있음.
  - ex) garden vegetables 모듈 안에 있는 `Asparagus` 타입은, `crate::garden::vegetables::Asparagus`로 찾아 쓸 수 있음
    - 여기서, `crate::garden::vegetables::Asparagus`를 참조할 수 있는 모든 스코프에서 앞에 `use`를 붙여줘서 단축 경로를 만들 수 있고, 그 이후부터는 `Asparagus` 만 작성해주면 이 타입을 사용할 수 있음.
- 모듈 내부의 코드는 부모 모듈에게 비공개임. 공개로 만들려면 `mod` 대신 `pub mod`를 써서 선언하기.
  - 왜?

## 경로를 사용하여 모듈 트리의 아이템 참조하기

- 경로는 두가지 형태가 존재함
  - 절대 경로: 크레이트 루트로부터 시작되는 전체 경로;
    - 현재 크레이트로부터의 코드는 `crate` 리터럴로 시작
    - 외부 크레이터로부터의 코드는 해당 크레이트 이름으로 절대 경로가 시작
  - 상대 경로: 현재의 모듈을 시작점으로 함. `self`, `super` 혹은 현재 모듈 내의 식별자를 사용.
