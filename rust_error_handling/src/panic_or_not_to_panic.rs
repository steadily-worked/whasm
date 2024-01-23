pub fn main() {
    // new 함수가 1과 100 사이의 값을 받았을 때만 인스턴스를 생성하는 Guess 타입을 정의하는 방법.
    pub struct Guess {
        // 숫자가 저장될 곳
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value } // value가 테스트를 통과한다면 value 파라미터로 value 필드를 설정한 새로운 Guess를 생성하여 이를 반환한다.
        }
        pub fn value(&self) -> i32 {
            self.value
            // 이 함수는 객체의 필드로부터 어떤 데이터를 가져와서 반환하는 것밖에 없음.
            // 이러한 형식의 메소드를 getter라고 부르는데, 이게 필요한 이유는 Guess struct의 value 필드가 비공개이기 때문.
            // value 필드가 비공개이기 때문에 Guess struct를 사용하는 코드는 value를 직접 설정할 수 없다는 것이 중요하다.
            // 모듈 밖의 코드는 반드시 `Guess::new` 함수로 새로운 Guess의 인스턴스를 생성해야 하며, 이를 통해 Guess가 `Guess::new` 함수의 조건에 의해 확인되지 않은 value를 가질 수 없음을 보장한다.
        }
    }
}
