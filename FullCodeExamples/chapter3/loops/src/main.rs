fn main() {
    let mut number = 3;

    loop {
        if number != 0 {
            println!("{}!", number);
            number -= 1;
        } else {
            break;
        }
    }
    println!("LIFTOFF!!!");

    // Looping with for over an array
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        //Using iter's which is a built in function for arrays
        println!("The value is: {} ", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");
}
