fn main() {
    let mut 가변변수 = 5;
    println!("The value of x is: {가변변수}");
    가변변수 = 6;
    println!("The value of x is: {가변변수}");

    const 항상_불변인_상수: u32 = 60 * 60 * 3;
    // 상수는 선언된 스코프 내에서, 프로그램이 동작하는 전체시간동안 유효함.

    let 섀도잉 = 5;
    let 섀도잉 = 섀도잉.to_string() + "3";
    println!("섀도잉의 현재 값은: {섀도잉}");
    // 여기서, let으로 다시 선언해줌으로써 섀도잉이 발생하고, 변수 "섀도잉"의 값이 5(i32)에서 53(String)으로 변함.
    // let mut로 선언해주는 것과는 다른데, let mut로 선언해주면 변수의 값은 변경할 수 있지만, 변수의 타입은 변경할 수 없다.

    // let 타입_없는_변수 = "42".parse().expect("not a number!"); // 타입 명시 필수 에러 발생

    let 양수_혹은_음수_8제곱_까지: i8 = -5;
    let 양수만_가능_8제곱_까지: u8 = 5;

    let 다섯개_배열 = [3; 5];

    let 한글자: char = 'G';
    let 읽기_전용_변수: &str = "Hello world";
    // &str 은 섀도잉은 가능하나 수정할 수는 없다

    let 가변적인_변수: String = String::new();

    let 조건 = true;
    let 조건에_따른_값 = if 조건 { "good" } else { 6 }; // if 조건문은 반드시 else를 가지고 있어야 하며, 갈래 값의 타입이 같아야한다.

    // let mut a: String = "Sangmin".to_string();
    // let mut capture_a = || println!("{}", a);

    // {
    //     capture_a();
    // }

    // a.push_str("the great");
    // capture_a();
}
