fn main() {
    let _v = vec![1, 2, 3];
    // `i32` 타입의 값을 가질 수 있는 비어있는 새 벡터 생성
    // `vec!` 매크로를 사용할 경우, 자동으로 제공된 값들을 저장한 `Vec`를 생성해준다. 그렇기에 타입 명시도 필요없다.
    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    println!("v2 is {:?}", v2); // v2 is [5, 6, 7, 8]

    {
        let v = vec![1, 2, 3, 4, 5];

        // 인덱싱 방식
        let third: &i32 = &v[2];
        println!("The third element is {third}");

        // get 메소드 사용
        let third: Option<&i32> = v.get(2);
        // `get` 함수에 인덱스를 파라미터로 넘기면, `match`를 통해 처리할 수 있는 `Option<&T>`를 얻게 된다.
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }
        // let does_not_exist = &v[100]; // 패닉을 일으킴. index out of bounds: the len is 5 but the index is 100
        // let does_not_exist = v.get(100); // `None`을 반환함.
        // println!("does not exist {:?}", does_not_exist);
    }
    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6);
        println!("The first element is: {first}"); // cannot borrow `v` as mutable because it is also borrowed as immutable
                                                   // 아이템의 참조자를 갖고 있는 상태에서 벡터에 새로운 요소를 추가하려고 시도할 땐, 에러가 발생한다.
                                                   // 벡터 끝에 새로운 요소가 추가될 때 메모리 위치에 공간이 없다면 새로운 곳에 메모리를 할당하고 요소들을 복사한다. 기존 요소의 참조자는 해제된 메모리를 가리키므로 이러한 상황을 막은 것이다.
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        } // 단지 접근만 하기 위한 용도의 반복문
        for i in &mut v {
            *i += 50;
        } // mutable한 v의 각 요소인 i에 50씩을 더해주는, 읽고 쓰는 용도의 반복문. 이경우 당연히 mut 처리를 해줘야 한다.
          // 가변 참조자가 가리키는 값을 수정하려면 `+=` 연산자를 쓰기 전에 `*` 역참조 연산자로 `i`의 값을 얻어야 한다.
    }
    {
        let data = "initial contents";
        let _str = data.to_string(); // 문자열 리터럴을 String 타입으로 바꿔주는 메소드.
        let _str = "initial contents".to_string(); // 이렇게 즉시 바꿀 수도 있다.
    }
    {
        let mut s1 = String::from("hello");
        let s2 = "world";
        s1.push_str(s2);
        println!("s2 is {s2}");
        // 문제없이 컴파일된다. `push_str` 함수가 `s2`의 소유권을 가져가지 않기 때문.
    }
    {
        // 하지만, `+` 연산자를 사용할 경우 소유권을 가져간다.
        let s3 = String::from("hello");
        let s4 = String::from(", world!");
        let _s5 = s3 + &s4; // s3은 여기로 이동되어 더이상 사용할 수 없다.
                            // `+` 연산자는 String 타입에 `&str` 타입을 더하는 것을 의미하는데, 그렇다면 이 코드블럭은 컴파일이 될 수가 없다. 근데 문제없이 컴파일이 되는데, 왜 되는 걸까?
                            // 이는 `&String` 인수가 `&str`로 강제될 수 있기 때문이다.
        let s6 = String::from("hello");
        let s7 = format!("{s6}{s4}"); // format! 매크로는 소유권을 이전하지 않는다.
        println!("{s7}");
    }

    {
        let s1 = String::from("hello");
        let h = s1[0]; // String` cannot be indexed by `{integer}`
    }
}
