#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1
}

fn main() {
    let mut item = Item { count: 0 };

    add_one(&mut item);

    println!("{:?}", item);
}
