#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn area(r: &Rectangle) -> i32 {
    r.width * r.height
}
fn main() {
    let width = 30;
    let height = 50;
    let scale = 2;
    let r = Rectangle {
        width: dbg!(width * scale),
        height,
    };

    println!("The area of the rectangle is {}", area(&r));
    dbg!(r);
}
