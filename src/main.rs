#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(r: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: sx, y: by },
        bottom_right: Point { x: bx, y: sy },
    } = r;
    (bx - sx) * (by - sy)
}

fn square(top_left: Point, length: f32) -> Rectangle {
    let Point { x: sx, y: by } = top_left;
    Rectangle {
        top_left: top_left,
        bottom_right: Point {
            x: sx + length,
            y: by - length,
        },
    }
}

fn main() {
    let r = square(Point { x: 1.0, y: 9.0 }, 8.0);
    println!("{:?}", r);
    println!("area={}", rect_area(r));
}
