mod cmd;
mod kvstore;

use kvstore::{traits::KVStore, memory::InMemoryKVStore};

fn main() {

    let mut store = InMemoryKVStore::new();
    let mut buffer = String::new();

    loop {

        cmd::read(&mut buffer);

        let result = cmd::parse_args(buffer.split_whitespace().map(|s| s.to_string()).collect());
        match result {
            Ok(cmd::Command::Get(key)) => {
                let value = store.get(&key);
                cmd::print_get(&key, value);
            },
            Ok(cmd::Command::Set(key, value)) => {
                store.set(&key, &value);
                cmd::print_set(&key, &value);
            },
            Ok(cmd::Command::Delete(key)) => {
                let value = store.delete(&key);
                cmd::print_delete(&key, value.as_ref());
            },
            Ok(cmd::Command::List) => {
                let items = store.list();
                cmd::print_list(items);
            },
            Ok(cmd::Command::Exit) => {
                cmd::print_exit();
                break;
            },
            Err(cause) => {
                cmd::print_error(cause);
                cmd::print_usage();
            },
        }
    }
}
