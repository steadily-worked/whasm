use crate::garden::vegetables::Asparagus;

pub mod garden; // 컴파일러에게 `src/garden.rs`에 있는 코드를 포함할 것을 알려준다.

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
