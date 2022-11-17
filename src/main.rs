#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

fn apply_transform<F>(mut f: F)
where
    F: FnMut(&mut i32),
{
    let mut val: i32 = 1;
    println!("val={val}");
    f(&mut val);
    println!("val={val}");
}

fn add_10(v: &mut i32) {
    *v += 10;
}

fn create_adder(n: i32) -> impl FnMut(&mut i32) {
    let add_fn = move |v: &mut i32| {
        *v += n;
    };
    return add_fn;
}

fn other() {
    let x = 7;
    let add_7 = |v: &mut i32| {
        *v += x;
    };
    let add_20 = create_adder(20);
    apply_transform(add_7);
    apply_transform(add_10);
    apply_transform(add_20);
}

fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz")
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", n)
        }
    }
    let vec1 = vec![1, 2, 3];
    println!("2 in vec: {}", vec1.into_iter().any(|n| n == 2));
}
