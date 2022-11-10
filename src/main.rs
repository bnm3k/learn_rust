use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (i, n) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{i}: {n}")?;
        }
        write!(f, "]")
    }
}

fn main() {
    let l = List(vec![10, 20, 30]);
    println!("{}", l);
}
