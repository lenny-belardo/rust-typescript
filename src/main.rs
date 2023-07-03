use std::collections::{HashMap, HashSet};
use std::fs;

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

    let collected_hash_map: HashMap<&str, usize> = vec!["this", "is", "a", "test"]
        .into_iter()
        .enumerate()
        .map(|(idx, item)| (item, idx))
        .collect(); 

    println!("collected_hash_map: {:?}", collected_hash_map);

    // from the book
    let contents_the_book_way = fs::read_to_string("lines.txt")
        .expect("Should have been able to read the file");

    println!("file contents:\n{contents_the_book_way}");

    // from the course
    let file = std::fs::read_to_string("lines.txt").unwrap();

    file
        .lines()
        .for_each(|line| println!("{}", line));

    file
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));

    file
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));

    enum Color {
        Red,
        Green,
        Blue,
        Yellow
    }

    impl Color {
        fn is_green(&self) -> bool {
            if let Color::Green = self {
                return true;
            }
            return false;
        }

        fn is_green_parts(&self) -> bool {
            match self {
                Color::Red => return false,
                Color::Green => return false,
                Color::Blue => return true,
                Color::Yellow => return true,
            }
        }
    }

    fn print_color(color: Color) {
        let chosen_color = match color {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue"),
            Color::Yellow => println!("yellow"),
            _ => println!("just color")
        };

        return chosen_color;
    }

    print_color(Color::Red);

    let color = Color::Yellow;

    println!("{}", color.is_green());
}