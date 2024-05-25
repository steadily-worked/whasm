use make_gui::{Button, Draw, Screen};

fn main() {
    println!("
    객체(object), 캡슐화(encapsulation), 상속(inheritance)
    객체지향: 객체로 구성. 객체는 데이터 및 이 데이터를 활용하는 프로시저를 묶음. 이 프로시저들을 보통 메소드 혹은 연산(operation)이라고 부름
    ");
    상속();
    트레이트_객체를_사용하여_다른_타입의_값_허용하기();
}

fn 상속() {
    println!("
    상속: 어떤 객체가 다른 객체의 정의로부터 요소를 상속받을 수 있는 매커니즘.
    러스트에서는, 매크로를 사용하지 않고 부모 구조체의 필드와 메소드 구현을 상속받는 구조체를 정의할 방법이 없다.

    상속의 이유: 크게 2가지
    1. 코드의 재사용: 어떤 타입의 특정한 동작을 구현할 수 있고, 상속을 통해 다른 타입에 대해 그 구현을 재사용할 수 있음. 
       러스트에서는 기본 트레이트 메소드의 구현을 이용해서 제한적으로 공유할 수 있음
    2. 타입 시스템 관련: 자식 타입을 부모 타입과 같은 위치에서 사용할 수 있게 하기 위함
       (다형성(polymorphism): 여러 객체가 일정한 특성을 공유한다면, 이들을 런타임에 서로 대체하여 사용할 수 있음을 의미).
    
    다형성 vs 상속
    - 다형성은 여러 타입의 데이터로 작업할 수 있는 코드를 나타내는, 더 범용적인 개념임.
    - 상속에서는 이런 타입들이 일반적으로 하위클래스에 해당함

    최근에는, 필요 이상으로 많은 코드를 공유할 수 있는 위험,
    하위 클래스가 늘 그들의 부모 클래스의 특성을 공유하게 되어 프로그램 설계의 유연성을 저하시킬 수 있으므로 
    상속이 프로그래밍 디자인 솔루션으로써 선호되고 있지 않다.
    ")
}

struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectBox: width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn 트레이트_객체를_사용하여_다른_타입의_값_허용하기() {
    // 때로는, 라이브러리 사용자가 특정 상황에서 유효한 타입의 집합을 확장할 수 있도록 하길 원할 때가 있다.
    // Button, TextField 각 아이템에 대해 draw 메소드를 호출하여 화면에 그리는 GUI를 만들어 볼 것임.

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("예"),
                    String::from("아니오"),
                    String::from("취소"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("확인"),
            }),
        ],
    };
    println!(
        "
    SelectBox, Button의 타입을 가진 Box를 components 벡터에 담아 Screen 인스턴스를 생성
    lib.rs를 보면 Button 타입에 대해서만 구현을 해뒀지만 새로 만드는 SelectBox에 대해서도 잘 작동함.
    이는, SelectBox가 Draw 타입을 구현했고 draw 메소드를 통해 새로 만들 수 있기 때문.
    -> 값의 구체적인 타입이 아닌, 값이 응답하는 메시지만 고려하는 개념은
    덕타이핑의 개념과 유사하다: 오리처럼 걷고 오리처럼 꽥꽥거린다면 오리가 틀림없다는 것.
    "
    );

    screen.run();
    // run은, 어떤 구체 타입인지 알 필요가 없다. 컴포넌트의 타입과 무관하게 draw 메소드를 호출하여 새롭게 만들 뿐임.
    // 이는, components 벡터의 타입을 Box<dyn Draw>로 지정하는 것으로, draw 메소드의 호출이 가능한 값을 요구하는 Screen을 정의.

    // let screen = Screen {
    //     components: vec![Box::new(String::from("hi"))],
    // };

    // screen.run();
    // String이 Draw trait을 구현하지 않아서 에러가 뜸.

    println!("
    단형성화로부터 야기된 코드는 정적 디스패치를 수행하는데, 이는 호출하고자 하는 메소드가 어떤건지 컴파일러가 컴파일시점에 아는것임.
    동적 피스패치는, 컴파일러가 호출하는 메소드를 컴파일 시점에 알 수 없을 경우 수행된다. 이경우 컴파일러는 런타임에 어떤 메소드가
    호출되는지 알아내는 메소드를 생성함.

    트레이트 객체 사용 시 러스트는 동적 디스패치를 이용해야 함. 어떤 타입에 구현된 어떤 메소드가 호출될지 알지 못함.
    대신 런타임에서 트레이트 객체 내 포인터를 사용해서 어떤 메소드가 호출될지 알아냄. 이는 런타임 코스트를 만들어내는데,
    유연성을 위해 고려할 만한 tradeoff임.
    ")
}
