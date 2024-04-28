use crate::garden::vegetables::Asparagus;

pub mod garden;
// 컴파일러에게 `src/garden.rs`의 코드를 모두 포함할 것을 알려줌

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
