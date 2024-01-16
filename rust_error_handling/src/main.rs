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
}
