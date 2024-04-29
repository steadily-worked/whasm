fn main() {
    매치();
    옵션_사용_매치();
    포괄패턴();
    if_let();
}

fn 매치() {
    #[derive(Debug, PartialEq)]
    enum 나라 {
        프랑스,
        스페인,
    }

    // 일련의 패턴에 대해 어떤 값을 비교한 뒤, 어떤 패턴에 매칭되었는지를 바탕으로 코드를 수행하도록 해준다.
    enum 동전 {
        원,
        달러,
        엔화,
        유로(나라),
    }

    fn 각_화폐의_가치를_원화로(화폐: &동전) -> u16 {
        match 화폐 {
            동전::원 => {
                println!("원입니다.");
                1
                // 두 줄 이상 들어갈 때는 중괄호를 넣어야 함.
            }
            동전::달러 => 1380,
            동전::엔화 => 9,
            동전::유로(나라) if 나라 == &나라::프랑스 => {
                println!("state from {:?}", 나라);
                1474
            }
            동전::유로(나라) if 나라 == &나라::스페인 => {
                println!("state from {:?}", 나라);
                1475
            }
            동전::유로(_) => 1477,
        }
        // if 사용 시 조건문에서 boolean을 반환해야하지만, 여기서는 무엇이든 가능.
        // 원이 아니라면 달러로, ... 이렇게 다음 갈래로 이동
    }
    각_화폐의_가치를_원화로(&동전::유로(나라::프랑스)); // state from 프랑스
}

fn 옵션_사용_매치() {
    fn 하나_더하기(숫자: Option<i32>) -> Option<i32> {
        match 숫자 {
            Some(k) => Some(k + 1),
            None => None,
        }
    }
    let 숫자_5 = Some(5);
    let 숫자_6 = 하나_더하기(숫자_5);
    let 논값 = 하나_더하기(None);
    println!("{:?}", 논값);
    fn 하나_더하기_2(숫자: Option<i32>) -> Option<i32> {
        match 숫자 {
            Some(i) => Some(i + 1),
            None => todo!(), //안넣을 시 에러. 모든 가능한 경우를 다루지 않았기 때문.
        }
    }
    let 논값2: Option<i32> = 하나_더하기_2(None);
    println!("{:?}", 논값2); // todo! 안해놓으면 터짐
}

fn 포괄패턴() {
    let 숫자 = 3;
    match 숫자 {
        1 => println!("1입니다."),
        2 => println!("2입니다."),
        3 => println!("3입니다."),
        _ => println!("1, 2, 3이 아닙니다."), // 1~3이 아닌 경우 기본 동작 취하기. 순차적으로 평가되므로 마지막에 둬야함
                                              // other =>, _ => 둘 다 가능
                                              //   _ => () 의 형태로, 아무 동작도 하지 않게 할 수 있음.
    }
}

fn if_let() {
    // if let은, if와 let을 조합하여, 하나의 패턴에만 매칭시키고 나머지는 무시하도록 값을 처리하는 방법을 제공.
    let 숫자 = Some(3);
    match 숫자 {
        Some(i) => println!("숫자는 {}입니다.", i),
        _ => (),
        // 어떤 값이 Some일 때만 코드를 실행하도록 함.
    }
    // 위의 케이스는, 굳이 match를 쓰지 않고 if let으로 쓸 수 있음.
    if let Some(i) = 숫자 {
        println!("숫자는 {}입니다.", i);
    } else {
        println!("숫자가 아닙니다.");
        // else도 쓸 수 있음. match로 쓰기에 너무 장황한 경우 if let을 사용할 수 있음.
    }
}
