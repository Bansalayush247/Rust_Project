use std::env;
        use std::process;
        
        fn main() {
            let args: Vec<String> = env::args().collect();
            
            if args.len() != 4 {
                eprintln!("Usage: calc [num1] [operator] [num2]");
                process::exit(1);
            }
                   let num1: f64 = args[1].parse().expect("First argument is not a number");
            let operator = &args[2];
            let num2: f64 = args[3].parse().expect("Third argument is not a number");
        
            match operator.as_str() {
                "+" => println!("Result: {}", num1 + num2),
                "-" => println!("Result: {}", num1 - num2),
                "*" => println!("Result: {}", num1 * num2),
                "/" => println!("Result: {}", num1 / num2),
                _ => {
                    eprintln!("Invalid operator. Please use +, -, *, or /.");
                    process::exit(1);
                }
            }
        }