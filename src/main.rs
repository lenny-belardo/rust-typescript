fn main() {
    println!("Hello, world!");

    let my_vec: Vec<isize> = vec![1, 2, 3]
      .iter()
      .map(|x| x + 1)
      .collect();

    println!("{:?}", my_vec);
}
