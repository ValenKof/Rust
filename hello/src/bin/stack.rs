struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

struct StackIter<'a, T> {
    head: &'a Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, value: T) {
        let node = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.value)
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<&T> {
        if let Some(head) = self.head.as_deref() {
            Some(&head.value)
        } else {
            None
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn iter(&self) -> StackIter<T> {
        StackIter { head: &self.head }
    }
}

impl<'a, T> std::iter::Iterator for StackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head) = self.head.as_deref() {
            self.head = &head.next;
            Some(&head.value)
        } else {
            None
        }
    }
}

fn debug_stack<T: std::fmt::Display>(st: &Stack<T>) -> String {
    let top = if let Some(top) = st.top() {
        top.to_string()
    } else {
        "<none>".to_string()
    };
    format!("empty: {} top: {}\n", st.empty(), top)
}

fn do_push<T: std::fmt::Display>(st: &mut Stack<T>, x: T) -> String {
    let res = format!("push: {}\n", &x);
    st.push(x);
    res + &debug_stack(st)
}

fn do_pop<T: std::fmt::Display>(st: &mut Stack<T>) -> String {
    let top = if let Some(top) = st.pop() {
        top.to_string()
    } else {
        "<none>".to_string()
    };
    format!("pop: {}\n", top) + &debug_stack(st)
}

fn test_stack<T: std::str::FromStr + std::fmt::Display>() -> String
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut res = String::new();
    let mut st: Stack<T> = Stack::new();
    res += &debug_stack(&st);
    res += &do_push(&mut st, T::from_str("1").unwrap());
    res += &do_push(&mut st, T::from_str("2").unwrap());
    res += &do_push(&mut st, T::from_str("3").unwrap());
    res += &do_pop(&mut st);
    res += &do_pop(&mut st);
    res += &do_push(&mut st, T::from_str("4").unwrap());
    res += &do_pop(&mut st);
    res += &do_pop(&mut st);
    res += &do_pop(&mut st);
    res += &do_push(&mut st, T::from_str("5").unwrap());
    res += &do_pop(&mut st);
    res += &do_pop(&mut st);
    res
}

fn main() {
    println!("{}", test_stack::<String>());
    println!(
        "Same? {}!",
        if test_stack::<String>() == test_stack::<i32>() {
            "YES"
        } else {
            "NO"
        }
    );

    let st: Stack<i32> = {
        let mut st = Stack::new();
        st.push(1);
        st.push(2);
        st.push(3);
        st.pop();
        st.push(4);
        st
    };

    for x in st.iter() {
        println!("{}", x);
    }
}
