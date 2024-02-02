## 제네릭 타입, 트레이트(trait), 라이프타임

### 1. 함수를 추출하여 중복 없애기

제네릭은, 여러가지 타입을 나타내는 자리표시자의 위치에 특정 타입을 집어넣는 것만으로 코드 중복을 제거할 수 있게 해준다. 제네릭을 사용하면 함수 시그니처나 구조체의 아이템에 다양한 구체적 데이터 타입을 사용할 수 있도록 정의할 수 있다.

#### 제네릭 함수 정의

함수 시그니처 내 파라미터와 반환 값의 데이터 타입 위치에 제네릭을 사용한다. 이렇게 작성된 코드는 더 유연해지고, 이 함수를 호출하는 쪽에서 더 많은 기능을 사용할 수 있도록 하며 중복 또한 방지한다.

```rs
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

이 두 함수의 본문은 완벽하게 동일하므로, 제네릭을 이용해 이 두 함수를 하나로 만들어서 코드 중복을 제거할 수 있다. 함수 본문에서 파라미터를 사용하려면 함수 시그니처에 파라미터의 이름을 선언하여 컴파일러에게 해당 이름이 무엇을 의미하는지 알려줘야 하는 것처럼, 타입 파라미터를 사용하기 전에도 타입 파라미터의 이름을 선언해야 한다. 예를들어 제네릭 `largest` 함수를 정의하려면 아래와 같이 함수명과 파라미터 목록 사이의 꺾쇠괄호(`<>`)에 타입 파라미터 이름을 선언해야 한다.

```rs
fn largest<T>(list: &[T]) -> &T {...}
```

이 정의는, **`largest` 함수는 어떤 타입 `T`에 대한 제네릭 함수**라고 읽힌다. 이 함수는 `T` 타입 값의 슬라이스인 `list` 파라미터를 갖고 있고, 동일한 `T` 타입의 값에 대한 참조자를 반환한다.

```rs
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
