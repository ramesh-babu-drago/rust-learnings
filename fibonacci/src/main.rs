fn main() {
    println!("Enter how many Fibonacci terms to generate (max 45):");

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        println!("Failed to read input.");
        return;
    }

    let num: u32 = match input.trim().parse() {
        Ok(n) if n <= 45 => n,
        Ok(_) => {
            println!("Please enter a number up to 45.");
            return;
        }
        Err(_) => {
            println!("Invalid input. Please enter a non-negative number.");
            return;
        }
    };

    generate_fibonacci(num);
}

fn generate_fibonacci(num: u32) {
    let (mut a, mut b) = (0, 1);
    println!("\n Fibonacci sequence ({} terms):\n", num);
    println!("{:<6} | {:>20}", "Index", "Value");
    println!("-------------------------------");

    for i in 0..num {
        println!("{:<6} | {:>20}", i, a);
        let temp = a + b;
        a = b;
        b = temp;
    }
}
