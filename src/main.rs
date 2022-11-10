enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn len(&self) -> u32 {
        use List::*;
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn stringify(&self) -> String {
        use List::*;
        match *self {
            Cons(head, ref tail) => {
                format!("[{}] -> {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}
fn main() {
    let l = List::new();
    let l = l.prepend(1).prepend(2).prepend(3);
    println!("l len={}", l.len());
    println!("{}", l.stringify())
}
