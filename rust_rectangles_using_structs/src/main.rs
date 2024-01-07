#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
