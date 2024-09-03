struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    Custom(Custom),
}

fn append(items: &mut Vec<Item>) {
    // passing str, it expects an `Item`
    // items.push("que pasa papi")

    // still passing str, it expects `Item::String(String)`
    // items.push(Item::String("que pasa papi"))

    /*
     * `.into()` it just a trait that says:
     * "Hey!, I want you to convert this thing into the right type.
     * And don't worry, you know the type"
     */
    items.push(Item::String("que pasa papi".into()));
}

pub fn run() {
    let mut items: Vec<Item> = vec![];

    append(&mut items);

    let my_item = Item::Number(5);

    match &my_item {
        // pull the inner value when pattern matching
        Item::Number(num) => println!("I am a number: {}", num),
        Item::String(string) => println!("I am a string {}", string),
        Item::Custom(custom) => println!("Name: {}, Age: {}", custom.name, custom.age),
    }

    match &my_item {
        Item::Custom(custom) => println!("Name: {}, Age: {}", custom.name, custom.age),
        // `_` ignore the rest.
        _ => {}
    }

    match &my_item {
        Item::Custom(Custom { age, .. }) => println!("Age: {}", age),
        _ => {}
    }

    match &my_item {
        Item::Custom(custom) if custom.name == "Jero" => println!("Que pasa Jero"),
        Item::Custom(custom) if custom.age > 33 => println!("N64 was the best console"),
        Item::Custom(custom) if custom.age < 30 => println!("Xbox was the best console"),
        _ => {}
    }
}
