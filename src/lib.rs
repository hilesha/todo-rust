use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
pub fn get_todo_list(path: &str) -> Result<Vec<String>, String> {
    let todo_file = File::open(path).unwrap();
    let todo_file_reader = BufReader::new(todo_file);
    let mut todo_items = Vec::new();
    for item in todo_file_reader.lines() {
        match item {
            Ok(item) => todo_items.push(format_todo_item(item)),
            Err(error) => return Err(format!("error reading line {}", error)),
        }
    }
    Ok(todo_items)
}

fn format_todo_item(todo_item: String) -> String {
    format!("[ ] - {}", todo_item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_run_the_app() {
        let path = "/Users/hilesha/self/rust/todo/src/todos.data";
        let todo_items = get_todo_list(path).unwrap();
        assert_eq!(2, todo_items.len());
        assert_eq!("[ ] - make a todo list application", todo_items[0]);
        assert_eq!("[ ] - play the drop game", todo_items[1]);
    }
}
