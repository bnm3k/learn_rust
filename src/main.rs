use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn map(list: &List) {
    match list {
        Cons(n, next) => {
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

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn main() {
    return;
    let a = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Cons(30, Rc::new(Nil)))))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    map(&b);
    map(&c);
    let b = Box::new(5);
    print_num(&b);
    let mb = MyBox::new(6);
    print_num(&mb);
    assert_eq!(5, *b);
    hello(&(MyBox::new(String::from("BNM"))));
}
