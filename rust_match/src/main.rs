#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //후략
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 이렇게 이넘의 한 값이 다른 이넘을 참조할 수도 있다. 이후 29행 대신 30행 참조
}

fn main() {
    // match: 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매칭되었는지를 바탕으로 코드를 수행하도록 해준다.

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // match 뒤에 coin에 대한 표현식을 작성했는데, 이는 if를 사용하는 조건식과 유사하지만 큰 차이점이 있다.
            // `if`를 사용할 경우에는 조건문에서 부울린 값을 반환해야 하지만, 여기서는 어떤 타입이든 가능하다. 위 예제에서 `coin`의 타입은 첫째 줄에서 정의했던 `Coin` 이넘이다.
            // 아래의 4개의 갈래(arm)와 연관될 코드는 표현식이고, 이 match 갈래에서의 표현식의 결과로써 생기는 값은 전체 match 표현식에 대해 반환되는 값이다.
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // Coin::Quarter => 25,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            } // Coin 타입을 가진 파라미터 coin의 값을 비교하여, Penny면 Penny와 연관된 코드가 실행되고, 매칭되지 않는 경우 다음 갈래인 Nickel과의 매칭 여부를 확인한다.
              // match 갈래 내에서 여러 줄의 코드를 실행시키고 싶다면 중괄호를 사용하면 된다. 결과가 짧으면 보통 중괄호는 사용하지 않는다.
              // 자바스크립트의 Switch랑 비슷해보인다.
        }
    }
    value_in_cents(Coin::Quarter(UsState::Alabama)); // Coin 이넘의 Quarter값을 불러오는데, Quarter 값은 파라미터로 UsState를 받으므로 마찬가지로 UsState에서 사용할 enum값을 가져온다.

    {
        // Option<T>를 이용하는 match
        // Option<i32>를 파라미터로 받아서, 내부에 값이 있으면 그 값에 1을 더하는 함수를 작성(내부에 값이 없다면 None을 반환하고 어떤 연산도 시도하지 않음.)
        fn plus_one(x: Option<i32>) -> Option<i32> {
            // match 내부의 갈래(arm)의 패턴들은 모든 가능한 경우를 다뤄야 한다.
            // 아래 매치 함수의 경우, 예를 들어 `None => None`이 없었다면, `plus_one(None)`으로 불러왔을 때 에러가 생길 것이다. 이 match가 None에 대해서는 커버를 해주고 있지 않기 때문에.
            // 다행히도 이런 부분을 Rust 컴파일러가 잡아서 에러를 던진다. `non-exhaustive patterns: `None` not covered`
            // Option<T>를 사용할 때, 이렇게 None을 다루는 것을 깜빡하더라고 Rust가 알아채고 알려주기 때문에 Rust의 match와의 궁합이 좋다. Null일지도 모를 값을 갖고 있어서 발생할 수 있는 실수를 불가능하게 만든다.
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let _six = plus_one(five); // 이렇게 실행될 때 plus_one의 파라미터는 `Some(5)`가 된다. 이 경우 `Some(i) => Some(i + 1)`에 따라 6을 담은 새로운 Some 값을 생성한다.
        let _none = plus_one(None); // 이렇게 실행될 때 plus_one의 파라미터는 `None`이 되고, `None => None`에 따라 더할 값이 없으므로 None 값을 반환한다. 그리고 match는 종료된다.
    }
    {
        // 포괄(catch-all) 패턴: match 패턴에서 특정 몇개의 값들에 대해서만 특별한 동작을 하고, 나머지 값들에 대해서는 기본 동작을 취하도록 할 수 있다.
        // 자바스크립트에서 switch문의 default를 생각하면 될 듯하다.
        let dice_roll = 0;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),

            // other => move_player(other), // 이 경우가 default인 것이다. match 내 패턴들은 순차적으로 평가되므로, 마지막에 포괄적인 갈래를 위치시켜야 한다.
            // 포괄 패턴이 필요하지만, 꼭 그 포괄 패턴의 값을 사용할 필요는 없는 경우에 쓸 수 있는 패턴은 아래와 같다.
            // _ => reroll(), // 이 경우 3과 7을 제외한 어떤 값도 매칭은 되지만 그 값을 바인딩하지는 않는다.
            _ => (), // 이렇게 어떠한 코드의 실행조차 하지 않도록 할 수도 있다.
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        // fn move_player(num_spaces: u8) {}
        // fn reroll() {}
    }
}
