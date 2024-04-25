fn main() {
    열거형();
    열거형2();
    option열거형();
}

fn 열거형() {
    // 열거형: 하나의 타입이 가질 수 있는 배리언트들을 열거함으로써 타입을 정의할 수 있도록 함.
    // 어떤 값이 여러 개의 가능한 값의 집합 중 하나라는 것을 나타내는 방법을 제공한다.
    enum 도형 {
        너비,
        높이,
    }
    let _너비 = 도형::너비;
    let _높이 = 도형::높이;
    // enum을 선언한 후, enum::key의 형태로 변수를 선언할 수 있다.

    fn route(rectangle: 도형) {}
    route(도형::너비); // 파라미터로 enum의 key를 넣을 수 있다.
    route(도형::높이);

    enum 도형2 {
        너비_4개(u8, u8, u8, u8),
        높이(String),
    }
    // 구조체로는, 튜플값을 담을 수가 없다.

    // let _너비2 = 도형2::너비_4개(String::from("good"));
    let _너비2 = 도형2::너비_4개(32, 32, 32, 32);
    // "도형2::너비" 는 u8 인수를 입력받아서, "도형2" 타입의 인스턴스 결과를 만드는 함수이다.
    // 열거형을 정의하면 이러한 생성자 함수가 자동으로 정의된다.
    let _높이2 = 도형2::높이(String::from("123"));

    struct 사각형 {
        너비: u32,
        높이: u32,
    }
    struct 삼각형 {
        너비: u32,
        높이: u32,
    }
    enum 도형3 {
        사각(사각형),
        삼각(삼각형),
    }
    // 열거형에는 어떤 종류의 데이터든 전부 담을 수 있다.
}

fn 열거형2() {
    impl Message {
        fn call(&self) { // 호출한 열거형의 값을 가져옴. m은 이 메소드 안에서, `self`가 됨.
                         // ..
        }
    }
    enum Message {
        Quit,                       // 연관 데이터 X
        Move { x: i32, y: i32 },    // 구조체처럼, 이름이 있는 필드를 가짐.
        Write(String),              // 하나의 String을 가짐
        ChangeColor(i32, i32, i32), // 세 개의 i32를 가지는 튜플.
    }

    struct QuitMessage; // 유닛 구조체
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // 튜플 구조체
    struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체
                                              // 열거형과 동일한 데이터를 가질 수 있는 구조체들.

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn option열거형() {
    // Option 타입: 값이 있거나 없을 수 있는 흔한 상황
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // Option<T>: 값의 존재, 부재를 표현할 수 있는 열거형. null을 대체한다.

    let 어떤_숫자 = Some(5);
    let _어떤_char = Some('e');
    let _없는_숫자: Option<i32> = None; // 값을 명시하지 않았으므로 타입을 명시해줘야 한다.

    let 어떤_숫자_2 = 5;
    // let 숫자들의_합 = 어떤_숫자 + 어떤_숫자_2; // 에러 발생: Option<i32>와 i32는 다른 타입이므로. Option<i32>는 자바스크립트로 치면 number | undefined이고, i32는 number라서, 없을 수도 있는 값을 더하려고 하므로 문제가 생긴다.

    // 이렇게, Option<T>를 사용하기 위해서는 각 variant를 처리할 코드가 필요하다.
    // Some(T)일 경우에는 확실하게 존재하는 것, None인 경우에는 존재하지 않는 것이므로, 관련해서 분기처리가 필요해보인다.
}
