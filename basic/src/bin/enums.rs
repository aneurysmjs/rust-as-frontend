enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }

        false
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => false,
            Color::Green => false,
            Color::Blue => true,
            Color::Yellow => true,
        }
    }
}

fn main() {
    fn print_color(color: Color) {
        match color {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue"),
            Color::Yellow => println!("yellow"),
        }
    }

    let my_green = Color::Green;
    let my_red = Color::Red;
    let my_blue = Color::Blue;
    let my_yellow = Color::Yellow;

    assert!(my_green.is_green());
    assert!(my_red.is_green_parts());
    assert!(my_blue.is_green_parts());
    assert!(my_yellow.is_green_parts());

    print_color(my_yellow);
}
