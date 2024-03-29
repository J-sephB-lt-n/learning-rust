// from "Rust by Example > Custom Types > Structures"

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

fn square(top_left: Point, side_len: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: top_left.x,
            y: top_left.y,
        },
        bottom_right: Point {
            x: top_left.x + side_len,
            y: top_left.y - side_len,
        },
    }
}

fn main() {
    let my_rect = Rectangle {
        top_left: Point { x: 3.1, y: 8.2 },
        bottom_right: Point { x: 5.0, y: 5.0 },
    };
    println!("Rectangle is: {:?}", my_rect);
    println!("Area is: {}", rect_area(my_rect));
    let top_left = Point { x: 6.9, y: 4.2 };
    let side_len: f32 = 1.5;
    let my_square: Rectangle = square(top_left, side_len);
    println!("Square is: {:?}", my_square);
}
