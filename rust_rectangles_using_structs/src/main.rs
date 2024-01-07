#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 메소드. 함수와는 달리 구조체 컨텍스트에 정의되고 첫번째 매개변수가 항상 `self`임.
impl Rectangle {
    fn area(&self) -> u32 {
        // self: Rectangle 구조체 자체를 의미함.
        // 소유권을 가져오는 것이 목적이 아니므로 &self. 호출한 인스턴스의 값을 변경하고 싶다면 매개변수로 `&mut self``를 사용하면 된다.
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (width1, height1);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of rectangle is {} square pixels.(using tuple)",
        // area(width1, height1)
        area(rect1)
    );

    println!(
        "The area of rectangle is {} square pixels.(using struct)",
        area_with_structs(&rect2)
    );
    println!(
        "The area of rectangle is {} square pixels.(using impl in struct)",
        rect2.area()
    );

    if rect2.width() {
        // width 뒤에 괄호를 붙이면, Rust는 `width` 메소드를 의도한다는 것을 인지한다. 괄호가 없다면 Rectangle 구조체의 `width` 필드를 의도한다는 것으로 인지한다.
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    println!("rect2 is {:#?}", rect2);
    // 한편, `rect2` 구조체를 println!으로 찍어보려고 하면 `Rectangle` doesn't implement `std::fmt::Display` 라는 에러가 발생한다.
    // {}로 지정해서 println!을 실행할 땐 Display라는, 최종 사용자를 위한 출력 형식을 사용한다. 지금까지의 기본 타입들은 Display가 기본적으로 구현되어 있었다.
    // 하지만 구조체의 경우, 중간중간 쉼표를 사용해야 할 수도 있고 중괄호도 출력해야 할 수도 있고 필드의 일부를 생략해야 할 수도 있는 등 여러 가지가 가능하다.
    // Rust는 이러한 애매한 상황에 우리가 원하는 걸 임의로 예상해서 제공하려 들지 않기 때문에 구조체에는 `println!` 및 `{}` 자리표시자와 함께 사용하기 위한 `Display` 구현체가 기본 제공되지 않는다.
    // 에러 메시지는 {} 대신 {:?}를 사용해서 프린트를 하라고 한다. {:?}은 `println!`에 `Debug`라는 출력 형식을 사용하고 싶다고 전달하는 것이다.
    // 이 `Debug`라는 trait는 최종 사용자가 아닌, 개발자에게 유용한 방식으로, 디버깅하는 동안 값을 볼 수 있게 해주는 trait이다.
    // 하지만 이를 사용하려면 명시적인 동의가 필요하다. `#[derive(Debug)]` 라는 선언을 구조체 위에 해줘야 한다. (1행 참고)
    // 그 결과 `rect2 is Rectangle { width: 30, height: 50 }` 의 형태로 출력이 된다.
    // 필드가 더 많은 구조체라면 이보다 읽기 편한 형태가 필요할 것인데, 이를 가능하게 하려면 {:?} 대신에 {:#?}을 넣어주면 된다.
    // rect2 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    // 이제 println!을 찍으면 이와 같은, 좀더 보기 좋은 형태로 출력되게 된다.

    {
        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                // self의 width가 비교대상의 width보다 크고, self의 height가 비교대상의 height보다 클 경우 true
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        // rect1 면적에 완전히 들어갈 수 있는지를 검사하는 `can_hold` 함수를 사용
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    // `impl` 블록 내에 구현된 모든 함수를 연관함수(associated function)라고 부르는데, 이는 `impl` 뒤에 나오는 타입과 모두 연관된 함수이기 때문이다.
    // 동작하는 데 해당 타입의 인스턴스가 필요하지 않다면 `self`를 첫 매개변수로 갖지 않는(즉 메소드가 아닌) 연관 함수를 정의할 수도 있다.
    // 메소드가 아닌 연관 함수는 구조체의 새 인스턴스를 반환하는 '생성자'로 자주 활용된다.
    // 아래에는 size를 받아서 size만큼의 width와 height를 가진 Rectangle 구조체의 새 인스턴스를 반환하는 `_square` 함수이다.
    {
        impl Rectangle {
            fn _square(size: u32) -> Self {
                Self {
                    // 여기서의 Self는 `Rectangle`이 된다.
                    width: size,
                    height: size,
                }
            }
        }
        let _sq = Rectangle::_square(3); // 이런 식으로 `String::from()`을 쓸 때처럼 구조체 명에 `::` 구문을 붙여서 호출한다.
    }

    // 각 구조체는 여러개의 impl 블록을 가질 수 있다.
    // 아래 코드에서는 물론 `impl` 블록을 여러 개로 나눠야 할 이유가 전혀 없다.
    {
        impl Rectangle {
            fn _area(&self) -> u32 {
                self.width * self.height
            }
        }

        impl Rectangle {
            fn _can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
    }
}

// 현재 area 함수는, 이 `width`와 `height`라는 두개의 파라미터가 서로 연관되어 있다는 것을 명확하게 표현하는 부분은 찾아볼 수 없다. 이 두 개를 튜플로 묶어볼 수 있다.
// fn area(width: u32, height: u32) -> u32 {
fn area(dimensions: (u32, u32)) -> u32 {
    // 튜플로 리팩토링했으나, 아래의 코드는 사실 0번 인덱스가 width고 1번 인덱스가 height라는 사실을 기억하고 있어야 한다.
    // 어차피 너비는 width * height기 때문에 두 순서가 바뀌어도 상관은 없으나, 다른 경우라고 한다면 인덱스에 어떤 값이 있는지를 기억해야 하기 때문에 좋지 않다.
    dimensions.0 * dimensions.1
}

// 이렇게 구조체로 리팩토링하니, 각각이 가진 문제점을 모두 해결할 수 있게 되었다. 어떤 값과 어떤 값을 곱하는지 명확해졌다.
// 타입이 &Rectangle일 수밖에 없는 이유는, 구조체의 소유권을 가져오면 main에서 정작 사용할 수가 없게 되어버리기 때문이다.
fn area_with_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 정리
// 1. 구조체를 사용할 경우, 도메인에 의미있는 커스텀 타입을 만들 수 있다.
// 2. 구조체를 사용함으로써 관련있는 데이터들을 하나로 묶어 관리할 수 있으며, 각 데이터 조각에 이름을 붙여 더 명확하게 만들 수 있다.
// 3. `impl` 블록 내에서는 타입에 대한 연관 함수들, 그리고 연관 함수의 일종인 메소드를 정의하여 구조체 인스턴스가 가질 동작들을 명시할 수 있다.
