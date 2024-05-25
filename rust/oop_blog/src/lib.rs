// pub struct Post {
//     state: Option<Box<dyn State>>, // 비공개 필드
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     // 텍스트를 추가하는 메소드 추가
//     pub fn add_text(&mut self, text: &str) {
//         // 가변 참조자 self: Post 인스턴스의 변경을 유발하므로
//         self.content.push_str(text);
//         // 게시물의 상태에 의존적이지 않으므로 상태 패턴이 아님
//     }

//     // 빈 컨텐츠를 반환하는 content 메소드 추가.
//     pub fn content(&self) -> &str {
//         // ""
//         self.state.as_ref().unwrap().content(&self) // state 값의 content 메소드를 얻어낸 값이 반환
//     }

//     // 게시물의 검토 요청 기능 -> 게시물의 상태를 Draft에서 PendingReview로 변경
//     pub fn request_review(&mut self) {
//         if let Some(e) = self.state.take() {
//             self.state = Some(e.request_review()) // 내부 메소드 request_review 호출
//         }
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }
// }

// trait State {
//     // request_review와 함께 추가
//     fn request_review(self: Box<Self>) -> Box<dyn State>; // Box<Self>: 메소드가 오직 그 타입을 보유한 Box에 대해서 호출될 경우에만 유효함.
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }
// struct Draft {}

// impl State for Draft {
//     // request_review와 함께 추가
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {}) // PendingReview 상태의 새로운 Box 인스턴스를 생성
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self // 아무 효과 X
//     }
// }

// // request_review와 함께 추가
// struct PendingReview {}

// // request_review와 함께 추가
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     // Publish된 경우, 각 메소드에 대해 원래 상태를 보존해야하므로 self를반환

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }

// // 규칙 관련한 로직들은, Post 전체에 흩어져있는 게 아니라, 상태 객체 안에서만 존재하게된다.
// // Enum으로도 가능하지만, match 표현식을 사용해야하므로, 코드가 길어질 수 있다.
// // 단점이라면, 일부 상태들은 결합되어있으므로 상태들 사이에 다른 상태를 추가한다면 고칠 게 많아진다는 점이 있다.

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        // Post가 아닌 DraftPost를 반환. Post를 반환할 함수가 없으므로, Post 인스턴스 생성 불가능.
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // 여기에 content는 없음 -> 이제 프로그램은, 모든 게시물이 draft post로 시작되고 draft post는 자신의 컨텐츠를 표시할 수 없음.

    // 이후..
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        // PendingReviewRequest 에서 Approve를 하는경우 마침내 Post 인스턴스가 반환
        Post {
            content: self.content,
        }
    }
}
