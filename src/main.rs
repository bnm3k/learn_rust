// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

#[derive(Debug)]
struct NumVal {
    n: i32,
}

fn create_box(n: i32) -> Box<NumVal> {
    Box::new(NumVal { n })
}

fn mut_box(mut b: Box<NumVal>, new_val: i32) -> Box<NumVal> {
    *b = NumVal { n: new_val };
    b
}

fn mut_ref_box(b: &mut Box<NumVal>, new_val: i32) {
    b.n = new_val
}

fn destroy_box(_: Box<NumVal>) {}

fn main() {
    // allocate on heap
    let n1 = create_box(100);
    println!("n1={:?}", n1);

    // take ownership
    let n1 = mut_box(n1, 200);
    println!("n1={:?}", n1);

    // borrow as mutable
    let mut n1 = n1;
    mut_ref_box(&mut n1, 300);
    println!("n1={:?}", n1);

    destroy_box(n1);
}
