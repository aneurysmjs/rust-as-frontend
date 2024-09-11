use shapes::{area::Area, circle::Circle, rectangle::Rectangle};

mod shapes;

fn main() {
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        height: 4.5,
        width: 4.5,
    };

    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 5.0,
    };

    println!("Rectangle {}", rect.area());
    println!("Circle {}", circle.area());
}
