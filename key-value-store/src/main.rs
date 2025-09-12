mod cmd;
mod kvstore;

use std::io::{stdin, stdout, Write};

fn main() {

    let mut store = kvstore::KVStore::new();
    let mut buffer = String::new();

    loop {

        print!("> ");
        let _ = stdout().flush();
        buffer.clear();
        let _ = stdin().read_line(&mut buffer);

        let command = cmd::parse_args(buffer.split_whitespace().map(|s| s.to_string()).collect());
        match command {
            cmd::Command::Get(key) => {
                let value = store.get(&key);
                cmd::print_get(&key, value);
            },
            cmd::Command::Set(key, value) => {
                store.set(&key, &value);
                cmd::print_set(&key, &value);
            },
            cmd::Command::Delete(key) => {
                let value = store.delete(&key);
                cmd::print_delete(&key, value.as_ref());
            },
            cmd::Command::List => {
                let items = store.list();
                cmd::print_list(items);
            },
            cmd::Command::Exit => {
                cmd::print_exit();
                break;
            },
            cmd::Command::Unknown => {
                println!("Invalid command");
                cmd::print_usage();
            },
        }
    }
}
