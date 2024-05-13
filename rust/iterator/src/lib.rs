use std::env;

#[test]
fn 반복자_구현() {
    let 벡터1 = vec![1, 2, 3];

    let mut 벡터1_반복자 = 벡터1.iter();
    // 벡터1_반복자는 가변으로 만들 필요가 있다: 반복자에 대한 next 메소드 호출은, 반복자 내부의 상태를 변경하여 반복자가 현재 시퀀스의 어디에 있는지 추적함.
    // 즉, 이 코드는 반복자를 소비(consume), 즉 다 써버림.

    assert_eq!(벡터1_반복자.next(), Some(&1));
    assert_eq!(벡터1_반복자.next(), Some(&2));
    assert_eq!(벡터1_반복자.next(), Some(&3));
    assert_eq!(벡터1_반복자.next(), None);
    // next에 대한 각 호출은, 반복자로부터 하나의 아이템을 소비함.
    // 또한, next 호출로 얻어온 값들은 벡터 내의 값들에 대한 불변 참조자이다. iter 메소드는 불변 참조자에 대한 반복자를 생성한다.
    // 만약 벡터1의 소유권을 얻어서 소유한 값을 반환하고싶다면 "iter" 대신 "into_iter"를 사용하면 된다.
    // 비슷하게, 가변 참조자에 대한 반복자가 필요하면 "iter" 대신 "iter_mut"를 사용하면 된다.
}

#[test]
fn 반복자의_합() {
    let 벡터1 = vec![1, 2, 3];
    let 벡터1_반복자 = 벡터1.iter();
    let 총합: i32 = 벡터1_반복자.sum(); // sum()은 반복자를 소유하여 호출하므로, "총합"을 호출한 후에는 "벡터1_반복자" 의 사용이 허용되지 않는다.
    assert_eq!(총합, 6);
}

// 환경으로부터 신발 사이즈를 캡처하는 클로저를 갖고, filter를 사용하여 신발 구조체 인스턴스의 컬렉션을 순회한다. 이는 지정된 크기의 신발만 반환
#[derive(PartialEq, Debug)]
struct 신발 {
    사이즈: u32,
    스타일: String,
}

// 신발 크기에 맞는 사이즈 filter
fn 신발_사이즈(신발: Vec<신발>, 신발_크기: u32) -> Vec<신발> {
    신발.into_iter().filter(|s| s.사이즈 == 신발_크기).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 사이즈에_따른_필터링() {
        let 신발들 = vec![
            신발 {
                사이즈: 10,
                스타일: String::from("스니커즈"),
            },
            신발 {
                사이즈: 13,
                스타일: String::from("샌들"),
            },
            신발 {
                사이즈: 10,
                스타일: String::from("부츠"),
            },
        ];

        let 내_사이즈에_맞는_신발들 = 신발_사이즈(신발들, 10);

        assert_eq!(
            내_사이즈에_맞는_신발들,
            vec![
                신발 {
                    사이즈: 10,
                    스타일: String::from("스니커즈"),
                },
                신발 {
                    사이즈: 10,
                    스타일: String::from("부츠"),
                },
            ]
        )
    }
}
// "신발_사이즈" 함수는, 파라미터로 신발들의 벡터에 대한 소유권과 신발 크기를 받는다.
// 이 함수는, 지정된 크기의 신발들만을 담고 있는 벡터를 반환한다.
// "신발_사이즈" 함수의 본문에서는, "into_iter"를 호출하여 이 벡터의 소유권을 갖는 반복자를 생성한다.
// 그 이후 filter를 호출하여 앞의 반복자를 새로운 반복자로 바꾸는데, 새로운 반복자에는 클로저가 true를 반환하는 요소들만 담겨있게 됨.
// 클로저는 환경에서 "신발_사이즈" 파라미터를 캡처하고, 각 신발의 크기와 값을 비교하여 지정된 크기의 신발만 유지하도록 함.
// 마지막으로 "collect"를 호출하여 적용된 반복자에 의해 반환된 값을 벡터로 모으고, 이 벡터가 함수에 의해 반환됨.
// -> 이 테스트는, "신발_사이즈" 를 호출했을 때 지정된 값과 동일한 크기의 신발들만 돌려받는다는 것을 보여줌.

// 커맨드라인 프로젝트 개선하기
// 이제, 비효율적인 clone을 지울 수 있게 되었음.
pub struct Config {
    pub 쿼리: String,
    pub 파일경로: String,
    pub 케이싱_무시: bool,
}

impl Config {
    pub fn build_before_iterator(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("argument 수가 부족합니다.");
        }

        let 쿼리 = args[1].clone();
        let 파일경로 = args[2].clone();

        let 케이싱_무시 = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            쿼리,
            파일경로,
            케이싱_무시,
        })
    }

    pub fn build_after_iterator(
        mut args: impl Iterator<Item = String>,
        // 반복자를 받도록 Config::build의 시그니처 업데이트. 이제 args 파라미터는 `&[String]` 대신 트레이트 바운드 `impl Iterator<Item = String>`를 받음.
        // args의 소유권을 가져와서, 이를 순회하면서 args를 변경할 것이므로 args 파라미터의 명세 부분에 mut 키워드(가변처리)를 추가하였음.
    ) -> Result<Config, &'static str> {
        args.next(); // env::args()의 첫번째 반환값은 프로그램의 이름이므로, 넘어간 것.

        let 쿼리 = match args.next() {
            Some(arg) => arg, // next가 Some을 반환하면, match를 사용하여 값을 추출하고
            None => return Err("쿼리 문자열을 찾을 수 없습니다."), // 만약 None을 반환하면, 이는 충분한 인수가 넘어오지 않았음을 의미하고, Err 값과 함께 일찍 반환함.
        };

        let 파일경로 = match args.next() {
            Some(arg) => arg,
            None => return Err("파일 경로를 찾을 수 없습니다."),
        };

        let 케이싱_무시 = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            쿼리,
            파일경로,
            케이싱_무시,
        })
    }
}
// 여기에서, String 요소들의 슬라이스를 args 매개변수로 받았으나 build 함수가 args를 소유하지 않기때문에 clone이 필요했다.
// 반복자에 대한 새로운 지식을 사용하면 인수로써 슬라이스를 빌리는 대신 반복자의 소유권을 갖도록 build 함수를 변경할 수 있다.

pub fn 반복자_사용_이전_검색<'a>(쿼리: &str, 컨텐츠: &'a str) -> Vec<&'a str> {
    let mut 결과 = Vec::new();

    for line in 컨텐츠.lines() {
        if line.contains(쿼리) {
            결과.push(line);
        }
    }

    결과
}

// 반복자를 사용하면 위의 코드를 더 간결한 방식으로 작성할 수 있음.
pub fn 반복자_사용_이후_검색<'a>(쿼리: &str, 컨텐츠: &'a str) -> Vec<&'a str> {
    컨텐츠.lines().filter(|라인| 라인.contains(쿼리)).collect()
}
// 검색 함수의 목적은 쿼리를 포함하는 컨텐츠의 모든 라인을 반환하는 것이었다.
// filter 예제와 유사하게, 이 코드는 "라인.contains(쿼리)" 이 true를 반환하는 라인들만 유지하기 위해 filter 어댑터를 사용한다.
// 그 다음 collect를 사용하여 매칭된 라인들을 모아 새로운 벡터로 만든다.
