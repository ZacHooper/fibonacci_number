fn main() {
    println!("Let's get Fibonacci!");
    println!("Type which number in the Fibonacci sequence you want to see:");

    // Get input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert input to u32
    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    // Fiboannci formula
    let fib = (1.0 / 5.0_f64.sqrt())
        * (((1.0 + 5.0_f64.sqrt()) / 2.0).powf(input as f64)
            - ((1.0 - 5.0_f64.sqrt()) / 2.0).powf(input as f64));

    println!(
        "The {}th number in the Fibonacci sequence is {}",
        input, fib,
    )
}
