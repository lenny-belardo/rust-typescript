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

    println!("{:?}", collected_vec);
}