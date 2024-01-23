mod panic_or_not_to_panic;

use std::{
    fs::{self, File},
    io::ErrorKind,
};

fn main() {
    // panic!("crash and burn");
    // thread 'main' panicked at src/main.rs:2:5: crash and burn
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // let v = vec![1, 2, 3];
    // v[99];
    // `panic!`을 일으키는, 벡터의 끝을 넘어서는 요소에 대한 접근 시도.
    // thread 'main' panicked at src/main.rs:7:6: index out of bounds: the len is 3 but the index is 99
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let greeting_file_result = File::open("hello.txt");
    // 파일을 여는 코드. `File::open`의 반환 타입은 Result<T, E>이다.
    // 제네릭 매개변수 T는 `File::open`의 구현부에 성공 값인 파일 핸들 `std::fs::File`로 채워져 있다. 에러 값에 사용된 E의 타입은 `std::io::Error`이다.
    // 이 `File::open` 함수는 예를 들어 해당 파일이 존재하지 않는 경우에는 실패할 수도 있다. 이렇게, 가능성을 알려주면서도 파일 핸들 혹은 에러 정보를 제공할 방법이 필요한데, 이 일을 `Result` 이넘이 한다.
    // unwrap으로 감쌀 수도 있는데, 만약 Result 값이 Ok라면 unwrap은 Ok를 반환하고, Err라면 panic! 매크로를 호출할 것이다.

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
        // Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" } 에러 리턴. 실제로 파일이 없으므로.
    };
    {
        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => {
                match error.kind() /* io::Error 구조체가 제공하는 `kind` 메소드를 호출하여 `io::ErrorKind` 값을 얻을 수 있다.*/{
                ErrorKind::NotFound => match File::create("hello.txt") { // file create까지 실패할 수 있으므로, 내부 match 표현식의 두번째 갈래(Err) 또한 작성해야 한다.
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
            }
        };
    }
    {
        // 위의 match문과 같은 역할을 하지만, match 표현식을 사용한 것보다 더 깔끔하게 읽힌다.
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file {:?}", error);
                })
            } else {
                panic!("Problem opening the file {:?}", error);
            }
        });
    }
    {
        let greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in this project");
        // 좋은 에러 메시지를 제공한다면, 개발자의 의도를 전달하면서 패닉의 근원이라는 것을 추적하기 쉽게 해준다.
    }
    {
        // 실패할 수도 있는 무언가를 호출할 때, 이 함수에서 에러를 처리하는 대신 이 함수를 호출하는 코드 쪽으로 에러를 반환하여 그쪽에서 수행할 작업을 결정하도록 할 수 있다(에러 전파하기).
        use std::io::{self, Read};

        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file, // File::open이 성공하면 패턴 변수 `file`의 파일 핸들은 가변 변수 `username_file`의 값이 된다.
                Err(e) => return Err(e), // 함수 전체를 일찍 끝내고, 호출한 코드 쪽에 `File::open`으로부터 얻은(지금의 경우 패턴 변수 e에 있는) 에러값을 이 함수의 에러값처럼 넘긴다.
                                         // 그렇다면 username_file은 결과적으로 "파일"을 갖게 되거나, Error를 갖게 된다.
            };

            let mut username = String::new(); // username_file에 파일 핸들을 얻게 되면 함수는 `username` 변수에 새로운 String을 생성함

            match username_file.read_to_string(&mut username) /* 파일의 내용물을 `username`으로 읽어들인다. read_to_string()이 실패할 수도 있으므로 match 형태를 띤다. */ {
                Ok(_) => Ok(username),
                Err(e) => return Err(e), // 마지막 표현식이므로 굳이 `return`을 명시적으로 적을 필요는 없다. 헷갈릴 수도 있으니 적는 것도 좋을듯.
            }
        }

        // 이 코드를 호출하는 코드는, 사용자 이름이 있는 `Ok` 값 혹은 `io::Error`를 담은 `Err` 값을 처리하게 될 것이다. 이 값을 갖고 어떤 일을 할지에 대한 결정은 호출하는 코드 쪽에 달려있다.
        // 만일 그쪽에서 `Err` 값을 얻었다면, 이를테면 `panic!`을 호출하여 프로그램을 종료시키는 선택을 할 수도 있고, 기본 사용자 이름을 사용할 수도 있고, 파일이 아닌 다른 어딘가에서 사용자 이름을 찾을 수도 있다.
        // 호출하는 코드가 정확히 어떤 것을 시도하려 하는 지에 대한 충분한 정보가 없기 때문에, 모든 성공 혹은 에러 정보를 위로 전파하여 호출하는 코드가 적절하게 처리하도록 한다.
    }
    {
        use std::fs::File;
        use std::io::{self, Read};

        {
            fn read_username_from_file() -> Result<String, io::Error> {
                let mut username = String::new();
                // let mut username_file = File::open("hello.txt")?; // 여기의 "?"는 Ok의 값을 변수 username_file에게 반환해 줄 것이다. 에러가 발생하면 "?"는 함수로부터 일찍 빠져나와 호출하는 코드에게 어떤 Err 값을 줄 것이다.
                // username_file.read_to_string(&mut username)?;
                File::open("hello.txt")?.read_to_string(&mut username)?;
                // 91, 92행의 두 줄은 93행의 한 줄로 변경할 수 있다.
                Ok(username)
            }
            // 위의 코드와 같은 기능을 가진 read_username_from_file의 구현체인데, "?" 연산자를 이용한다.
            // "?"은 위의 match 표현식과 거의 같은 방식으로 동작하게끔 정의되어 있다. Result의 값이 Ok라면 Ok 안의 값이 얻어지고 Err라면 return 키워드로 에러 값을 호출하는 코드에게 전파하는 것처럼 Err의 값이 반환될 것이다.
            // "?" 연산자를 사용할 때의 에러 값들은 `from` 함수를 거친다. "?" 연산자가 from 함수를 호출하면, "?" 연산자가 얻게 되는 에러를 "?" 연산자가 사용된 현재 함수의 반환 타입에 정의된 에러 타입으로 변환한다.
            //
        }
        {
            fn read_username_from_file() -> Result<String, io::Error> {
                fs::read_to_string("hello.txt")
            }
            // 파일을 열고 읽는 대신 `fs::read_to_string`을 사용한 예시이다. 파일에서 문자열을 읽는 코드는 굉장히 흔하게 사용되기 때문에, 표준 라이브러리에서는
            // 1. 파일을 열고
            // 2. 새 String을 생성하고
            // 3. 파일 내용을 읽고
            // 4. 내용을 String에 집어넣고 반환
            // 하는 `fs::read_to_string`라는 편리한 함수를 제공한다.
        }
        {
            // "?"는 "?"이 사용된 값과 호환 가능한 반환 타입을 가진 함수에서만 사용될 수 있다(match와 동일한 방식이기 때문). 함수의 반환 타입이 Result여야 이 return과 호환이 가능하다.
            fn main() {
                let greeting_file = File::open("hello.txt")?;
                // 이 코드는 파일을 열고, 이는 실패할 수도 있다. "?" 연산자는 `File::open`에 의해 반환되는 `Result` 값을 따르지만, `main` 함수는 반환 타입이 `Result`가 아니라 `()`이다. 이 코드를 컴파일할 경우 아래의 메시지를 얻게 된다.
                // the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
                // the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
                // 이 에러는, "?" 연산자는 `Result`,`Option` 혹은 `FromResidual`을 구현한 타입을 반환하는 함수에서만 사용될 수 있음을 지적하고 있다. 이를 해결하는 방법은 두가지이다:
                // 1. "?" 연산자가 사용되는 곳의 값과 호환되도록 함수의 반환 타입을 수정하기: 이러한 수정을 막는 제약 사항이 없는 한에서 가능하다.
                // 2. `Result<T, E>`를 적절한 식으로 처리하기 위해 `match` 혹은 `Result<T, E>`의 메소드 중 하나를 사용하기.

                // 이 에러는 "?" 연산자가 `Option<T>` 값에 대해서도 사용될 수 있다는 점 또한 알려주는데, 함수가 `Option`을 반환하는 경우에는 `Option`에서만 "?"를 사용할 수 있다.
                // `Option<T>`에서 호출되었을 때의 "?" 연산자 동작은 `Result<T, E>`에서 호출되었을 때의 동작과 비슷하다:
                // 1. `None`의 경우 그 함수의 해당 지점으로부터 `None` 값을 일찍 반환한다.
                // 2. `Some` 값이라면, `Some` 안에 있는 값이 이 표현식의 결괏값이 되면서 함수가 계속된다.

                fn last_char_of_first_line(text: &str) -> Option<char> {
                    text.lines().next()?.chars().last()
                    // 이 함수는 주어진 텍스트에서 첫 번째 줄의 마지막 문자를 찾는 함수의 예시인데, 이 함수는 문자가 있을 수도 없을 수도 있기 때문에 `Option<char>`를 반환한다.
                    // text 문자열 슬라이스를 가져와서 `lines` 메소드를 호출하는데, 이는 해당 문자열의 라인에 대한 반복자를 반환한다.
                    // lines() 까지에 대한 검사가 필요하므로 반복자 `next`를 호출하여 첫번째 값을 얻어온다.
                    // 만일 text가 빈문자라면 `next` 호출은 `None`을 반환하고, "?"를 사용하여 `last_char_of_first_line`의 실행을 멈추고 `None`을 반환한다.
                    // 빈문자가 아니라면 `next`는 text의 첫 번째 줄의 문자열 슬라이스를 담고 있는 `Some`의 값을 반환한다.
                }
            }
        }
        {
            // `Result`와 `Option` 각각을 반환하는 경우에 "?" 연산자를 사용할 수 있지만, 섞어서 사용할 수 없다는 것을 주목해야 한다.
            // "?" 연산자는 자동으로 Result를 Option으로 변환하거나 혹은 그 반대를 할 수 없다; 그러한 경우 Result의 `ok` 메소드 혹은 Option의 `ok_or` 메소드 같은 것들을 통해 명시적으로 변환을 할 수 있다.
            // 지금껏 main은 무조건 `()`를 반환했는데, 이는 `main` 함수의 특별함(실행 프로그램의 시작점이자 종료점) 때문이고 프로그램이 기대한 대로 동작하려면 반환 타입의 종류에 대한 제약사항이 있다.
            // 다행히도 main은 `Result<(), E>`도 반환할 수 있다.
            use std::error::Error;

            fn main() -> Result<(), Box<dyn Error>> {
                let greeting_file = File::open("hello.txt")?;

                Ok(())
                // 지금은, Box<dyn Error>는 "어떤 종류의 에러"를 의미한다고만 보면 될 것 같다.
                // 반환할 에러 타입이 Box<dyn Error>라고 명시하면, 이후 main의 구현체에 다른 에러들을 반환하는 코드가 추가되더라도 계속 올바르게 작동할 것이다.
                // main 함수가 `Result<(), E>`를 반환하게 되면 실행 파일은 `main`이 `Ok(())`를 반환할 경우 0 값으로 종료되고, `main`이 `Err` 값을 반환할 경우 0이 아닌 값으로 종료된다.
            }
        }
    }
}
