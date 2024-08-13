use std::io;
// use std::cmp::Ordering;

fn main() {
    const ONE_HOUR_IN_SECONDS: u32 = 60;
    const ARRAY_SIZE: u32 = 5;

    let x = 5;
    println!("X is {x}");


    let mut tup = (500, 25.2, 1);

    let (x, y, z) = tup;

    tup.0 = 300;

    println!("Tuple: {} and some variable: {}", y, tup.0);

    let a: [i32; 5] = [1,2,3,4,5]; //on the stack, not heap
    let b = [3; 6];

    println!("I have an array of 5 elements, what item would you like to see?");

    let mut intake = String::new(); // we always read a string

    io::stdin()
        .read_line(&mut intake)
        .expect("Failed read");

    let intake: i32 = match intake.trim().parse() { //this variable will be used to access the element, so usize
        Ok(num) => num,
        Err(_) => {
            println!("Not a good number");
            return;
        }
    };    

    if intake>=0 && (intake as usize) <= a.len() {
        println!("The element at {} is {}", intake, a[intake as usize]);
    }
    else{
        println!("Index out of bounds");
    }
    
}
