#[allow(dead_code)]
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList{ head: Option::None }
    }
}

pub fn example_1() {
    // create an empty list, v1
    println!("\nLinked List Example 1...");
    
    let empty_list: LinkedList<u32> = LinkedList{ head: Option::None };
    println!("empty_list: {:?}", empty_list);
}

pub fn example_2() {
    // create an empty list, v2
    println!("\nLinked List Example 2...");
    
    let empty_list: LinkedList<u32> = LinkedList::new();
    println!("empty_list: {:?}", empty_list);
}

pub fn example_3() {
    // create a list with one node
    println!("\nLinked List Example 3...");

    let the_one_node = Node {
        value: 27,
        next: None,
    };

    let singleton_list = LinkedList {
        head: Some(the_one_node),
    };
    println!("singleton list: {:?}", singleton_list);
}

pub fn example_4() {
    // create a list with two nodes
    println!("\nLinked List Example 4...");

    let node_2 = Node {
        value: 88,
        next: None,
    };

    let node_1 = Node {
        value: 27,
        next: Some(Box::new(node_2)),
    };

    let multi_list = LinkedList {
        head: Some(node_1),
    };

    println!("multi-list: {:?}", multi_list);
}

