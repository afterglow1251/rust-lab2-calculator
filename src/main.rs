use std::io;
mod infix;
mod polish;
mod utils;
mod constants;

use crate::utils::common::{evaluate_expression, check_exit_command, choose_mode, mode_to_string};
use crate::constants::{Mode, MODE_COMMON, MODE_POLISH, EXIT_COMMAND};

fn main() {
    let mut mode: Option<Mode> = None; // Initialize mode as None

    loop {
        // Ask user for mode selection until a valid mode is chosen
        if mode.is_none() {
            println!("Choose a mode:\n\
            - Common (type '{}')\n\
            - Polish notation (type '{}')\n\
            q to exit", MODE_COMMON, MODE_POLISH);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read string");
            let input = input.trim();

            // Check for exit
            if check_exit_command(input) {
                return;
            }

            if let Some(new_mode) = choose_mode(input) {
                mode = Some(new_mode);
            } else {
                println!("Incorrect choice. Try again");
            }
        }

        // If mode is still None, continue to prompt for mode selection
        if mode.is_none() {
            continue;
        }

        println!("Now let's process expressions in mode: {}", mode_to_string(&mode));

        loop {
            println!("Enter an expression to calculate:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.trim();

            // Check for exit
            if check_exit_command(input) {
                return;
            }

            // Check for mode switching
            if let Some(new_mode) = choose_mode(input) {
                mode = Some(new_mode);
                println!("Mode changed to: {:?}", mode);
                continue;
            }

            match evaluate_expression(input, mode.as_ref().unwrap()) {
                Ok(result) => {
                    println!("Result: {}", result);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}


