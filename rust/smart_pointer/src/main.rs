use std::rc::Rc;

use smart_pointer::{MyBox, 커스텀_스마트_포인터};

fn main() {
    box_개념();
    box_재귀적_타입();
    box_참조자();
    자체_smart_pointer_정의();
    deref_트레이트();
    암묵적_역참조_강제();
    drop_트레이트();
    값을_일찍_버리기();
    rc_참조_카운트();
}

fn box_개념() {
    let b = Box::new(5);
    println!("b = {}", b);
    // 힙에 i32 값을 저장
    // 박스 안에 있는 데이터는, 마치 "이 데이터가 스택에 있는 것처럼" 접근할 수 있다.
    // but 이렇게 단일 값을 힙에 넣는 건 주로 안 함. i32는 기본적으로 스택에 저장되므로.
}

fn box_재귀적_타입() {
    // // 예시: (1, (2, (3, Nil)))
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // // 이 List 타입을 이용하면, 1, 2, 3을 저장할 경우 아래와 같이 보인다.
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // 이 List 타입은 재귀적이라서, 무한한 크기라고 말해준다: 즉, 이것은 자신의 또다른 값을 직접 갖는다.

    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn box_참조자() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // x: i32값 5. y에 대해서 단언하고 싶다면, *y로 참조자를 따라가서
    // 이 참조자가 가리키는 값을 얻어내어 컴파일러가 비교할 수 있도록 해야함
    // 역참조가 없다면, 숫자와 참조자를 비교하는 것은 불가능하므로 허용되지 않음.

    let x2 = 5;
    let y2 = Box::new(x2);
    assert_eq!(5, x2);
    assert_eq!(5, *y2);
    // y와의 차이점은, y2에 "x2의 복제된 값"을 가리키는 Box<T>의 인스턴스를 설정했다는 것
}

fn 자체_smart_pointer_정의() {
    struct MyBox<T>(T); // 튜플 구조체 정의: 모든 타입의 값을 가질 수 있도록 하고 싶기 때문.

    impl<T> MyBox<T> {
        // 유닛 구조체에 대한 메소드 정의
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    let x = 5;
    // let y = MyBox::new(x);
    assert_eq!(5, x);
    // assert_eq!(5, *y); // 에러 발생: type `MyBox<{integer}>` cannot be dereferenced
    // 즉, 역참조가 불가능하다는건데, 이는 그런 기능을 구현한 적이 없기 때문이다.
}

fn deref_트레이트() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // *y는 뒤편에서 *(y.deref())로 변환된다. 여기서 역참조로 감싸야 하는 이유는,
    // 소유권 시스템과 함께 작동하기 위함: deref 메소드가 값의 참조자 대신 "직접 값을 반환"했다면 그값은 self 밖으로 이동.
    // 이는, MyBox<T> 내부의 값에 대한 소유권을 얻으려는 것이고 이는 대부분의 경우 우리가 원하는 것이 아니기 때문에,
    // 역참조로 감싸는 것임.
}

fn 암묵적_역참조_강제() {
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // 역참조 강제는, "MyBox<String>" 타입 값에 대한 참조자로 "hello" 함수를 호출할 수 있게 해준다.
    // 여기서 &m이 "MyBox<String>"에 대한 참조자이다. MyBox<T>에서 deref 트레이트를 구현했으므로,
    // 러스트는 deref를 호출하여 "&MyBox<String>"을 "&String"으로 바꿀 수 있다.
    // 표준 라이브러리에 구현되어있는 String의 deref는 "str"을 반환하므로, 러스트는 다시한번 deref를 호출하여
    // &String 을 &str 로 바꾼다. 이것이 hello 함수의 정의와 일치하게 된다.

    // 만약 러스트에 역참조 강제가 구현되어있지 않았다면:
    let m2 = MyBox::new(String::from("Rust"));
    hello(&(*m2)[..]);
    // (*m2)는 MyBox<String>을 String으로 바꾼다. 그다음 &과 [..] 가 전체 문자열과 동일한 String의 문자열 슬라이스를
    // 받아와서 hello 시그니처와 일치되도록 함. 역참조 강제가 없다면, 이렇게 모든 기호를 이해해야 해서
    // 읽기도 쓰기도 이해하기도 어려워짐
}

fn drop_트레이트() {
    let _c = 커스텀_스마트_포인터 {
        데이터: String::from("안녕하세요"),
    };
    let _d = 커스텀_스마트_포인터 {
        데이터: String::from("안녕히가세요"),
    };
    println!("커스텀 포인터가 생성되었다.")
} // 이 부분에서 "커스텀_스마트_포인터" 인스턴스들은 스코프 밖으로 벗어날 것이고, 러스트는 `drop` 메소드에 집어넣은 코드를 호출할 것이고,
  // 이는 마지막 메시지를 출력한다:
  // "커스텀_스마트_포인터 인스턴스가 제거되었습니다. 데이터는: 안녕히가세요"와
  // "커스텀_스마트_포인터 인스턴스가 제거되었습니다. 데이터는: 안녕하세요".

fn 값을_일찍_버리기() {
    let c = 커스텀_스마트_포인터 {
        데이터: String::from("안녕하세요"),
    };
    println!("커스텀 포인터가 생성되었다.");
    // c.drop();
    // 위 코드는 에러를 발생시킴: drop을 명시적으로 호출하는 것이 허용되지 않음 -> 러스트가 여전히 main의 끝부분에서
    // 그 값에 대한 drop 호출을 자동으로 할 것이기 때문. 이는 러스트가 동일한 값에 대해 두 번 메모리 정리를 시도할
    // 것이므로 중복 해제(double free) 에러가 될 수 있음.
    // 어떤 값에 대한 메모리 정리를 강제로 일찍 하기 위해서는 `std::mem::drop` 함수를 이용함.

    drop(c);
    // "std::mem::drop"은 러스트의 표준 라이브러리에 있어서, 불러오지 않고도 사용할 수 있음.
    println!("main 함수가 끝나기 전에 커스텀 포인터가 제거되었다.")
}

fn rc_참조_카운트() {
    // 두 개의 리스트를 만들고, 이 둘이 모두 세 번째 리스트의 소유권을 공유하도록 함.
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }

    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // Cons variant는 자신이 들고있는 데이터를 소유하므로, b 리스트를 만들 때 a는 b 안으로 이동되어
    // b의 소유가 된다. 그다음 c를 생성할 때 a는 이미 이동되었으므로 a를 다시 사용할 수 없다.
    // 이 상황에서, Box<List>를 Rc<List>로 변경하면 가능해진다.
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // Rc는 prelude에 포함되어있지 않아서, 이를 스코프로 가져오려면 use 구문을 추가해야함.
    // main 안에서 5, 10을 갖고있는 리스트가 만들어지고 이것이 a의 새로운 Rc<List>에 저장됨.
    // 그다음 b와 c를 만들때는 Rc::clone 함수를 호출하고 a의 Rc<List>에 대한 참조자를 인수로서 넘김.
    // Rc::clone(&a) 대신 a.clone()을 호출할 수도 있지만, 위의 경우 러스트의 관례는 Rc::clone을 이용하는 것.
}
