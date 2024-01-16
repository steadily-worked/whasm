use std::{fs::File, io::ErrorKind};

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
        let greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in this project");
    }
    {
        use std::io::{self, Read};

        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file, // File::open이 성공하면 패턴 변수 `file`의 파일 핸들은 가변 변수 `username_file`의 값이 된다.
                Err(e) => return Err(e), // 함수 전체를 일찍 끝내고, 호출한 코드 쪽에 `File::open`으로부터 얻은(지금의 경우 패턴 변수 e에 있는) 에러값을 이 함수의 에러값처럼 넘긴다.
            };

            let mut username = String::new(); // username_file에 파일 핸들을 얻게 되면 함수는 `username` 변수에 새로운 String을 생성함

            match username_file.read_to_string(&mut username) /* 파일의 내용물을 `username`으로 받아들인다. read_to_string()이 실패할 수도 있으므로 match 형태를 띤다. */ {
                Ok(_) => Ok(username),
                Err(e) => Err(e), // 마지막 표현식이므로 굳이 `return`을 명시적으로 적을 필요는 없다.
            }
        }

        // 이 코드를 호출하는 코드는, 사용자 이름이 있는 `Ok` 값 혹은 `io::Error`를 담은 `Err` 값을 처리하게 될 것이다. 이 값을 갖고 어떤 일을 할지에 대한 결정은 호출하는 코드 쪽에 달려있다.
        // 만일 그쪽에서 `Err` 값을 얻었다면, 이를테면 `panic!`을 호출하여 프로그램을 종료시키는 선택을 할 수도 있고, 기본 사용자 이름을 사용할 수도 있고, 파일이 아닌 다른 어딘가에서 사용자 이름을 찾을 수도 있다.
        // 호출하는 코드가 정확히 어떤 것을 시도하려 하는 지에 대한 충분한 정보가 없기 때문에, 모든 성공 혹은 에러 정보를 위로 전파하여 호출하는 코드가 적절하게 처리하도록 한다.
    }
}
