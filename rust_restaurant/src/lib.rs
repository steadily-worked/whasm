// 모듈은 크레이트의 코드를 읽기 쉽고 재사용하기도 쉽게끔 구조화를 할 수 있게 해준다. 모듈 내의 코드는 기본적으로 비공개이므로 모듈은 아이템의 공개 여부(privacy)를 제어하도록 해주기도 한다.
// 비공개 아이템은 외부에서의 사용이 허용되지 않는 내부의 세부 구현이다. 모듈과 모듈 내 아이템을 선택적으로 공개할 수 있는데, 이렇게 하여 외부의 코드가 모듈 및 아이템을 의존하고 사용할 수 있도록 노출해 준다.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // fn seat_at_table() {}
    }

    mod serving {
        // fn take_order() {}

        // fn serve_order() {}

        // fn take_payment() {}
    }
}

// 앞서 `src/main.rs`와 `src/lib.rs`는 크레이트 루트로 부른다고 언급했었는데, 그 이유는 모듈 트리(module tree)라고 불리는 크레이트 모듈 구조에서 최상위에 `crate`라는 이름을 갖는 일종의 모듈로 형성되기 때문이다.
// 위의 코드를 모듈 트리의 형태로 구조화한 것이다.
// crate
//  └── front_of_house
//  ├── hosting
//  │   ├── add_to_waitlist
//  │   └── seat_at_table
//  └── serving
//      ├── take_order
//      ├── serve_order
//      └── take_payment
// 트리는 모듈이 서로 어떻게 중첩되어 있는지 보여준다; 예를 들어 `hosting` 모듈은 `front_of_house` 내에 위치한다.
// 이 트리는 또한 어떤 모듈이 서로 형제(sibling) 관계에 있는지 나타내기도 하는데, 이는 동일한 모듈 내에 정의되어 있음을 말한다; `hosting`과 `serving`은 `front_of_house` 모듈 내에 정의된 형제이다.
// 모듈 A가 모듈 B 안에 있으면 모듈 A는 모듈 B의 자식이며, 모듈 B는 모듈 A의 부모라고 말한다. 전체 모듈 트리 최상위에 `crate`라는 모듈이 암묵적으로 위치한다.

// 경로는 두가지 형태가 존재한다.
// 1. 절대 경로: 크레이트 루트로부터 시작되는 전체 경로이다; 외부 크레이트로부터의 코드에 대해서는 해당 크레이트 이름으로 절대 경로가 시작되고 현재의 크레이트로부터의 코드에 대해서는 `crate` 리터럴로부터 시작된다.
// 2. 상대 경로: 현재의 모듈을 시작점으로 하여 `self`,`super` 혹은 현재 모듈 내의 식별자를 사용한다.
// 절대 경로와 상대 경로의 뒤에는 `::`으로 구분된 식별자가 하나 이상 따라온다.

// eat_at_restaurant 함수는 라이브러리 크레이트의 공개 API중 하나이다. 그렇기에 `pub` 키워드로 지정되어 있다.
pub fn eat_at_restaurant() {
    // `add_to_waitlist` 함수를 호출하고 싶다면..
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();
    // `add_to_waitlist` 함수는 `eat_at_restaurant` 함수와 동일한 크레이트에 정의되어 있으므로 절대 경로의 시작점에 `crate` 키워드를 사용할 수 있다.

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
    // `eat_at_restaurant`와 동일한 위치에 지정되어 있는 `front_of_house` 모듈로 시작한다.
    // 모듈 이름으로 시작한다는 것이 즉 상대 경로를 의미한다.
}

// 해당 프로젝트를 빌드하려고 하면 실패한다. `hosting` 모듈이 private하기 때문이다. `pub` 처리를 하지 않으면 접근이 불가능하다.
// pub mod hosting 처리를 해주면 되는데, 그렇다 하더라도 빌드 에러가 발생한다. 이유는 이제 `add_to_waitlist` 함수가 private하기 때문이다.
// `hosting` 모듈 자체에는 접근을 할 수가 있지만, `hosting` 모듈의 '내용'은 여전히 비공개이다. 모듈을 공개했다고 해서 내용까지 공개되지는 않는다.
// pub fn add_to_waitlist() 처리를 해주면 컴파일이 되는데, 여기서 궁금한 점은 '왜 front_of_house' 모듈은 비공개인데 접근이 가능할까? 가 될 것이다.
// `front_of_house` 모듈은 공개가 아니지만, `eat_at_restaurant` 함수와 `front_of_house` 모듈은 같은 모듈 내에 정의되어 있으므로(즉, sibling 관계이므로) `eat_at_restaurant` 함수에서 `front_of_house` 모듈을 참조할 수 있는 것이다.

// `super`로 시작하면, 현재 모듈 혹은 크레이트 루트 대신 자기 부모 모듈로부터 시작되는 상대 경로를 만들 수 있다.

fn deliver_order() {}

mod back_of_house {
    // struct에 `pub`를 사용하면 구조체는 공개되지만 구조체의 필드는 비공개로 유지된다. (이후 92행으로 이동)
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 구조체와 다르게 이넘은 공개로 지정할 경우 모든 variant가 공개된다.
    // 이넘은 그 variant가 공개되지 않는다면 큰 쓸모가 없다; 이넘의 모든 variant에 대해 전부 `pub`를 붙이는 것은 귀찮은 일이 될 것이므로, 열거형의 variant는 기본적으로 공개이다.
    // 구조체의 경우 종종 필드를 공개로 하지 않는 것이 유용하므로 `pub`를 명시하지 않는 한 기본적으로 모든 것이 비공개라는 일반적인 규칙을 따른다.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super가 없으려면, `use crate::delivery_order;`로 모듈에서 불러와야 한다.
    }

    fn cook_order() {}
}

pub fn eat_breakfast_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("호밀");
    meal.toast = String::from("밀");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("블루베리"); 이 줄을 주석해제하면 컴파일되지 않는다.

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    //
}
// `back_of_house::Breakfast` 구조체의 `toast` 필드는 공개 필드이기 때문에 `eat_breakfast_at_restaurant` 함수에서 점 표기법으로 `toast`를 읽고 쓸 수 있지만,
// `seasonal_fruit` 필드는 비공개 필드이기 때문에 `eat_breakfast_at_restaurant` 함수에서 사용할 수 없다: 92행의 주석을 해제하면 컴파일되지 않는다.
