mod splitter;

use splitter::Splitter;

fn main() {
    dbg!("{}", Splitter::new("this is a text, task, test, toast...", ',').collect::<Vec<&str>>());
}
