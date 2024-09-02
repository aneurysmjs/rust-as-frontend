fn main() {
    let file = std::fs::read_to_string("./text.txt").unwrap();

    file.lines()
        //.enumerate()
        .for_each(|line| println!("{}", line));
}
