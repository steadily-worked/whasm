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

    let spaces = "      ";
    let spaces = spaces.len();
    // shadowing: let 키워드를 사용해서 재할당을 하는 경우 이름은 동일하게 쓰지만 타입은 변경할 수 있음.
    // mut: 기존의 타입을 변경할 수 없음.
    println!("{spaces}")
}
