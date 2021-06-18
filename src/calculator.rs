use std::io;

enum Instruction {
    Quit,
    EnterOperation,
    Undo,
    Invalid,
}

enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
    Invalid,
}

fn read_user_input() -> String {
    // mutable variable
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input.");

    return String::from(input.trim());
}

/// Gets an int from a user by checking it
fn get_user_instruction() -> Instruction {
    println!("Enter a command to continue");
    println!(" - e to enter an integer");
    println!(" - u to undo");
    println!(" - q to quit");
    loop {
        let input_str = read_user_input();
        if input_str.len() > 0 {
            let c = input_str.chars().nth(0).unwrap();
            let instruction: Instruction = match c {
                'e' => Instruction::EnterOperation,
                'u' => Instruction::Undo,
                'q' => Instruction::Quit,
                _ => Instruction::Invalid,
            };
            if let Instruction::Invalid = instruction {
                println!("Invalid command. Try again.");
            } else {
                return instruction;
            }
        }
    }
}

/// Gets an operation from a user by checking it
fn get_user_operation() -> Operation {
    println!("Enter an operation to continue (+, -, /, *)");
    loop {
        let input_str = read_user_input();
        if input_str.len() > 0 {
            let c = input_str.chars().nth(0).unwrap();
            let operation: Operation = match c {
                '+' => Operation::Add,
                '-' => Operation::Subtract,
                '/' => Operation::Divide,
                '*' => Operation::Multiply,
                _ => Operation::Invalid,
            };
            if let Operation::Invalid = operation {
                println!("Invalid operation. Try again.");
            } else {
                return operation;
            }
        }
    }
}

/// Gets an int from a user by checking it
fn get_user_number() -> f32 {
    // by default mutable variable
    let user_float: f32;

    loop {
        println!("Enter an int");

        // read user input
        let input_str = read_user_input();
        let input = input_str.trim().parse::<f32>();

        // catch errors and try again
        match input {
            Ok(v) => {
                user_float = v;
                break;
            }
            Err(_) => {
                println!("Invalid response, try again");
            }
        }
    }
    return user_float;
}

fn execute_instruction(instruction: Instruction, history: &mut Vec<HistoryItem>) -> bool {
    match instruction {
        Instruction::EnterOperation => enter_integer(history),
        Instruction::Undo => undo(history),
        Instruction::Quit => return false,
        _ => (),
    }
    display_history(history);
    display_result(history);
    return true;
}

fn undo(history: &mut Vec<HistoryItem>) {
    if history.len() == 0 {
        println!("Cannot undo since the tree is empty");
    } else {
        history.pop();
    }
}

fn get_char_from_operation(operation: &Operation) -> char {
    match operation {
        Operation::Add => '+',
        Operation::Subtract => '-',
        Operation::Divide => '/',
        Operation::Multiply => '*',
        Operation::Invalid => ' ',
    }
}

fn display_history(history: &mut Vec<HistoryItem>) {
    print!("Stack: ");
    let length = history.len();
    for h in history {
        print!("{} ({} ", get_char_from_operation(&h.operation), h.value)
    }
    for _ in 0..length {
        print!(")");
    }
    println!();
}

fn display_result(history: &mut Vec<HistoryItem>) {
    let sliced_history = &history[1..];
    let mut result;
    result = &history[0].value + 0.0;
    for h in sliced_history {
        result = match h.operation {
            Operation::Add => result + h.value,
            Operation::Subtract => result - h.value,
            Operation::Divide => result / h.value,
            Operation::Multiply => result * h.value,
            Operation::Invalid => result
        };
    }
    println!("Result: {}", result);

}

fn enter_integer(history: &mut Vec<HistoryItem>) {
    let operation = if history.len() > 0 {
        get_user_operation()
    } else {
        Operation::Invalid
    };
    let number = get_user_number();

    history.push(HistoryItem {
        operation: operation,
        value: number,
    });
}

struct HistoryItem {
    value: f32,
    operation: Operation,
}

fn main() {
    println!("Binary tree");
    let mut history: Vec<HistoryItem> = Vec::new();
    loop {
        let instruction = get_user_instruction();
        if !execute_instruction(instruction, &mut history) {
            break;
        }
    }
}
