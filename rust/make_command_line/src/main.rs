use std::{env, error::Error, fs}; // analyzer에서 하는대로. 유효하지 않은 유니코드를 포함한 파일명을 다루기 위해서는 std::env::args_os를 사용해야 한다.

fn main() {
    인수_받고_저장하고_파일_읽기();
    config_해보는_함수();
    run_써보기();
    lib_에서_가져온_것_사용하기();
}

fn 인수_받고_저장하고_파일_읽기() {
    // cargo run -- searchstring example-filename.txt 이러한 형태로 파일을 찾는 것을 만들 것.
    let args: Vec<String> = env::args().collect(); // minigrep으로 넘겨진 커맨드 라인 인수의 반복자(iterator)를 반환하는데, 여기서는 collect를 사용하여 이 반복자를 벡터로 변환한다.
                                                   // 러스트에서는 타입 명시의 필요가 거의 없지만, 러스트가 내가 원하는 종류의 컬렉션을 추론할 수는 없으므로, collect 는 타입 표기가 자주 필요한 함수 중 하나이다.
    dbg!(&args); // 출력값: [커맨드라인_프로그램_만들기/src/main.rs:25:5] &args = [
                 // "/Users/sangminpark/Desktop/whasm/rust/target/debug/커맨드라인_프로그램_만들기",
                 // ]
    println!("{:?}", &args); // 출력값: ["/Users/sangminpark/Desktop/whasm/rust/target/debug/커맨드라인_프로그램_만들기"]
                             // 바이너리 파일의 이름이 출력된다. 이는 C에서의 인수 리스트의 동작과 일치하며, 프로그램이 실행될 때 호출된 이름을 사용할 수 있게 해준다.

    // cargo run -- needle haystack으로 실행할 경우, 벡터에 기존 출력값 외에도 "needle", "haystack"이 추가된다.
    let query = &args[1];
    let file_path = &args[2];

    // let (query, file_path) = parse_config(&args);
    // let (query, file_path) = parse_config_using_struct(&args);
    // let config = Config::new(&args);
    println!("Searching for {}", query); // Searching for needle
    println!("In file {}", file_path); // In file haystack

    // 인덱스만 잘 알고 있다면, 인수 값들을 변수에 저장할 수 있다.
    // 첫번째 인수는 검색하고자 하는 문자열이므로 query 변수에 참조자를, 두번째 인수는 파일 경로가 될것이므로 file_path 변수에 참조자를 저장한다.

    // 파일 읽기
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{}", contents);
    // run(config);
}

// 인수 파서 추출: 커맨드 라인 파싱 로직을 lib.rs로 옮기기 위한 준비 단계. 함수 추출하기
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
// 여전히 커맨드 라인 인수는 벡터로 모으지만, main 함수 내에서 인덱스 1번의 인수 값을 query 변수에 할당하고 2번 인수값을 file_path에 할당하는 대신
// 전체 벡터를 parse_config 함수에 넘긴다. 이제 parse_config 함수는 어떤 인수 값이 어떤 변수에 들어갈지 정하는 로직을 담고있고, 이 값들을 main에게 다시 넘겨준다.
// 여전히 query와 file_path 변수는 main 안에서 만들지만 main은 더이상 커맨드라인 인수와 변수들이 어떻게 대응되는지를 결정할 책임이 없다.
// -> 즉, &args[1] 같은 값으로 직접 지정해 줄 필요가 없다는 것. 함수에서 위치에 따른 값을 지정해서 넘겨주므로.

// 두 값이 연관되어 있다는 사실을, 위의 parse_config 함수에서는 전달하지 못하고 있다. 따라서, 구조체로 묶어줌으로써 이 두 값의 연관성을 명확히 할 수 있다.
struct Config {
    query: String,
    file_path: String,
}

fn parse_config_using_struct(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    // consider cloning the value if the performance cost is acceptable: `.clone()`rustcE0508 -> .clone() 처리
    Config { query, file_path }
}
// 메인 함수의 args 변수는 인수 값들의 소유자이고, parse_config 함수는 이 값을 참조하기만 하므로, parse_config 함수는 args의 소유권을 가져갈 필요가 없다.
// String 데이터를 관리하는 방법은 다양하며 가장 쉬운 방법은 그 값에서 clone 메소드를 호출하는 것이다. -> 데이터의 전체 복사본을 만들어서 Config 인스턴스가 소유할 수 있게 해주는 건데,
// 이는 문자열 데이터의 참조자를 저장하는 것에 비해 더 많은 시간 및 메모리를 소비한다.
// 그렇지만 값의 복제는 참조자의 라이프타임을 관리할 필요가 없어지므로 코드를 매우 직관적으로 만들어 주기도 한다. 이 환경에서 약간의 성능을 포기하고 단순함을 얻는 것은 가치있는 절충안이다.

// Config를 위한 생성자 만들기: 이제 parse_config(_using_struct) 함수의 목적이 Config 인스턴스를 생성하는 것이 되었으므로, parse_config를 일반 함수에서 Config 구조체와 연관된
// new라는 이름의 함수로 바꿀 수 있다. -> 이러한 변경이 코드를 더 자연스럽게 만들어준다. Config 인스턴스의 생성을 `Config::new()`의 호출로 할 수 있다.
// let config = Config::new(&args); 라인으로 이동

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}

// 에러 처리 수정
// args 벡터에 3개보다 적은 아이템이 들어있는 경우 "index out of bounds: the len is 1 but the index is 1" 에러가 발생함
// `cargo run` 만 실행하여 args에 다른 값을 넣지 못하게 되면 위처럼 에러가 발생함

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             // 3개 미만이면 패닉
//             panic!("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         Config { query, file_path }
//     }
// }
// thread 'main' panicked at 'not enough arguments' 에러 발생. 이 메시지가 더 좋음. 원인을 제대로 포함하고 있으므로.

// panic! 대신 Result 반환하기
// 성공한 경우에는 Config를 담고, 에러가 난 경우에는 문제를 설명해 줄 Result 값을 반환시킬 수 있음.
// new -> build로 변경. 왜냐면, 많은 프로그래머가 new 함수가 절대 실패하지 않으리라 예상하기 때문.
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
// build 함수는, 성공한 경우 Config를, 에러가 난 경우 `&'static str`을 갖는 Result를 반환함. 에러값은 언제나 'static 라이프타임을 갖는 문자열 리터럴.
// panic!을 호출하는 대신 Err 값을 반환하며, 반환 값 Config를 Ok로 감쌌음. 이러한 변경점이, 함수의 새로운 타입 시그니처에 맞도록 함.
// Config::build 로부터 Err 값을 반환하는 것은, main 함수가 build 함수로부터 반환된 Result 값을 처리하여 에러가 난 경우 프로세스를 더 깔끔하게 종료하도록 해줌.

fn config_해보는_함수() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("-------- Config 해보는 함수 ---------");
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
}

// 여기서 unwrap_or_else는 표준 라이브러리의 Result<T, E>에 구현되어있는데, 이를 사용하면 커스터마이징된 panic! 이 아닌 에러 처리를 할 수 있다.
// 만약 Result가 Ok 값이라면 이 메서드의 동작은 unwrap과 유사하다: 즉 Ok가 감싸고 있는 안쪽의 값을 반환한다.
// 하지만 값이 Err 값이라면, 이 메소드는 클로저(closure) 안의 코드를 호출하는데, 이는 unwrap_or_else의 인수로 넘겨준, 우리가 정의한 익명 함수이다.
// 우선 지금은, unwrap_or_else가 Err 내부 값을 클로저의 세로 파이프(||) 사이에 있는 err 인수로 넘겨주는데, 이번 경우 그 값이 "not enough arguments"라는 것만 알면 된다.
// 그러면 실행했을 때 클로저 내의 코드가 err 값을 사용할 수 있게 된다.
// 다 주석처리 후 cargo run 을 해보면 Problem parsing arguments: not enough arguments 만 리턴한다.
// err값만 출력 후 process::exit 을 호출하여, 프로그램을 즉시 멈추고 넘겨진 숫자(1)를 종료 상태코드로서 반환하게 된다.
// panic!을 했을 때와 달리, 추가 출력문이 사라진다.

// main으로부터 로직 추출하기
// fn run(config: Config) {
//     let contents =
//         fs::read_to_string(config.file_path).expect("Should have been able to read the file");
//     println!("With text:\n{}", contents);
// }
// run 함수는 이제 파일을 읽는 부분부터 시작되는, main으로부터 남은 모든 로직을 담고있음. run 함수는 Config 인스턴스를 인수로 취함.
// run 함수로부터 에러 반환하기: 여기서도 에러 처리 기능을 개선할 수 있음.

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);

    Ok(())
}

// Box<dyn Error>는, 이 함수가 Error 트레이트를 구현한 어떤 타입을 반환하는데, 그 반환값이 구체적으로 어떤타입인지 특정하지 않아도 된다는 것을 의미
// "?"을 활용하여 expect의 호출을 제거함. "?"은 에러 상황에서 panic! 대신 호출하는 쪽이 처리할 수 있도록 현재의 함수로부터 에러 값을 반환할 것임.
// 이제 run 함수는 성공하면 Ok를 반환하며, run 함수의 성공 타입은 시그니처 상에서 ()로 선언되었는데, 이는 유닛 타입 값을 Ok로 감쌀 필요가 있다는 의미임.
// Ok(())은 처음에 좀 이상해보일 수 있지만, 이렇게 ()를 사용하는 것은 run을 호출하여 부작용에 대해서만 처리하겠다는 것을 가리키는 자연스러운 방식임 -> 반환값 불필요.
// 코드 실행 시 컴파일은 되지만 경고가 나타남:

// = note: this `Result` may be an `Err` variant, which should be handled
// = note: `#[warn(unused_must_use)]` on by default

// 러스트가 우리에게, Result 값이 무시되고있으며 Result 값이 에러가 발생했음을 나타낼지도 모른다고 알려줌. -> main에서 run으로부터 반환된 에러 처리하기.

fn run_써보기() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("----------- run 써보기 -----------");
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}

// run이 Err 값을 반환했는지 검사하고, 만일 그렇다면 process::exit(1)을 호출하기 위해 사용한 unwrap_or_else 대신 if let이 사용되었음.
// run 함수가 반환한 값("Ok(())")은 Config 인스턴스를 반환하는 Config::build과 동일한 방식대로 unwrap(_or_else)을 하지 않아도 된다.
// run이 성공한 경우 () 를 반환하기 대문에 에러를 찾는 것만 신경쓰면 되므로, 고작 () 가 들어있을 값을 반환하기 위해 unwrap_or_else를 사용할 필요가 없다.

// 이제 Config 구조체, Config 구현체, run 함수를 lib.rs로 옮긴다.
// 이제 main.rs는 다음과 같이 간단해진다.
use make_command_line;
use std::process::exit;

fn lib_에서_가져온_것_사용하기() {
    let args: Vec<String> = env::args().collect();
    let config = make_command_line::Config::build(&args).unwrap();

    if let Err(e) = make_command_line::run(config) {
        println!("Application error: {}", e);
        exit(1);
    }
}
