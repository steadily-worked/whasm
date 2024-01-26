fn main() {
    {
        // 가장 큰 수를 찾는 프로그램
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];
        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        // 두 개의 다른 숫자 리스트에서 가장 큰 숫자를 찾는 프로그램. 아래 코드에서 이어짐. 두 개의 array의 가장 큰 값을 찾고 큰 값을 비교..
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let mut largest = &number_list[0];
        for number in &number_list {
            if number > largest {
                largest = number
            }
        }

        println!("The largest number is {}", largest);
    }
    // 동작은 하지만 너무 길다. 중복된 코드가 많다. 로직을 수정하기 위해 바꿔야 할 부분이 많다.
    {
        fn largest(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        // 이전 코드에 비해
        // 1. 중복된 코드를 식별하고
        // 2. 중복된 코드를 함수의 본문으로 분리하고, 함수의 시그니처 내에 해당 코드의 입력값 및 반환값을 명시했다.
        // 3. 중복됐었던 두 지점의 코드를 함수 호출로 변경했다.
    }
    {
        fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    // error[E0369]: binary operation `>` cannot be applied to type `&T`
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
        // 이 코드는 컴파일에 실패한다. 이유는, `largest`의 본문이 `T`가 될 수 있는 모든 타입에 동작할 수 없기 때문이다.
        // 함수 본문에서 T 타입 값들에 대한 비교가 필요하므로, 여기에는 값을 정렬할 수 있는 타입에 대해서만 동작할 수 있다.
        // 우선은, T의 타입으로 `std::cmp::PartialOrd` 를 넣어주면 된다. 이는, 표준 라이브러리가 `i32`와 `char` 둘 모두에 대한 `PartialOrd`를 구현하고 있기 때문이다.
    }
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        fn main() {
            let _integer = Point { x: 5, y: 10 };
            let _float = Point { x: 1.0, y: 4.0 };
            // let wont_work = Point { x: 1.0, y: 4 }; // expected floating-point number, found integer
        }
        // Point<T> 선언에 하나의 제네릭 타입만 사용했으므로, 이 선언은 `Point<T>`가 어떤 타입 T에 대한 제네릭이며 `x`,`y` 필드는 실제 타입이 무엇이건 간에 둘다 동일한 타입이라는 것을 의미한다.
        // 무조건 같은 타입이어야 하므로, 89행의 `wont_work` 변수는 컴파일에 실패한다.

        struct MultiPoint<T, U> {
            x: T,
            y: U,
        }
        fn main2() {
            let _both_integer = MultiPoint { x: 5, y: 10 };
            let _both_float = MultiPoint { x: 1.0, y: 4.0 };
            let _integer_and_float = MultiPoint { x: 5, y: 4.0 };
        }
        // 제네릭의 타입이 두개일 경우, 자유롭게 부여하여 사용할 수 있다. 이 경우는 같아도 된다.
    }
    {
        enum Option<T> {
            Some(T),
            None,
        }
        // `Option<T>` 이넘은 `T` 타입에 대한 제네릭이며, `T` 타입을 들고 있는 `Some` Variant와 아무런 값도 들고 있지 않은 `None` Variant를 갖는다.
        // `Option<T>` 이넘을 사용함으로써 옵션 값에 대한 추상화된 개념을 표현할 수 있고, `Option<T>` 이넘이 제네릭으로 되어있는 덕분에 옵션 값이 어떤 타입이건 상관없이 추상화해서 사용할 수 있다.

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
        // `Result` 이넘은 `T`,`E` 두 타입을 이용한 제네릭이며, `T` 타입을 갖는 `Ok`와 `E` 타입을 갖는 `Err` Variant를 갖는다.
        // 제네릭으로 정의되어 있는 덕분에 연산이 성공할지(따라서 `T` 타입 값을 반환할지) 실패할지(`E` 타입 값을 반환할지) 알 수 없는 어느 곳이든 `Result` 이넘을 편리하게 사용할 수 있다.
    }
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            // impl 바로 뒤에 T를 선언하여 `Point<T>` 타입에 메소드를 구현한다고 명시. 이렇게 할 경우 Rust는 Point의 꺾쇠괄호 내 타입이 구체적인 타입이 아닌 제네릭 타입임을 인지한다.
            // struct 정의에 선언된 제네릭 매개변수와는 다른 제네릭 매개변수를 선택할 수도 있었겠지만 같은 이름을 사용하는 것이 관례이다.
            // 제네릭 타입이 선언된 `impl` 안에 작성된 메소드는 이 제네릭 타입에 어떤 구체 타입을 집어넣을지와는 상관없이 어떠한 타입의 인스턴스에라도 정의될 것이다.
            fn x(&self) -> &T {
                // x 필드 데이터의 참조자를 반환하는 x 메소드
                &self.x
            }
        }
        fn main() {
            let p = Point { x: 5, y: 10 };
            println!("p.x={}", p.x()) // p.x=5
        }
    }
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl Point<f32> {
            // 이 경우는 제네릭 타입의 파라미터 T가 `f32`일 경우에만 적용이 된다.
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
            // T가 `f32` 타입이 아닌 `Point<T>` 인스턴스는 이 메소드가 정의되지 않는다. 이 함수는 생성된 점의 원점으로부터의 거리를 리턴하는 함수이다.
        }
    }
    {
        // struct 정의에서 사용한 제네릭 타입 파라미터와 struct의 메소드 시그니처 내에서 사용하는 제네릭 타입 파라미터가 항상 같진 않다.
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }
        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                // `self` `Point`의(X1 타입인) x값과 파라미터로 넘겨받은 `Point`의(Y2 타입인) y값으로 새로운 Point 인스턴스를 생성한다.
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
        fn main() {
            let p1 = Point { x: 5, y: 10.4 };
            let p2 = Point { x: "Hello", y: 'c' };
            let p3 = p1.mixup(p2);
            println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
        }
        // `main`에서는 `i32` 타입 `x`(`5`)와 `f64` 타입 `y`(`10.4`)를 갖는 `Point`를 정의함.
        // mixup의 결과인 p3은 `Point<i32, char>`의 타입을 가짐.
        // 이 예제는, 제네릭 파라미터 중 일부가 `impl`에 선언되고 일부는 메소드 정의에 선언되는 경우를 보여주기 위한 예시이다.
        // 여기서 제네릭 파라미터 `X1`,`Y1`는 struct의 정의와 한 묶음이니 `impl` 뒤에 선언했지만, 제네릭 파라미터 `X2`,`Y2`는 `mixup` 메소드에만 연관되어 있으므로 `fn mixup` 뒤에 선언한다.
    }
    {
        // 제네릭 타입의 사용은 구체적인 타입을 사용했을 때와 비교해서 전혀 느리지 않다.
        // Rust는 컴파일 타임에 제네릭을 사용하는 코드를 단형성화(monomorphization)한다. 단형성화란 제네릭 코드를 실제 구체 타입으로 채워진 특정한 코드로 바꾸는 과정이다.
        // 이 과정에서, 컴파일러는 제네릭 코드가 호출된 곳을 전부 찾고 제네릭 코드가 호출할 때 사용된 구체 타입으로 코드를 생성한다.
        let integer = Some(5);
        let float = Some(5.0);
        // Rust는 이 코드를 컴파일할 때 단형성화를 수행한다. 이 과정 중 컴파일러는 `Option<T>` 인스턴스에 사용된 값을 읽고, `i32`,`f64`의 두 종류의 `Option<T>`가 있다는 것을 인지한다.
        // 그리고, 제네릭 정의를 `i32`와 `f64`에 대해 특성화시킨 정의로 확장함으로써, 제네릭 정의를 이 구체적인 것들로 대체한다.

        // 위의 코드를 단형성화한다면 아래처럼 될 것이다(이름은 예시에 불과함).
        enum Option_i32 {
            Some(i32),
            None,
        }
        enum Option_f64 {
            Some(f64),
            None,
        }
        fn main() {
            let integer = Option_i32::Some(5);
            let float = Option_f64::Some(5.0);
        }
        // 제네릭 `Option<T>`가 컴파일러에 의해 특정한 정의들로 대체되었다. Rust 컴파일러가 제네릭 코드를 각 인스턴스의 명시적인 타입으로 변경해 주는 덕분에, 굳이 런타임 비용을 줄이기 위해 수동으로 직접 각 타입마다 중복된 코드를 작성할 필요가 없다.
        // 단형성화 과정은 Rust 제네릭을 런타임에 극도로 효율적으로 만들어준다.
    }
}
