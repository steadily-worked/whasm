// 절대 경로건 상대 경로건 `add_to_waitlist`를 호출할 때마다 `front_of_house`,`hosting` 모듈을 매번 지정해줘야 했던 것들은 꽤나 불편하고 반복적인 느낌을 준다.
// 이 과정을 단축하는 방법이 있다: `use` 키워드를 한번 사용하여 어떤 경로의 단축경로를 만들 수 있고, 그렇게 될 경우 스코프 안쪽 어디서라도 짧은 이름을 사용할 수 있다.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
// `crate::front_of_house::hosting` 모듈을 `eat_at_restaurant` 함수가 존재하는 스코프로 가져와서 `eat_at_restaurant` 함수 내에서 `add_to_waitlist` 함수를 `hosting::add_to_waitlist` 경로만으로 호출한다.
// 크레이트 루트에 `use crate::front_of_house::hosting`를 작성하면, 해당 스코프에서 `hosting` 모듈을 크레이트 루트에 정의한 것처럼 사용할 수 있다.
// 한편 `use` 키워드로 이름을 가져올 경우, 해당 이름은 새 위치의 스코프에서 비공개가 된다.
// `pub use`를 사용하면, 우리 코드를 호출하는 코드가 해당 스코프에 정의된 것처럼 해당 이름을 참조할 수 있다. (re-exporting)
// `pub use crate::front_of_house::hosting;`의 형태로 바꿀 경우, 이 `pub use`가 루트 모듈로부터 `hosting` 모듈을 다시 내보냈으므로 이제 외부 코드는 `restaurant::hosting::add_to_waitlist()`로 경로를 대신 사용할 수 있다.

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    use crate::front_of_house::hosting; // `use` 구문은 사용된 스코프 내에서만 적용이 되기 때문에, 새로운 모듈 내에서 불러오려면 다시 `use crate`를 사용하여 불러와야 한다.

    pub fn _eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// 근데, 왜 `hosting`까지만 함수를 호출하고 `hosting::add_to_waitlist`의 형태로 사용해야 할까?
// `use crate::front_of_house::hosting::add_to_waitlist;` 를 사용하여 불러와도 되지만, 보편적인 작성 방식은 아니다.
// 함수의 부모 모듈을 `use` 키워드로 가져오면, 함수를 호출할 때 부모 모듈을 특정해야 한다.
// 함수 호출 시 부모 모듈을 특정하면, 전체 경로를 반복하는 것을 최소화하면서도 함수가 로컬에 정의되어 있지 않음을 명백히 보여줄 수 있다.
use crate::front_of_house::hosting::add_to_waitlist;

pub fn _eat_at_restaurant() {
    add_to_waitlist();
}
// 이와 같은 방식을 사용해도 되지만, 이 경우 33행의 함수만으로는 이 `add_to_waitlist`가 어디에 정의되어 있는지가 불분명하다.
