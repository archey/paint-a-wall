extern crate math; // import math for round and ceil support

use std::io;
use std::io::Write; // to bring flush into scope
use math::round; // to bring in needed round and ceil function to round up the amount of paint

// 1 gallon of paint covers 350 sq ft
// Gallons of paint are sold in units of 1
// Round up gallons variable
// Assume 1 gallon of paint costs $27
// Assume you can paint 100 sqft per hour

fn main() {
    let mut width = String::new();
    let mut height = String::new();
    //let _amount_of_paint: f64; // 64bit floating point type
    //let _cost_of_paint: u64; // 64bit unsigned integer 8 byte
    //let _amount_of_time: f64; // 64bit floating point type
    const COST_PER_GALLON: f64 = 27.0; // 64 bit floasting point type
    const COST_PER_HOUR: f64 = 100.0; // 64 bit floating point type
    const SQFT_PER_GALLON: f64 = 350.0; // 64bit floating point type

    print!("Enter the width of the wall in feet: ");
    // flushes stdout to display the print statement
    io::stdout().flush().unwrap(); 

    
    io::stdin().read_line(&mut width)
            .expect("Failed to read witdh");
    
    print!("Enter the height of the wall in feet: {}", height);
    io::stdout().flush().unwrap(); // flushes stdout to display the print statment

    io::stdin().read_line(&mut height)
            .expect("Failed to read height");

    let width: f64 = width.trim().parse().expect("Not a number!!!");

    let height: f64 = height.trim().parse().expect("Not a number!!!");

    // Output total area of the room
    println!("Total area to paint in square feet: {}", area(width, height));

    // Output the amount of paint needed
    println!("Total number of gallons of paint needed: {}", 
             amount_of_paint(area(width, height), SQFT_PER_GALLON));

    // Output cost of paint in dollars
    println!("Total cost of paint in dollars: {}",
             cost_of_paint(amount_of_paint(area(width, height), SQFT_PER_GALLON), COST_PER_GALLON));

    // Output the amount of time needed to paint in hours
    println!("Total time to paint in hours: {:.2}",
             amount_of_time(area(width, height), COST_PER_HOUR));
}

// Calculate the area of the room
fn area(w: f64, h: f64) -> f64 {
    w * h
}

// Calculate the amount of paint needed
fn amount_of_paint(area: f64, sqft: f64) -> f64 {
    // round up to the first decimal place
    // ceil(value, scale)
    // TODO Typecasting to unsigned 8bit to remove decimal point, need to FIX
    round::ceil(area / sqft, 0)
}

// Calculate the cost of the paint
fn cost_of_paint(amount: f64, cost: f64) -> f64 {
    round::ceil(amount * cost, 0)
}

// Calculate the amount of time it will take to paint a room
fn amount_of_time(area: f64, time: f64) -> f64 {
    area / time
}
