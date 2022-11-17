#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::{Debug, Display};

struct Point<T> {
    x: T,
    y: T,
}

impl<T: Debug + Display> Point<T> {
    fn print(&self) {
        println!("Point(x: {}, y: {})", self.x, self.y);
    }
}

struct Empty;
struct Null;

trait HasNothing {
    fn is(&self, s: &str);
}

impl HasNothing for Empty {
    fn is(&self, val: &str) {
        println!("Is Empty: {val}")
    }
}

impl HasNothing for Null {
    fn is(&self, val: &str) {
        println!("Is Null: {val}")
    }
}

impl<T> HasNothing for Point<T> {
    fn is(&self, val: &str) {
        println!("Is Point: {val}")
    }
}

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T: HasNothing, U: HasNothing> DoubleDrop<T> for U {
    fn double_drop(self, t: T) {
        self.is("self");
        t.is("t");
    }
}
// ---------------------------------------------------------------------------
struct Pair(i32, i32);

trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Pair {
    type A = i32;
    type B = i32;

    fn contains(&self, n1: &i32, n2: &i32) -> bool {
        (&self.0 == n1) && (&self.1 == n2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    // let p = Point { x: 10.0, y: 20.0 };
    // p.print();
    // p.print();
    // let empty = Empty;
    // let null = Null;
    // empty.double_drop(null);
    // null.double_drop(empty);
    // p.double_drop(empty);
    // empty;
    // null;
    //
    let n1 = 3;
    let n2 = 10;
    let container = Pair(n1, n2);
}
