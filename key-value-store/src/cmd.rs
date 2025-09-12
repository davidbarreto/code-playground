pub enum Command {
    Set(String, String),
    Get(String),
    Delete(String),
    List,
    Exit,
    Unknown,
}

pub fn parse_args(args: Vec<String>) -> Command {

    match args.get(0).map(String::as_str) {
        Some("set") if args.len() == 3 => Command::Set(args[1].clone(), args[2].clone()),
        Some("get") if args.len() == 2 => Command::Get(args[1].clone()),
        Some("delete") if args.len() == 2 => Command::Delete(args[1].clone()),
        Some("list") => Command::List,
        Some("exit") => Command::Exit,
        _ => Command::Unknown,
    }
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