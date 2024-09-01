use std::{
    collections::{HashMap, HashSet},
    isize,
};

fn main() {
    let data = vec![1, 2, 3];
    let mut data_iter = data.iter().map(|x| x + 1);

    let mut new_vector = vec![];

    while let Some(x) = data_iter.next() {
        new_vector.push(x + 1);
    }

    println!("{:?}", new_vector);

    /*
     * collect into string
     */

    let my_string: String = vec!["this", "is", "a", "test"].into_iter().collect();

    println!("my_string {:}", my_string);

    /*
     * collect into HashSet (this will be Set in js)
     */

    let my_set: HashSet<isize> = vec![1, 2, 3].into_iter().collect();

    println!("my set {:?}", my_set);

    let my_hash: HashMap<&str, usize> = vec!["Hello this", "is", "a", "string", "to", "HashMap"]
        .into_iter()
        .enumerate()
        .map(|(idx, item)| (item, idx))
        .collect();

    println!("my_hash {:?}", my_hash);

    /*
     * sum
     */
    let sum: i32 = vec![1, 2, 3].iter().sum();

    assert_eq!(sum, 6);

    /*
     * how many times
     */
    let how_many_times: usize = vec![1, 2, 3].into_iter().skip(2).count();

    assert_eq!(how_many_times, 1);
}
