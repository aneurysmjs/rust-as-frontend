fn multiply(num: Option<usize>) -> usize {
    // if num.is_some() {
    //     return num.unwrap() * 5;
    // }
    // 0

    num.unwrap_or(0) * 5
}

fn multiply_some(num: Option<usize>) -> Option<usize> {
    // `map` allows to access to the inner value if there's an inner value.
    // if there's not it just remain 'None'
    // num.map(|x| x * 5)

    // if you have a function that returns an 'Err' or an 'Option'
    // that is the same type your working with.
    // You can use the `?` operator
    //
    // let num = num?;
    //
    // what it does under the hood:
    //
    // match num {
    //  Some(x) => x,
    //  None => return None,
    // }
    Some(num? * 5)
}

fn main() {
    multiply(Some(5));

    multiply_some(Some(5));
}
