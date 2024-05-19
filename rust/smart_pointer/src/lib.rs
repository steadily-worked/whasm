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
