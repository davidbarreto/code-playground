use std::io::{stdin, stdout, Write};

pub enum Command {
    Set(String, String),
    Get(String),
    Delete(String),
    List,
    Exit,
}

pub enum ParseError {
    UnknownCommand(String),
    WrongArguments(String, u8, u8),
}

pub fn parse_args(args: Vec<String>) -> Result<Command, ParseError> {

    match args.get(0).map(String::as_str) {
        Some("set") => {
            if args.len() == 3 {
                Ok(Command::Set(args[1].clone(), args[2].clone()))
            } else {
                Err(ParseError::WrongArguments(args[0].clone(), 2, (args.len()-1) as u8))
            }
        },
        Some("get") => {
            if args.len() == 2 {
                Ok(Command::Get(args[1].clone()))
            } else {
                Err(ParseError::WrongArguments(args[0].clone(), 1, (args.len()-1) as u8))
            }
        },
        Some("delete") => {
            
            if args.len() == 2 {
                Ok(Command::Delete(args[1].clone()))
            } else {
                Err(ParseError::WrongArguments(args[0].clone(), 1, (args.len()-1) as u8))
            }
        },
        Some("list") => Ok(Command::List),
        Some("exit") => Ok(Command::Exit),
        _ => Err(ParseError::UnknownCommand(args[0].clone())),
    }
}

pub fn read(buffer: &mut String) {
    print!("> ");
    let _ = stdout().flush();

    buffer.clear();
    let _ = stdin().read_line(buffer);
}

pub fn print_get(key: &String, value: Option<&String>) {
    match value { 
        Some(v) => println!("{}: {}", key, v),
        None => println!("Key '{}' not found", key),
    }
}

pub fn print_set(key: &String, value: &String) {
    println!("Set key [{}] to value [{}]", key, value);
}

pub fn print_list(pairs: Vec<(&String, &String)>) {

    if pairs.is_empty() {
        println!("No key-value pairs found.");
    } else {
        for (key, value) in pairs {
            println!("{}: {}", key, value);
        }
    }
}

pub fn print_delete(key: &String, result: Option<&String>) {
    match result {
        Some(v) => println!("Deleted key [{}] with value [{}]", key, v),
        None => println!("Key '{}' not found for deletion", key),
    }
}

pub fn print_error(error: ParseError) {
    match error {
        ParseError::UnknownCommand(command) => println!("Unknown command: {}", command),
        ParseError::WrongArguments(command, expected, found) => println!("Wrong number of arguments for command {}: expected {}, found {}", command, expected, found),
    }
}

pub fn print_exit() {
    println!("Exiting... bye!");
}

pub fn print_usage() {
    println!("Usage:");
    println!("  set <key> <value>   - Set a key-value pair");
    println!("  get <key>           - Get the value for a key");
    println!("  delete <key>        - Delete a key-value pair");
    println!("  list                - List all key-value pairs");
    println!("  exit                - Exit the program");
}