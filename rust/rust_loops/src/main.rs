fn main() {
    let mut counter = 0;
    // loop: 명시적으로 끝내는(break) 코드가 없을 경우 무한정 지속됨.
    // continue: continue 아래의 코드를 무시하고 다음 반복으로 넘어가도록 함. Python의 continue와 동일
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break과 동시에 연산이 가능함.
        }
    };

    println!("The result is: {result}");

    // multiple loops. disambiguate by labeling `'counting_up`
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // outer loop을 끝내버림. inner loop에서 설정할 수 있음
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while: 위의 loop의 if, else, break 조건을 일반적으로 사용하게 하기 위한 방법
    let mut number = 3;

    while number != 0 {
        // number == 0일 때 break을 걸어버림. while문 뒤에는 조건이 필요함. 이 조건이 false가 될 때 종료
        println!("{number}!");

        number -= 1;
    }
    println!("lift off");

    // for의 collection을 통한 loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        // index < 5와 같은 특정한 조건으로 반복문을 돌리는 건 좋은 방법이 아님. 컴파일러가 루프를 반복할 때마다 인덱스가 배열의 범위 내에 있는지
        // 조건부 검사를 수행하기 위해 런타임 코드를 추가하므로, 속도가 느려짐.

        index += 1;
    }
    // 위의 while문을 아래와 같이 바꿀 수 있음. 자바스크립트의 for of와 유사
    for element in a {
        println!("the value is: {element}");
        // 이 경우 element는 무조건 a 배열에 있음을 보장하므로 배열의 길이보다 길게 진행되거나, 짧게 진행되고 끝나는 등의 문제를 해결할 수 있음.
    }

    for number in (1..4).rev() {
        // rev: reverse. 4-1부터 1까지 진행. Python과 같다.
        println!("{number}!");
    }
    println!("lift off");
}
