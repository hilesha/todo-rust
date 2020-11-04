use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    FileReadError(#[from] std::io::Error),
    #[error("invalid command specified in the input")]
    InvalidInput(String),
}

pub enum TaskStatus {
    Added,
    Deleted,
    Marked,
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub fn get_todo_list(path: &str) -> Result<Vec<String>> {
    match File::open(path) {
        Ok(todo_file) => {
            let todo_file_reader = BufReader::new(todo_file);
            let mut todo_items = Vec::new();
            for item in todo_file_reader.lines() {
                match item {
                    Ok(item) => todo_items.push(format_todo_item(item)),
                    Err(error) => return Err(Error::FileReadError(error)),
                }
            }
            Ok(todo_items)
        }
        Err(error) => return Err(Error::FileReadError(error)),
    }
}

pub fn insert_todo(path: &str, todo_item: String) -> Result<TaskStatus> {
    match OpenOptions::new().append(true).open(path) {
        Ok(ref mut file_writer) => {
            let mut item_write_to_file = todo_item;
            item_write_to_file.push('\n');
            file_writer.write(item_write_to_file.as_bytes());
            Ok(TaskStatus::Added)
        }
        Err(error) => Err(Error::FileReadError(error)),
    }
}

fn format_todo_item(todo_item: String) -> String {
    format!("[ ] - {}", todo_item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_run_the_app() {
        let path = "/aUsers/hilesha/self/rust/todo/src/todos.data";
        let todo_items = get_todo_list(path).unwrap();
        assert_eq!(2, todo_items.len());
        assert_eq!("[ ] - make a todo list application", todo_items[0]);
        assert_eq!("[ ] - play the drop game", todo_items[1]);
    }
}
