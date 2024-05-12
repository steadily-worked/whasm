use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        // env 모듈의 var 함수를 사용. IGNORE_CASE 라는 이름의 환경변수에 어떤 값이 설정되었는지 확인
        // Result 타입이므로, 환경 변수가 설정되어있다면 Ok variant를 반환, 설정되어있지 않다면 Err variant를 반환
        // 아무 값도 없다면 ignore_case는 false가 되며, 대소문자를 구분하는 검색을 수행할 것이다.
        // 환경변수에 대해서는 is_ok 를 사용하여 "값"에 대해서는 고려하지 않고, "값이 있는지 없는지"만 확인한다.

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let 결과 = if config.ignore_case {
        대소문자_구분_안하는_찾기_함수(&config.query, &contents)
    } else {
        성공하는_찾기_함수(&config.query, &contents)
    };

    for 라인 in 결과 {
        println!("{}", 라인);
    }

    Ok(())
}

// 실패하는 테스트 작성하기

// print를 제거하고, test 모듈과 함께 테스트 함수를 추가한다. 테스트 함수는 search 함수가 가져야 할 동작을 지정한다:
// 즉, 쿼리와 컨텐츠를 입력받아서, 컨텐츠로부터 쿼리의 값을 담고 있는 라인들만 반환하는 것.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 대소문자_구분() {
        let 쿼리 = "duct";
        let 컨텐츠 = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            // 실패하는_찾기_함수(쿼리, 컨텐츠)
            성공하는_찾기_함수(쿼리, 컨텐츠)
        );
    }

    #[test]
    fn 대소문자_구분_없음() {
        let 쿼리 = "rUsT";
        let 컨텐츠 = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            대소문자_구분_안하는_찾기_함수(쿼리, 컨텐츠)
        );
    }
}

// 이 테스트는 문자열 duct를 검색한다. 검색하는 텍스트가 세줄인데, 그중 한줄만 duct를 갖고 있다.
// search 함수는 이 한줄만 반환해야 한다. 이 테스트는 search 함수가 제대로 동작하는지 확인한다.
pub fn 실패하는_찾기_함수<'a>(_쿼리: &str, _컨텐츠: &'a str) -> Vec<&'a str> {
    vec![]
}
// TDD 원칙에 따라, 항상 빈 벡터를 반환하는 search 함수 정의부를 추가하여 컴파일과 테스트가 동작하기에 딱 충분한 코드만 집어넣는다.
// 라이프타임 파라미터는, 어떤 인수의 라이프타임이 리턴값의 라이프타임과 연결되는지를 특정한다는 점을 상기해보자.
// 위의 경우 반환된 벡터(Vec<&'a str>)에 인수 contents의 슬라이스를 참조하는 문자열 슬라이스가 들어있음을 나타내고 있다.
// 바꿔말하면 러스트에게 search 함수에 의해 반환된 데이터가 search 함수의 contents 인수로 전달된 데이터만큼 오래 살 것이라는 것을 말해준 것.
// 슬라이스에 의해 참조된 데이터는, 그 참조자가 유효한 동안 유효할 필요가 있다; 만일 컴파일러가 contents 대신 query의 문자열 슬라이스를 만들고 있다고 가정하면
// 안전성 검사는 정확하지 않게 될 것이다.

// 러스트는 두 인수 중 어떤 쪽이 필요한지 알 가능성이 없고, 따라서 이를 명시적으로 말해줄 필요가 있다. contents가 모든 텍스트를 갖고 있는 인수이고, 이 텍스트에서 일치하는 부분을 반환하고 싶은 것이므로
// 라이프타임 문법을 사용해 반환 값과 연결되어야 할 인수는 contents라는 사실을 알고 있다.

// cargo test의 결과, 아래와 같은 리턴값과 함께 실패했다.
// ---- tests::하나의_값 stdout ----
// thread 'tests::하나의_값' panicked at make_command_line/src/lib.rs:45:9:
// assertion `left == right` failed
//   left: ["safe, fast, productive."]
//  right: []
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// 테스트가 통과되도록 코드 작성하기
// 현재는 무조건 빈 벡터만 리턴하므로 테스트가 실패한다. search를 구현하려면 프로그램에서 아래의 단계를 따라야 한다:
// * 컨텐츠의 각 라인에 대해 반복한다.
// * 각 라인이 쿼리의 문자열을 담고있는지 검사한다.
// * 만일 그렇다면, 반환하고자 하는 값의 리스트에 추가한다.
// * 아니라면 아무것도 안한다.
// 매칭된 결과 리스트를 반환한다.

pub fn 성공하는_찾기_함수<'a>(쿼리: &str, 컨텐츠: &'a str) -> Vec<&'a str> {
    let mut 결과 = Vec::new();

    for 라인 in 컨텐츠.lines() {
        // lines 메소드는 반복자를 반환한다.
        if 라인.contains(쿼리) {
            // 라인이 쿼리를 포함하는지 검사한다.
            결과.push(라인);
        }
    }

    결과
}

// 이제 성공했다.

// run 함수에서 search 함수 사용하기
pub fn 대소문자를_구분하는_찾기함수를_사용하는_run(
    config: Config,
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path);

    for line in 성공하는_찾기_함수(&config.query, &contents?) {
        println!("대소문자를_구분하는_찾기함수를_사용하는_run함수의 결과");
        println!("{}", line);
    }

    Ok(())
    // 이제 컨텐츠를 읽어나가면서 찾기 함수를 실행하여 해당 쿼리를 가진 라인들을 출력하게 된다.
    // 없는 단어를 검색하는 경우 아무 줄도 나오지 않는다.
}

// 대소문자를 구분하지 않는 "성공하는_찾기_함수"에 대한 실패하는 테스트 작성하기
pub fn 대소문자_구분_안하는_찾기_함수<'a>(
    쿼리: &str,
    컨텐츠: &'a str,
) -> Vec<&'a str> {
    let 소문자_쿼리 = 쿼리.to_lowercase(); // 소문자화
    let mut 결과들 = Vec::new();

    for 라인 in 컨텐츠.lines() {
        // 컨텐츠를 돌면서
        if 라인.to_lowercase().contains(&소문자_쿼리) {
            // 라인을 전부 소문자화한다음 쿼리를 갖고있는지확인. 여기에서, rust든 rUst든 rUsT든 모두 찾아낼 수 있다.
            결과들.push(라인); // 있으면 push
        }
    }

    결과들
}

pub fn 대소문자_구분_안하는_찾기함수를_사용하는_run(
    config: Config,
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path);

    for line in 대소문자_구분_안하는_찾기_함수(&config.query, &contents?) {
        println!("대소문자_구분_안하는_찾기함수를_사용하는_run함수의 결과");
        println!("{}", line);
    }

    Ok(())
}

// cargo run -- to poem.txt: "to"에 대해서만 출력(소문자 only)
// Are you nobody, too?
// How dreary to be somebody!

// IGNORE_CASE=1 cargo run -- to poem.txt: "to" 에 대해 대소문자 무관 출력
// Are you nobody, too?
// How dreary to be somebody!
// To tell your name the livelong day
// To an admiring bog!

// 이 커맨드라인이 출력하는 내용들이, 현재 표준 에러 쪽에 출력하고 싶은 메시지를 포함하여 어떤식으로 표준 출력에 기록되는지 관찰해보자.
// cargo run > output.txt
