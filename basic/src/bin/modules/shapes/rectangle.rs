// `super` means "one module up"
// so: rectangle -> shapes -> area
use super::area::Area;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
