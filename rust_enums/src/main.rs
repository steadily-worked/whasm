enum IpAddrKink {
    V4,
    V6,
}

fn main() {
    struct IpAddr {
        kind: IpAddrKink,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKink::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKink::V6,
        address: String::from("::1"),
    };

    {
        // 각 enum variant에 데이터를 직접 넣는 방식을 사용해서 이넘을 구조체의 일부로 사용하는 방식보다 간결하게 동일한 개념을 표현할 수도 있다.
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let _home = IpAddr::V4(String::from("127.0.0.1"));
        // 여기서 `IpADDR::V4`는, String 인수를 입력받아서 `IpAddr` 타입의 인스턴스를 만드는 함수이다. 이넘을 정의한 결과로써 이러한 생성자 함수가 자동적으로 정의된다.
        let _loopback = IpAddr::V6(String::from("::1"));
    }
    {
        // 구조체 대신 이넘을 사용하는 경우, 각 배리언트는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있다.
        // V4 IP 주소는 항상 0~255 사이의 숫자 4개로 된 구성요소를 갖게 될 것이다. V4의 주소에는 u8 값을 저장하길 원하지만 V6 주소는 하나의 String 값으로 표현되길 원한다면 구조체로는 이렇게 할 수 없지만 이넘은 쉽게 처리할 수 있다.
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let _home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));
    }
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        // Quit: 연관된 데이터가 전혀 없다.
        // Move: 구조체처럼 이름이 있는 필드를 가진다.
        // Write: 하나의 String을 가진다.
        // ChangeColor: 3개의 `i32`를 가진다.

        // 위의 Message 이넘은 아래처럼 4개의 구조체를 가진 것과 같다.
        struct _QuitMessage; // 유닛 구조체
        struct _MoveMessage {
            x: i32,
            y: i32,
        }
        struct _WriteMessage(String); // 튜플 구조체
        struct _ChangeColorMessage(i32, i32, i32); // 튜플 구조체

        // 이넘에도 `impl`을 사용하여 메소드를 정의할 수 있다. 아래는 `Message` 이넘 내에 정의한 `call` 메소드이다.
        impl Message {
            fn call(&self) {
                // 메소드 본문이 여기에 정의된다.
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        // Option 이넘: 이넘 내에 optional value가 존재하는 경우.
        enum Option<T> {
            None,
            Some(T),
        }
        let some_number = Some(5);
        let some_char = Some('e');

        let absent_number: Option<i32> = None;
        // Some 값을 얻게 되면, 값이 존재한다는 것과 해당 값이 Some 내에 있다는 것을 알 수 있다.
        // None 값을 얻게 되면, 얻은 값이 유효하지 않다는, 어떤 면에서는 Null과 같은 의미를 갖는다.
        // Option<T>가 Null보다 나은 이유는 뭘까?
        // Option<T>와 T는 다른 타입이기 때문에, 컴파일러는 Option<T> 값을 명백하게 유효한 값처럼 사용하지 못하도록 한다.

        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        let sum = x + y;
        // Option<i8>과 i8은 다른 타입이기 때문에, Rust는 이 둘을 어떻게 더해야 하는지 모른다. 그렇기 때문에 컴파일러는 에러를 뱉는다.
        // Option<T>를 T로 변환해주는 과정이 있어야만 이러한 연산이 가능해진다.
        // Nullable한 값을 사용하기 위해서는, 명시적으로 값의 타입을 Option<T>로 만들어줘야 한다. 그 후 값을 사용할 때 명시적으로 Null인 경우를 처리해야 한다.
    }
    {}
}
