use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // 표준 라이브러리의 Deref 트레이트는, deref의 메소드를 하나 구현하도록 요구함.
    // -> 이는 self를 빌려와서 내부 데이터의 참조자를 반환.
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
    // deref의 메소드 본문이 "&self.0"을 반환하도록 구현했으므로,
    // deref는 "*" 연산자를 이용하여 접근하려는 값의 참조자를 반환한다.
    // 이제 MyBox<T> 인스턴스에 대해 "*" 연산자를 사용할 수 있고, assert_eq!(5, *y) 는 컴파일이 된다.
}

pub struct 커스텀_스마트_포인터 {
    pub 데이터: String,
}

impl Drop for 커스텀_스마트_포인터 {
    fn drop(&mut self) {
        println!(
            "커스텀_스마트_포인터 인스턴스가 제거되었습니다. 데이터는: {}",
            self.데이터
        );
        // drop 함수의 본문에는, 해당 타입의 인스턴스가 스코프 밖으로 벗어났을 때 실행시키고 싶은 어떤 로직이라도
        // 집어넣을 수 있다. 여기서는 인스턴스가 제거되었을 때 데이터를 출력하도록 했다.
    }
}

// 최대값을 기준으로 어떤 값을 추적하여 현재 값이 최대값에 얼마나 근접했는지에 대한 메시지를
// 전송하는 라이브러리 예시.
pub trait Messenger {
    fn send(&self, msg: &str);
    // 불변 참조자와 메시지의 텍스트를 파라미터로 갖는 send 메소드 하나를 갖고있음.
}

pub struct LimitTracker<'a, T: Messenger> {
    메신저: &'a T,
    값: usize,
    최대값: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(메신저: &'a T, 최대값: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            메신저,
            값: 0,
            최대값,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        // 이 동작은 테스트가 필요함. value에 뭘 넘길지를 바꿀 순 없지만,
        // set_value는 단언에 필요한 어떤 것도 반환하지 않음.
        // Messenger 트레이트를 구현한 어떤 것과, 최대값에 대한 특정 값과 함께 LimitTracker를 만든다면
        // value에 대해 다른 숫자들을 넘겼을 때 메신저가 적합한 메시지를 보냈다고
        // 말할 수 있길 원하는 것.
        self.값 = value;

        let 비율 = self.값 as f64 / self.최대값 as f64;

        if 비율 >= 0.75 && 비율 < 0.9 {
            self.메신저.send("경고: 사용량이 75% 이상입니다.");
        } else if 비율 >= 0.9 && 비율 < 1.0 {
            self.메신저.send("긴급 경고: 사용량이 90% 이상입니다.");
        } else if 비율 >= 1.0 {
            self.메신저.send("에러: 사용량이 100% 이상입니다.");
            // 각 케이스들에 대해 send했다고 언급하는 메시지만 추적할 mock object가 필요함.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        전송된_메시지: Vec<String>,
        // 전송된_메시지: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                전송된_메시지: vec![],
                // 전송된_메시지: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // 불변 참조자를 가져옴 -> MockMessenger 수정 불가.
            self.전송된_메시지.push(String::from(msg));
            // &mut self를 사용하는 것도 불가능하다. 이렇게되면 Messenger 트레이트 정의에 있는 시그니처(41행 &self)와 맞지 않게 되기 때문이다.
            // self.전송된_메시지.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn 메시지_보내야함_75퍼_넘었다는() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.전송된_메시지.len(), 1);
        // assert_eq!(mock_messenger.전송된_메시지.borrow().len(), 1);
    }
}

// 이 테스트 코드는, 보냈다고 알려주는 메시지를 추적하기 위한 String Vector인 "전송된_메시지" 필드를 갖는
// MockMessenger 구조체를 정의한다.
// 다음 new 를 정의하여 새로운 MockMessenger 값을 생성할 수 있도록 하고
// MockMessenger에 대한 Messenger 트레이트를 구현하여 MockMessenger를 LimitTracker에 넘겨줄 수 있도록 한다.
// send 메소드의 정의부분에는 파라미터로 넘겨진 메시지를 가져와서 MockMessenger 내 "전송된_메시지" 리스트에 저장한다.

// 테스트 내에서는 LimitTracker의 "값"에 "최대값"의 75% 이상인 어떤 값이 설정되었다고 했을 때 무슨 일이 일어나는지 테스트 중이다.
// 먼저 새로운 MockMessenger(빈 벡터)를 만들고, 새로운 LimitTracker를 만들고 여기에 MockMessenger의 참조자와 "최대값" 값 100을
// 파라미터로 넘긴다. LimitTracker의 set_value 메소드를 80값으로 호출했는데, 이는 75% 이상이다. 그다음
// MockMessenger가 추적하고 있는 메시지 리스트가 이제 한개의 메시지를 갖고있는지를 검사한다.

// 이 케이스가 "내부 가변성"의 도움을 받을 수 있는 상황이다: "전송된_메시지"가 RefCell<T> 내에 저장되게 하면,
// send 메소드는 "전송된_메시지"를 수정하여 우리에게 보이는 메시지를 저장할 수 있게 될 것이다.

// RefCell<T>로 변경 후 ...
// "전송된_메시지" 필드는 이제 Vec<String> 대신 RefCell<Vec<String>> 타입이다.
// new 함수에서는 빈 벡터를 감싼 새로운 RefCell<Vec<String>> 인스턴스를 생성한다.
// send 메소드의 구현부에서 첫번째 파라미터는 여전히 self의 불변 대여(&self) 형태인데, 이는 트레이트의 정의와 일치한다.
// "self.전송된_메시지" 의 "RefCell<Vec<String>>"에 있는 borrow_mut 를 호출하여 RefCell<Vec<String>> 내부 값,
// 즉 벡터에 대한 가변 참조자를 얻고 다음으로 그 벡터에 대한 가변 참조자의 push를 호출하여 테스트하는 동안 보내진 메시지를 추적할 수 있다.
// 내부 벡터 안에 몇개의 아이템이 있는지 보기위해 RefCell<Vec<String>>의 borrow를 호출하여 벡터에 대한 불변 참조자를 얻는다.
