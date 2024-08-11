use std::io::Read;

pub fn example_1() -> std::io::Result<()> {
    println!("\nExample 1...");
    let mut b = "This string will be read.".as_bytes();
    let mut buffer = [0; 10];
    
    // read up to 10 bytes
    let bytes_read = b.read(&mut buffer)?;
    println!("bytes_read: {}", bytes_read);
    println!("modified buffer: {:?}", buffer);
    
    // return the Ok variant wrapping an empty tuple 
    Ok(())
}

pub fn example_2() -> std::io::Result<()> {
    println!("\nExample 2...");
    // let mut b = "This string will be read.".as_bytes();
    let mut b = "This even longer string will be read.".as_bytes();
    let mut buffer = [0; 25];
    
    // read exactly 25 bytes or return an error
    b.read_exact(&mut buffer)?;
    println!("modified buffer: {:?}", buffer);
    
    // return the Ok variant wrapping an empty tuple 
    Ok(())
}

pub fn example_3(bytes: &mut &[u8]) -> std::io::Result<[u8; 25]> {
    println!("\nExample 3...");
    let mut buffer = [0_u8; 25];
    println!("pre-read buffer: {:?}", buffer);

    bytes.read_exact(&mut buffer)?;

    Ok(buffer)
}