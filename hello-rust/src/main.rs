use std::io;

fn main() {
    println!("-- Calculator --");

    loop {
        println!("Please enter your expression (e.g., 2 + 2), or type 'quit' to exit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Problem reading line");

        let input = input.trim();

        if input == "quit" {
            println!("Closing...");
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 3 {
            println!("Invalid expression. Please enter an expression in the format 'number operator number'.");
            continue;
        }

        let num1: f64 = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", tokens[0]);
                continue;
            }
        };

        let num2: f64 = match tokens[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", tokens[2]);
                continue;
            }
        };

        let result = match tokens[1] {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero");
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Invalid operator: {}", tokens[1]);
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
