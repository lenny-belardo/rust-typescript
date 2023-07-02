use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let my_vec = vec![1, 2, 3];

    let mut iterated_vec = my_vec
      .iter()
      .map(|x| x + 1);

    let mut collected_vec = vec![];

    while let Some(x) = iterated_vec.next() {
        collected_vec.push(x)
    }

    println!("collected_vec: {:?}", collected_vec);

    let collected_string: String = vec!["this", "is", "a", "test"]
        .into_iter()
        .collect(); 

    println!("collected_string: {:?}", collected_string);

    let collected_hash_set: HashSet<isize> = vec![1, 2, 3, 4]
        .into_iter()
        .collect(); 

    println!("collected_hash_set: {:?}", collected_hash_set);
}