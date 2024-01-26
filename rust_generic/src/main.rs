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
}
