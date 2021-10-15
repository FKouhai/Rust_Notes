fn main() {
    fibo_calc(5);
}

fn fibo_calc(mut x: i32) {
    let mut z = 0; //Sum vuale
    let mut fibo = 1; // Last digit
    let mut finalfibo = 0; // Current value in the fibonaccie series
    for _i in 1..x {
        z = fibo + finalfibo; //Summing the last digit (In 1st iter == 1) with the current == 0
        finalfibo = fibo; // Assigning the current digit to the last digit
        fibo = z; //Assigning the last digit to the sum value
    }
    println!("{}", z);
}
