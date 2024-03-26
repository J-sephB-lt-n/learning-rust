// from "Rust by Example"

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

fn rect_area(rect: Rectangle) -> f32 {
    (rect.top_left.y - rect.bottom_right.y) * (rect.bottom_right.x - rect.top_left.x)
}

fn main() {
    let my_rect = Rectangle {
        top_left: Point { x: 3.1, y: 8.2 },
        bottom_right: Point { x: 5.0, y: 5.0 },
    };
    println!("Rectangle is: {:?}", my_rect);
    println!("Area is: {}", rect_area(my_rect));
}
