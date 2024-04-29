use rand::Rng; // Cargo.toml에 추가해야함.
               // std 표준 라이브러리도 마찬가지로 외부 크레이트지만, 러스트 언어에 포함되어있으므로 Cargo.toml에 추가할 필요가 없다.
               // 그래도, 여전히 `use std::collections::HashMap;` 이런식으로 사용해야한다.

// use std::assert;
// use std::cmp::Ordering;
// 이 두가지는 아래의 한가지로 바꿀 수 있다.
use std::{assert, cmp::Ordering};

// use std::io;
// use std::io::Write;
// 이 두가지는 아래의 한가지로 바꿀 수 있다. 그 자체는 self가 된다.
// 근데, 왜 그 자체를 불러오면서 Write도 불러오는걸까? 이런 케이스가 많을까?
use std::io::{self, Write};

// use std::collections::*;
// 경로 내에 정의된 모든 공개 아이템을 가져올 수 있다.
// but, 어떤 아이템이 어디에 정의되었는지 파악하기 어렵게 만들 수 있다.

fn main() {
    let 비공개_숫자 = rand::thread_rng().gen_range(1..=100);
}
