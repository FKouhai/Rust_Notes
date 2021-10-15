use std::io;

fn main() {
    println!("Farenheit to Celsius");
    let mut far_to_cell = String::new();

    io::stdin()
        .read_line(&mut far_to_cell)
        .expect("Failed to read line");
    let far_to_cell: u32 = match far_to_cell.trim().parse() {
        Ok(num) => num,
        Err(_) => 32,
    };
    let final_celsius = (far_to_cell - 32) * 5 / 9;
    println!("This {} ºF equals to {} ºC", far_to_cell, final_celsius);
}
