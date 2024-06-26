fn main() {
    // 제네릭: concrete 타입 혹은, 기타 속성에 대한 추상화된 대역(?)이다.
    // 컴파일 및 실행 시점에, 제네릭들이 실제로 무슨 타입으로 채워지는지 알 필요가 없이 제네릭의 동작이나 다른 제네릭과의 관계를 표현할 수 있다.
    // 이미 이전에 Option<T>, Vec<T>, HashMap<K ,V>, Result<T, E> 등을 봐봤다.
    함수_추출하여_중복_없애기();
    제네릭_데이터_타입();
    제네릭_구조체_정의();
    제네릭_열거형_정의();
    제네릭_코드의_성능();
}

fn 함수_추출하여_중복_없애기() {
    let 숫자_리스트 = vec![34, 50, 25, 100, 65];
    let mut 제일큰값 = &숫자_리스트[0];
    for 숫자 in &숫자_리스트 {
        if 숫자 > 제일큰값 {
            제일큰값 = 숫자;
        }
    }
    println!("제일 큰 값은 {}", 제일큰값);

    let 숫자_리스트 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut 제일큰값 = &숫자_리스트[0];
    for 숫자 in &숫자_리스트 {
        if 숫자 > 제일큰값 {
            제일큰값 = 숫자;
        }
    }
    println!("제일 큰 값은 {}", 제일큰값);
    // 위 로직은, 아래의 로직으로 단순화할 수 있음.

    fn 제일큰값_찾기(리스트: &Vec<i32>) -> &i32 {
        let mut 제일큰값 = &리스트[0];

        for 숫자 in 리스트 {
            if 숫자 > 제일큰값 {
                제일큰값 = 숫자;
            }
        }
        제일큰값
    }

    let 숫자_리스트 = vec![34, 50, 25, 100, 65];
    let 결과값 = 제일큰값_찾기(&숫자_리스트);
    println!("제일 큰 값은 {}", 결과값);
    let 숫자_리스트 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let 결과값 = 제일큰값_찾기(&숫자_리스트);
    println!("제일 큰 값은 {}", 결과값);

    // 중복된 코드 식별 -> 중복된 코드를 함수의 본문으로 분리 후, 함수의 시그니처 내에 해당 코드의 입력값 및 반환값 명시 -> 중복됐던 두 지점의 코드를 함수 호출로 변경
    // 위의 과정을 제네릭을 통해 해보자. '제일큰값_찾기' 함수의 본문이 추상화된 '리스트'로 동작하는 것처럼 제네릭도 그렇다.
}

fn 제네릭_데이터_타입() {
    // 제네릭을 사용하면 함수 시그니처나 struct의 아이템에 구체적인 데이터 타입을 사용할 수 있도록 정의할 수 있다.
    // fn 가장큰_i32(리스트: &[i32]) -> &i32 {
    //     let mut 제일큰값 = &리스트[0];

    //     for 아이템 in 리스트 {
    //         if 아이템 > 제일큰값 {
    //             제일큰값 = 아이템;
    //         }
    //     }

    //     제일큰값
    // }

    // fn 가장큰_char(리스트: &[char]) -> &char {
    //     let mut 제일큰값 = &리스트[0];

    //     for 아이템 in 리스트 {
    //         if 아이템 > 제일큰값 {
    //             제일큰값 = 아이템;
    //         }
    //     }

    //     제일큰값
    // }

    fn 가장큰거_찾기<T: std::cmp::PartialOrd>(리스트: &[T]) -> &T {
        let mut 제일큰값 = &리스트[0];

        for 아이템 in 리스트 {
            if 아이템 > 제일큰값 {
                // '가장큰거_찾기'의 본문이 T가 될 수 있는 모든 타입에 대해 동작할 수 없음
                // `std::cmp::PartialOrd` 트레이트를 T의 타입으로서 지정하면 해결 -> 표준 라이브러리가 i32, char 둘 모두에 대한 PartialOrd를 구현하고 있기 때문.
                제일큰값 = 아이템;
            }
        }

        제일큰값
    }

    let 숫자_리스트 = vec![34, 50, 25, 100, 65];
    let 결과값 = 가장큰거_찾기(&숫자_리스트);
    println!("제일 큰 값은 {}", 결과값);

    let 문자_리스트 = vec!['y', 'm', 'a', 'q'];
    let 결과값 = 가장큰거_찾기(&문자_리스트);
    println!("제일 큰 값은 {}", 결과값);
}

fn 제네릭_구조체_정의() {
    struct 포인트<T> {
        x: T,
        y: T,
    }

    let _정수_포인트 = 포인트 { x: 5, y: 10 };
    let _실수_포인트 = 포인트 { x: 1.0, y: 4.0 };
    // let 혼합_포인트 = 포인트 { x: 5, y: 4.0 }; // 에러: x와 y의 타입이 일치하지 않음

    // 제네릭을 사용하면, 구조체의 필드들이 동일한 타입을 가져야 한다.
    // 제네릭을 사용하지 않으면, 구조체의 필드들이 서로 다른 타입을 가질 수 있다.

    struct 포인트_서로다른타입<T, U> {
        x: T,
        y: U,
    }
    // 이처럼 두 타입을 가진 제네릭을 사용한다면, x와 y의 타입이 서로 다른 포인트를 생성할 수 있다. 그렇지만, 타입이 많아질수록 가독성은 떨어진다.

    let _둘다_정수 = 포인트_서로다른타입 { x: 5, y: 10 };
    let _둘다_실수 = 포인트_서로다른타입 { x: 1.0, y: 4.0 };
    let _정수와_실수 = 포인트_서로다른타입 { x: 5, y: 4.0 };
}

fn 제네릭_열거형_정의() {
    // enum 또한 variant에 제네릭 데이터 타입을 갖도록 할 수 있다. Option<T>를 다시 보자.
    enum Option<T> {
        Some(T),
        None,
    }
    // Option<T> enum을 사용함으로써 옵션 값에 대한 추상화된 개념을 표현할 수 있고, Option<T> enum이 제네릭으로 되어있는 덕분에 옵션 값이 어떤 타입이건 상관없이 추상화하여 사용할 수 있다.

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    // Result<T, E> enum은 Ok와 Err 두 가지 variant를 가지며, 각각은 T와 E 타입의 데이터를 갖는다.
    // 제네릭으로 정의되어 있는 덕분에, 연산이 성공할지(-> T 타입 반환) 실패할지(-> E 타입 반환) 알 수 없는 어느 곳이든 `Result` 열거형을 편하게 사용할 수 있다.
    // 지난주에 파일을 열 때 Result를 사용했었는데, 그 때에 성공했을 경우 T는 `std::fs::File` 타입이 되고, 문제가 생기면 E는 `std::io::Error` 타입이 되었었다.

    struct 포인트<T> {
        x: T,
        y: T,
    }

    impl<T> 포인트<T> {
        // 둘중 하나만 제네릭을 요구하는 경우가 있음.
        // impl 바로뒤에 T를 선언하여 포인트<T> 타입에 메소드를 구현한다고 명시했음에 주의하자.
        // 이렇게 하면 러스트는 포인트의 꺾쇠괄호 내 타입이 구체적 타입이 아닌, 제네릭 타입임을 인지한다. 보통은 구조체 정의에 선언된 제네릭 파라미터와 같은 이름의 파라미터를 사용하는 게 관례이다.
        fn x값(&self) -> &T {
            &self.x
        }
        fn y값(&self) -> &T {
            &self.y
        }
    }

    // 제네릭 타입에 대한 제약을 지정할 수도 있음. 포인트<f32> 타입에 대해서만 아래 메소드를 호출할 수 있음
    impl 포인트<f32> {
        fn 거리_계산(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
            // 원점 (0.0, 0.0)으로부터의 거리를 리턴하는 함수
        }
    }

    let 샘플_포인트 = 포인트 { x: 5.0, y: 10.0 };
    // let 샘플_포인트_두번째 = 포인트 { x: 5, y: 10 };
    println!("샘플 포인트의 x 값: {}", 샘플_포인트.x값()); // 5
    println!("샘플 포인트의 거리 계산: {}", 샘플_포인트.거리_계산());
    // println!(
    //     "샘플_포인트_두번째의 거리 계산: {}",
    //     샘플_포인트_두번째.거리_계산()
    // );

    // 매개변수의 제네릭 타입과 구조체에서 사용한 제네릭 타입 매개변수가 다를 수 있음.
    struct 포인트_2<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> 포인트_2<X1, Y1> {
        fn 섞기<X2, Y2>(self, other: 포인트_2<X2, Y2>) -> 포인트_2<X1, Y2> {
            포인트_2 {
                x: self.x,
                y: other.y,
            }
            // 기존 x값에 매개변수의 y값인 other.y를 대입하여 새로운 포인트를 생성하는 함수
        }
    }

    let 포인트_1 = 포인트_2 { x: 5, y: 10.4 };
    let 포인트_2 = 포인트_2 { x: 1.0, y: 40 };
    let 포인트_3 = 포인트_1.섞기(포인트_2);
    println!(
        "포인트_3의 x 값: {}, 포인트_3의 y값: {}",
        포인트_3.x, 포인트_3.y
    ); // 5, 40
}

fn 제네릭_코드의_성능() {
    // 제네릭 타입의 사용이, 구체적인 타입을 사용했을 때와 비교해서 전혀 속도를 느리게 하지 않는다.
    // 러스트는 컴파일 시점에, 제네릭을 사용하는 코드를 단형성화(monomorphization)한다. 단형성화란, 제네릭 코드를 실제 구체 타입으로 채워진 특정한 코드로 바꾸는 과정을 말한다.
    // 즉 컴파일러는 제네릭 코드가 호출된 곳을 전부 찾고, 제네릭 코드가 호출될 때 사용된 구체 타입으로 코드를 생성한다.

    let 정수 = Some(5);
    let 실수 = Some(5.0);
    // 러스트는 위의 두 줄을 컴파일할 때 단형성화를 수행한다. Option<T> enum을 사용하기 때문.
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }
    let 정수 = Option_i32::Some(5);
    let 실수 = Option_f64::Some(5.0);
    // 이러한 형태로 단형성화가 이뤄진다고 보면 되겠다. (실제로 컴파일러에 의해 생성되는 이름은 다르다.)
}
