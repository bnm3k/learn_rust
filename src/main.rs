use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn map(list: &List) {
    match list {
        Cons(r, next) => {
            let n = r.borrow();
            print!("{} -> ", n);
            map(next);
        }
        Nil => {
            println!("Nil");
        }
    }
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox<T>")
    }
}

fn print_num(n: &i32) {
    println!("n={}", n);
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let tail = Rc::new(Nil);
    let node = |num, next| Cons(Rc::new(RefCell::new(num)), next);
    let list = node(6, Rc::new(node(5, tail)));
    map(&list);
    let b = Box::new(5);
    print_num(&b);
    let mb = MyBox::new(6);
    print_num(&mb);
    assert_eq!(5, *b);
    hello(&(MyBox::new(String::from("BNM"))));
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        let msg = if percentage_of_max >= 1.0 {
            Some("Error: You are over your quota!")
        } else if percentage_of_max >= 0.9 {
            Some("Urgent warning: You've used up over 90% of your quota!")
        } else if percentage_of_max >= 0.75 {
            Some("Warning: You've used up over 75% of your quota!")
        } else {
            None
        };

        if let Some(msg_content) = msg {
            self.messenger.send(msg_content)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
