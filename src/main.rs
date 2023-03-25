use std::io;

fn main() {
    fn read_integer_input(prompt: &str) -> f64 {
        loop {
            let mut input = String::new();
            println!("{}", prompt);

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            match input.trim().parse::<f64>() {
                Ok(num) => return num,
                Err(_) => println!("Please enter a valid integer.")
            }

        }
    }

    let num1 = read_integer_input("Please provide the first integer.");
    let num2 = read_integer_input("Please provide the second integer.");

    println!("Sum: {}", num1 + num2);
    println!("Difference: {}", num1 - num2);
    println!("Product {}", num1 * num2);
    println!("Quotient: {}", num1 / num2);
    

}
