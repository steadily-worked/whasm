pub struct 평균_컬렉션 {
    리스트: Vec<i32>,
    평균: f64,
}

impl 평균_컬렉션 {
    pub fn add(&mut self, value: i32) {
        self.리스트.push(value);
        self.평균_업데이트();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.리스트.pop();
        match result {
            Some(value) => {
                self.평균_업데이트();
                Some(value)
            }
            None => None,
        }
    }

    pub fn 평균(&self) -> f64 {
        self.평균
    }

    fn 평균_업데이트(&mut self) {
        let total: i32 = self.리스트.iter().sum();
        self.평균 = total as f64 / self.리스트.len() as f64;
    }
}

// 벡터에 대한 평균값을 담고있는 구조체: 현재 각 필드들이 접근불가이므로, 공개 메소드로만 접근할 수 있음. -> 외부 코드에서 list의 값을 변하게 하지 않기 위함
// 내부 필드의 공개 여부를 통해 캡슐화를 결정할 수 있다. 리스트 필드를 공개했을 경우, 외부 코드에서 리스트 필드에 접근하여 값을 변경해버릴 경우 관련 메소드들도 모두 변경되어야 함.

pub trait Draw {
    fn draw(&self);
    // draw 라는 이름의 메소드를 갖는 Draw trait 정의
}

pub struct Screen /*<T: Draw>*/ {
    // pub components: Vec<T>,
    pub components: Vec<Box<dyn Draw>>, // Box<dyn Draw> 타입의 벡터: 트레이트 객체.
                                        //  Draw 트레이트를 구현한 Box 내 모든 타입에 대한 대역
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    // components의 각 요소마다 draw 메소드를 호출하는 run 메소드 정의.
    // 트레이트 객체를 사용하면, 런타임에 트레이트 객체에 대해 여러 구체타입을 채워넣을 수 있음.
    // 동일 타입의 컬렉션만 사용한다면, 제네릭 + trait bound를 사용하는 게 바람직함.
    // -> 그 정의들은 컴파일 타임에 단형성화되어 구체 타입으로 사용되기 때문.
    // 트레이트 객체를 사용할 경우, Screen 인스턴스가 Box<Button>, Box<TextField> 등을 가질 수 있는
    // Vec<T>를 보유할 수 있음.
}

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button: width: {}, height: {}, label: {}",
            self.width, self.height, self.label
        );
    }
}
