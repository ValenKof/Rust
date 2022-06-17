struct Node {
    value: String,
    next: Option<Box<Node>>,
}

struct Stack {
    head: Option<Box<Node>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { head: None }
    }

    pub fn push(&mut self, value: String) {
        let node = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<String> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.value)
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<&str> {
        if let Some(head) = self.head.as_deref() {
            Some(&head.value)
        } else {
            None
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn debug(&self) {
        println!(
            "empty: {} top: {}",
            self.empty(),
            self.top().unwrap_or("<none>")
        );
    }
}

fn do_push(st: &mut Stack, s: &str) {
    println!("push: {}", s);
    st.push(s.to_string());
    st.debug();
}

fn do_pop(st: &mut Stack) {
    let top = st.pop();
    println!("pop: {}", top.unwrap_or("<none>".to_string()));
    st.debug();
}

fn main() {
    let mut st = Stack::new();
    st.debug();
    do_push(&mut st, "1");
    do_push(&mut st, "2");
    do_push(&mut st, "3");
    do_pop(&mut st);
    do_pop(&mut st);
    do_push(&mut st, "4");
    do_pop(&mut st);
    do_pop(&mut st);
    do_pop(&mut st);
    do_push(&mut st, "5");
    do_pop(&mut st);
    do_pop(&mut st);
}
