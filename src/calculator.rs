use std::io;

/// The user can tell the program what to do
/// The program communicates using these instead of chars
enum Instruction {
    Quit,
    EnterOperation,
    Undo,
    Invalid,
}

/// The user can tell the program what to do with each number
/// The program communicates using these instead of chars
enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
    Invalid,
}

/// Gets the user input as a string
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
    println!(" - e to enter a number");
    println!(" - u to undo");
    println!(" - q to quit");
    loop {
        let input_str = read_user_input();
        if input_str.len() > 0 {
            // just get the first char
            let c = input_str.chars().nth(0).unwrap();

            // map the instruction to the enum
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
            // just get the first char
            let c = input_str.chars().nth(0).unwrap();

            // map the input to the enum
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

/// Gets a number from a user by checking it
fn get_user_number() -> f32 {
    // by default mutable variable
    let user_float: f32;

    loop {
        println!("Enter a number");

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

/// takes the instruction enum and mutates the history
fn execute_instruction(instruction: Instruction, history: &mut Vec<HistoryItem>) -> bool {
    match instruction {
        Instruction::EnterOperation => enter_number(history),
        Instruction::Undo => undo(history),
        Instruction::Quit => return false,
        _ => (),
    }
    return true;
}

/// removes the last element of the history
fn undo(history: &mut Vec<HistoryItem>) {
    if history.len() == 0 {
        println!("Cannot undo since the tree is empty");
    } else {
        history.pop();
    }
}

/// converts the enum back to the char to display the operation
fn get_char_from_operation(operation: &Operation) -> char {
    match operation {
        Operation::Add => '+',
        Operation::Subtract => '-',
        Operation::Divide => '/',
        Operation::Multiply => '*',
        Operation::Invalid => ' ',
    }
}

/// displays the history so we can see what is going on
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

/// Does the calculation as is displayed
fn display_result(history: &mut Vec<HistoryItem>) {
    print!("Result: ");
    if history.len() == 0 {
        println!();
        return;
    }
    let sliced_history = &history[1..];
    let mut result;
    result = &history[0].value + 0.0;
    for h in sliced_history {
        result = match h.operation {
            Operation::Add => result + h.value,
            Operation::Subtract => result - h.value,
            Operation::Divide => result / h.value,
            Operation::Multiply => result * h.value,
            Operation::Invalid => result,
        };
    }
    println!("{}", result);
}

/// Puts an operation and value into the history
fn enter_number(history: &mut Vec<HistoryItem>) {
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

/// value and operation go together in one struct for better structure and integrity
struct HistoryItem {
    value: f32,
    operation: Operation,
}

/// My Calculator
///
/// This calculator goes to show that most calculators, like this one, do not have
/// order of operations, rather they mutate the result one operation at a time.
///
/// This does have one extra feature that most basic calculators don't have: undo.
///
/// The instructions will be shown on screen.
fn main() {
    println!("My Calculator");
    let mut history: Vec<HistoryItem> = Vec::new();
    loop {
        let instruction = get_user_instruction();
        if !execute_instruction(instruction, &mut history) {
            break;
        }
        display_history(&mut history);
        display_result(&mut history);
    }
}
