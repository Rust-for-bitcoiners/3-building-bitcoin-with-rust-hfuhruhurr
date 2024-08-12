// mod linked_list;
// mod block;
// mod mresult;

mod examples_enum;
mod examples_box;

fn main() {
    println!("About to call Example 1, yo...");
    let _ = examples_enum::example_1();
    let _ = examples_enum::example_2();


    // Example 3
    let mut b = "no hay 25 chars".as_bytes();

    match examples_enum::example_3(&mut b) {
        Ok(buffer) => println!("read successfully! returned buffer: {:?}", buffer),
        Err(e) => eprintln!("Tenemos una problema!: {}", e)
    }

    examples_box::example_1();
    println!("dude: {:?}", examples_box::example_2());
}
