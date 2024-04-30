fn main() {
    // this is how we initialize variables
    let x: i8 = -5; // signed integer
    let y: u32 = 77; // unsigned integer
    let z: f32 = 1000.001; // float

    // How we print the above variables:
    // Method - 1 ->
    print!("x: {}, y: {}, z: {} \n", x, y, z);

    // Method - 2 ->
    println!("x: {} ", x);
    println!("y: {} ", y);
    println!("z: {}", z);
}
