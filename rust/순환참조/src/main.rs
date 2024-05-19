use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
fn main() {
    순환참조();
    node();
    변화의_시각화();
}
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    // 이제 Cons variant의 두번째 요소가 RefCell<Rc<List>> 타입인데, 이는 i32 값을 변경하는
    // 능력을 갖는 대신 Cons variant가 가리키고 있는 List 값을 변경하길 원한다는 것이다.
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            // Cons variant를 갖고있다면 두번째 아이템에 접근하기 편하게 만들었음.
            Nil => None,
        }
    }
}

fn 순환참조() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail()); // RefCell이 될 것.

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail()); // a가 될 것.

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
// a의 리스트가 b를 가리키게 한 이후 a와 b의 Rc<List> 인스턴스의 참조 카운트는 둘다 2이다.
// main의 끝에서 러스트는 b를 버리는데, 이는 Rc<List> 참조 카운트를 2에서 1로 줄인다.
// Rc<List>가 힙에 보유한 메모리는 이 시점에서 해제되지않는다. 참조카운트가 0이아니라 1이기때문.
// 그다음 러스트는 a를 버리고, 이는 마찬가지로 Rc<List> 인스턴스가 가진 참조 카운트를
// 2에서 1로 줄인다. 이 인스턴스의 메모리 또한 버려질 수 없다. 왜냐면 이쪽의 Rc<List> 인스턴스도
// 여전히 무언가를 참조하기 때문이다.

// 이 케이스는 그렇게 심각하진 않다. 순환 참조 생성 직후 프로그램이 종료되기 때문.

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // 자식 노드가 그의 부모를 알게 하기 위해 parent 필드 추가.
    // 이는 순환 참조를 방지하기 위해 Rc 대신 Weak를 사용한다.
    // 이 관계를 생각해보면, 부모노드는 자식들을 소유해야한다. 즉 부모노드가 버려지게되면
    // 자식노드들도 버려져야한다. 하지만 자식은 그의부모를 소유해서는 안된다.
    // 즉, 자식노드가 버려져도 부모노드는 여전히 존재해야한다 -> 약한 참조를 위한 경우.
    children: RefCell<Vec<Rc<Node>>>,
}

fn node() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // 새로 추가
        children: RefCell::new(vec![]),
        // 자식노드가 없는 Node 인스턴스
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()), // 새로 추가
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        // leaf를 자식으로갖는 Node 인스턴스
    });
    // 이제 leaf에 있는 Node의 소유자가 둘이되었음. branch.children으로 접근할 수 있게되었지만,
    // leaf에서 branch로 접근할 수 없다. 이는 leaf가 branch를 참조하지 않기 때문이다.

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // leaf의 parent 필드의 RefCell<Weak<Node>>에 있는 borrow_mut 메소드를 사용하고
    // 그다음 Rc::downgrade 함수를 사용하여 branch의 Rc<Node>로부터 branch에 대한 Weak<Node> 참조자를
    // 생성했다.

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // 이제 leaf의 parent 필드는 branch를 참조하고있다.
    // 부모를 출력할 때는 branch를 갖고있는 Some variant를 얻게 될것이다: 이제 leaf는 부모에 접근할 수 있다.
    // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
}

fn 변화의_시각화() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // leaf 생성 후 이것의 Rc<Node>는, 강한참조 카운트 1과 약한참조 카운트 0을 가짐.

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        // 내부 스코프에서, branch를 만들고 leaf와 연관을 짓게 함

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        // 카운트 출력시 branch의 Rc<Node>는 강한참조 카운트 1과
        // (leaf.parent에 대한) 약한참조 카운트 1을 가짐.

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        // leaf 내 카운트 출력시 강한참조 카운트 2를 갖고있음: branch가 branch.children에
        // 저장된 leaf의 Rc<Node>에 대한 클론을 갖게되었지만 약한참조가 0이기 때문.
    }
    // 내부스코프가 끝나면 branch는 스코프 밖으로 벗어남. Rc<Node>의 강한참조 카운트가 0이되고
    // 그에따라 이것의 Node는 버려짐.

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // 여기서 leaf의 Rc<Node>는 강한참조 카운트 1과 약한참조 카운트 0을 가짐.
    // 이제 leaf 변수가 다시 Rc<Node>의 유일한 참조자이기 때문.

    // 자식과 부모의 관계가 Weak<T> 참조자로 있어야 함을 Node의 정의에 특정함으로써, 순환 참조와
    // 메모리 누수를 만들지 않으면서 자식 노드를 가리키는 부모 노드 혹은 그 반대의 것을
    // 만들 수 있다.
}
