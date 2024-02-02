fn main() {
    // Structs(구조체): 여러 개의 관련 값을 보유한다는 점에서 튜플 타입과 유사하다.
    // 그렇지만, 각 데이터 조각에 네이밍을 하여 값의 의미를 명확하게 알 수 있고, 이렇게 이름을 지정한 경우 인스턴스의 값을 지정하거나 액세스하기 위해 데이터의 순서에 의존할 필요가 없기 때문에 튜플보다 유연하다.
    // (내 의견: 타입스크립트의 interface가 하는 일과 우선은 비슷해 보인다.)
    // 구조체를 지정하려면, `struct` 키워드를 입력하고 전체 구조체의 이름을 지정한다.
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // 실제 구조체를 사용할 경우 타입을 지정하고 required value의 default value를 모두 지정해준다.
    let mut user1 = User {
        active: true,
        username: String::from("steadily-worked"),
        email: String::from("iamsangminpark@gmail.com"),
        sign_in_count: 1,
    };

    // 이렇게 직접 접근하여 값을 바꿔줄 수도 있다(물론 이렇게 하려면 user1 구조체가 mutable해야 한다.)
    // 가변성은 해당 인스턴스 전체가 지니게 된다.
    user1.email = String::from("iamsangminpark2@gmail.com");

    // email과 username을 받고 User 인스턴스를 반환
    fn _build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // 다행히 Rust에서도 자바스크립트에서처럼 key=value인 경우 축약할 수 있다.
            email,
            sign_in_count: 1,
        }
    }

    // user1의 값 중 하나(이메일)만 다르게 한 새로운 user2 인스턴스
    let _user2 = User {
        // active: user1.active,
        // username: user1.username,
        // email: String::from("iamsangminpark3@gmail.com"),
        // sign_in_count: user1.sign_in_count,
        // 위와 같은 형태는 아래와 같이 더 편하게 작성할 수 있다.
        email: String::from("iamsangminpark3@gmail.com"),
        ..user1 // 자바스크립트의 rest parameter를 생각하면 될 듯. 이 값은 가장 마지막에 적어야 한다.
    };
    // 이렇게 _user2를 생성한 이후에는 더이상 user1을 사용할 수 없다. 그 이유는, user1의 username 필드의 String이 user2로 이동되기 때문이다.
    // user1을 사용하려면, email과 username 필드의 String을 모두 user2로 제공하고 active와 sign_in_count만 사용할 경우엔 가능하다.

    {
        // Rust는 튜플 구조체 또한 지원한다. 구조체에는 이름을 지어주지만 필드에는 타입만 적어 넣은 형태이다.
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        fn _main() {
            let _black = Color(0, 0, 0);
            let _origin = Point(0, 0, 0);
            // 당연히 _black과 _origin의 타입은 다르다. 구조체의 타입이 다르기 때문.
        }
    }

    {
        // 필드가 아예 없는 유사 유닛 구조체(unit-like structs)를 정의할 수도 있다.
        struct AlwaysEqual;

        fn _main() {
            let _subject = AlwaysEqual;
            // 이렇게 선언만 해주면 된다. `AlwaysEqual`의 모든 인스턴스는, 언제나 다른 모든 타입의 인스턴스와 같도록 하는 동작을 구현하여 이미 알고 있는 결과값의 테스트 용도로 사용한다고 가정했을 때 유용하다. 왜냐면 이런 동작에는 데이터가 필요 없기 때문이다.
        }
    }

    // 참고로, 지금까지 공부한 내용만으로는 구조체에는 &str 타입을 사용할 수 없다. 구조체 인스턴스가 유효한 동안 각 인스턴스 내의 모든 데이터가 유효하도록 만들기 위해서 String 타입을 사용한다.
    // 참조자를 이용해 구조체가 소유권을 갖지 않는 데이터도 저장할 수는 있지만, 이경우 이후에 공부할 lifetime을 활용해야 한다. lifetime을 활용할 경우 구조체가 존재하는 동안에 구조체 내 참조자가 가리키는 데이터의 유효함을 보장받을 수 있기 때문이다.
    // lifetime을 명시하지 않고 참조자를 저장하려고 한다면, `expected named lifetime parameter` 라는 에러를 내뱉는다.
}
