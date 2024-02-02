use std::{cmp::Ordering, io};
// prelude에 없는 라이브러리의 경우 명시적으로 `use` 키워드와 함께 스코프에 가져와야함.
// std 라이브러리의 io를 사용하겠다.
use rand::Rng;
// std 라이브러리의 Rng를 사용하겠다(난수 생성).

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 범위: start..=end

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // let은 기본적으로 immutable함. let 뒤에 `mut`를 추가하면 변수 변경이 가능함.
        // String: 기본 제공 타입. `::`는, `new`가 String 타입과 관련있음을 나타내기 위함.

        io::stdin()
            // std::io로 라이브러리를 가져오지 않았더라도, `std::io::stdin`으로 사용할 수 있음.
            .read_line(&mut guess)
            // 입력값을 `mut guess` 변수에 저장함. `&`는 이 인수가 참조(reference)임을 나타냄.
            .expect("Failed to read line"); // 프로그램이 충돌을 일으킬 경우 expect 내의 텍스트를 보여줌. 자바스크립트로 따지면 throw new Error("")와 같은 것인듯. expect는 필수값임.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 아래에서 `secret_number`(i32)와 비교하기 위해 `guess` 값을 i32의 형태로 변환.
        // trim(): JS의 trim()과 동일. u32에는 숫자 데이터만 들어갈 수 있으므로 반드시 trim 처리를 해줘야함.
        // i32 vs u32: number type이라는 점은 동일하나, i32는 음수도 가능한 반면 u32는 양수만 가능함.
        // 문자열의 parse: 다른 유형으로 변환(여기선 숫자로: guess의 타입이 u32이므로)

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // cmp: self와 other의 비교. Less, Greater, Equal 세가지 값을 가짐 self: guess, other: secret_number
            // match: 각 요소들을 확인 후 맞아떨어지면 return, 아닐 경우 넘어감.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
