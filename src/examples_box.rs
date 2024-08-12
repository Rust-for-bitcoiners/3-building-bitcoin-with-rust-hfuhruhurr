extern crate rand;
// use rand::Rng;

pub fn example_1() {
    println!("\nBox Example 1...");
    
    // reading values
    let a: Box<i32> = Box::new(13);
    println!("a: {}", a);

    // mutation
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let mut b: Box<Point> = Box::new(Point{x: 27, y: 88});
    b.x = 25;
    b.y = 11;
    println!("b: {:?}", b);
}

#[derive(Debug)]
#[allow(dead_code)] // suppresses the "fields are never read" warning
pub struct Person {
    name: String,
    age: u8,
}

pub fn example_2() -> Option<Box<Person>> {
    println!("\nBox Example 2...");

    let random_u8 = rand::random::<u8>();
    println!("random #: {}", random_u8);

    if (random_u8 % 2) == 1 {
        Some(Box::new(Person{
            name: "Dude".to_string(),
            age: 42,
        }))
    } else {
        None
    }
}