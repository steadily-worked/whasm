use std::io;

fn main() {
    let mut x = 5; // let으로 선언한 경우 재할당 불가. 자바스크립트의 const와 같음
    println!("The value of x is: {x}");
    x = 6; // x가 mut 변수가 아닌 경우 cannot assign twice to immutable variable `x` 에러
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const(상수)에는 type annotation이 필수임. 네이밍 컨벤션: THIS_IS_CONSTANT
    println!("{THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
        // shadowing: 중괄호 내부에서만 y의 값에 덮어씌워짐
        // 변수를 mut로 표시하는 것과는 다름. 변형 완료 후에는 변수가 변경되지 않도록 할 수 있음.
    }

    println!("The value of y is: {y}");

    let _spaces = "      ";
    let _spaces = _spaces.len();
    // shadowing: let 키워드를 사용해서 재할당을 하는 경우 이름은 동일하게 쓰지만 타입은 변경할 수 있음.
    // mut: 기존의 타입을 변경할 수 없음.

    types(5, 'h');
    find_index();
    statements_and_expressions();
    // main 함수가 실행된 후에 main 함수에 나타나는 순서대로 함수가 실행됨.
}

fn types(value: i32, unit_label: char) {
    println!("x: {value}, unit_label: {unit_label}");
    // 단일 값을 나타태는 scalar 타입: integer, floating-point numbers, Booleans, characters.
    // signed: 부호있음(정수), unsigned: 부호없음(양수).
    // signed: -2^(n-1) ~ 2^(n-1)-1의 범위를 가짐. ex) `i32``: `-2^31` ~ `2^31-1`. 정수의 기본값은 `i32`
    // 자바스크립트와 같이 1000을 1_000으로 표현할 수 있음.

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // 0을 기준으로 가장 가까운 정수로 잘라냄. -1.66 -> -1. 그래서 이 타입이 `i32`임.

    // remainder
    let _remainder = 43 % 5;

    // Boolean
    let _t = true;
    let _f: bool = false; // type annotation

    // char: 작은 따옴표로 지정함. 큰따옴표는 `&str` 타입
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    // tuple: 길이가 고정되어있어 한 번 선언하면 크기가 커지거나 줄어들지 않음.
    let tup: (i32, f64, u8) = (500, 6.4, 1); // _tup은 전체 튜플에 바인딩됨.
    let (_x, _y, _z) = tup; // 자바스크립트의 구조분해할당처럼 사용할 요소를 불러와서 사용할 수 있음. 변수명은 아무거나 해도 되지만, 순서대로 값이 할당됨.

    let _five_hundred = tup.0; // 자바스크립트의 tup[0]처럼 인덱스로 접근할 때 `.`을 사용함
    let _six_point_four = tup.1;
    let _one = tup.2;

    // array: 다른 언어들과 다르게 Rust의 array는 길이가 고정되어있음.
    // Vector 타입은 길이를 조절할 수 있음. 보통 둘 중에 뭘 사용할지 모르겠다면 Vector를 사용하면 되지만, 길이 조절의 가능성이 없는 경우엔 Array가 더 유용함.
    let a: [i32; 5] = [3; 5]; // 이 경우 [3, 3, 3, 3, 3]의 Array가 생성됨
    let _first = a[0];
    let _second = a[1];
}

fn find_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}")
}

fn statements_and_expressions() {
    fn five() -> i32 {
        5 // five func의 return값. 5의 타입이 i32이므로 func의 타입도 i32임
    }
    fn plus_one(x: i32) -> i32 {
        return x + 1; // 혹은 x + 1.  세미콜론을 붙여서 표현식으로 만들려면 반드시 return문과 함께해야함.
    }

    let _y = 6; // statement는 value를 리턴하지 않고 특정한 액션을 취함. expressions(표현식)는 값으로 평가됨.
    let z = {
        let x = 3;
        x + 1 // 표현식에는 세미콜론을 붙이지 않으며, 세미콜론을 붙일 경우 statement의 형태를 취하고, 값을 리턴하지 않게 됨.
    };
    let _five = five();
    let five_plus_one = plus_one(_five);
    println!("z: {z}, five: {_five}, five + one = {five_plus_one}")
}
