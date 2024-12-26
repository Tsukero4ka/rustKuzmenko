use std::io;

fn main() {
    let num1 = loop {
        println!("Enter the first number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(n) => break n,
            Err(_) => println!("Invalid input. Please try again."),
        }
    };

    let num2 = loop {
        println!("Enter the second number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(n) => break n,
            Err(_) => println!("Invalid input. Please try again."),
        }
    };
    // Get the operator
    let operator = loop {
        println!("Enter the operator (+, -, *, /):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "+" | "-" | "*" | "/" => break input.trim().to_string(),
            _ => println!("Invalid operator. Please try again."),
        }
    };

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: division by zero!");
                return;
            }
            num1 / num2
        }
        _ => unreachable!(), // Impossible case due to operator validation
    };
    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}