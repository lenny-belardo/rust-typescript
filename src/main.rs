mod shapes;
use anyhow::Result;
use shapes::{circle::Circle, rect::Rect, collisions::{Points, Contains}};
use std::{collections::{HashMap, HashSet}, fmt::Display};
use std::fs;
use std::str::FromStr;

use crate::shapes::collisions::Collidable;

enum Shape {
    Circle(Circle),
    Rect(Rect)
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("bad shape"))
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => return c.points(),
            Shape::Rect(r) => return r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => return c.contains_point(point),
            Shape::Rect(r) => return r.contains_point(point),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => return write!(f, "{}", c),
            Shape::Rect(r) => return write!(f, "{}", r),
        }
    }
}

fn main() -> Result<()> {
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

    #[derive(Debug)]
    struct Custom {
        age: usize,
        name: String
    }

    #[derive(Debug)]
    enum Item {
        Number(usize),
        String(String),
        MyCustom(Custom)
    }

    fn append(items: &mut Vec<Item>){
        items.push(Item::String("Hello Fem!".into()));
    }

    let mut items: Vec<Item> = vec![Item::Number(1), Item::Number(2)];
    println!("{:?}", items);
    append(&mut items);
    println!("{:?}", items);

    fn multiply_number(number: Option<usize>) -> Option<usize> {
        // 0 or x * 5 
        // if let Some(x) = number {
        //     return x * 5;
        // }
        // return 0;

        // return number.unwrap_or(0) * 5;

        // None or x * 5
        // let number = match number {
        //     Some(x) => x * 5,
        //     None => return None
        // };
        // return Some(number);

        return Some(number? * 5);
    }

    println!("{:?}", multiply_number(Some(5)));
    println!("{:?}", multiply_number(None));

    fn practice(nums: Vec<usize>, idx: usize) -> usize {
        return nums.get(idx).unwrap_or(&idx) * 5;
    }

    println!("{:?}", practice(vec![1, 2, 3], 5));
    println!("{:?}", practice(vec![1, 2, 3], 1));

    let file_name = std::env::args().nth(1)
        .expect("the file name to be passed in");

    let number_file = std::fs::read_to_string(file_name)
        .expect("unable to read the file to string");

    number_file
        .lines()
        .for_each(|line| {
            if let Ok(value) = line.parse::<usize>() {
                println!("{}", value)
            } else {
                println!("Line not a number");
            }
        });

    #[derive(Debug)]
    struct Entry {
        count: usize
    }

    fn add_one(entry: &mut Entry) {
        entry.count + 1;
    }

    let mut entry = Entry { count: 1 };
    println!("{:?}", entry);
    add_one(&mut entry);
    println!("{:?}", entry);

    fn print_all(items: &Vec<Entry>) {
        for item in items {
            println!("{:?}", item);
        }
    }

    let mut items = vec![Entry { count: 1 }];
    let first: Option<&mut Entry> = items.get_mut(0);
    let second = items.get_mut(1);
    println!("{:?}", second);
    print_all(&items);

    let vec_of_numbers = vec![1, 2, 3];
    let formatted_vec_of_numbers = vec_of_numbers
        .iter()
        .map(|x| x + 1);

    println!("{:?}", formatted_vec_of_numbers);

    use crate::shapes::{area::Area, circle::Circle, rect::Rect};

    let rect = Rect::default();
    let rect_2 = Rect::default();

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };

    let circ_2 = Circle {
        x: 1.5,
        y: 1.5,
        radius: 4.0,
    };

    println!("{}", rect);
    println!("{}", circ.area());
    println!("{}", rect.area());
    println!("{}", 6.4.area());

    rect.collide(&rect_2);
    circ.collide(&circ_2);
    rect.collide(&circ);

    let shapes = std::fs::read_to_string("shapes")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    shapes
        .iter()
        .skip(1)
        .zip(shapes
            .iter()
            .take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| {
            println!("{} collides with {}", a, b);
        });

    return Ok(());
}