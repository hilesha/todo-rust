use rustbreak::{deser::Ron, FileDatabase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    FileReadError(#[from] std::io::Error),
    #[error("invalid command specified in the input")]
    InvalidInput(String),
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Added,
    Deleted,
    Marked,
}

#[derive(Debug)]
pub enum TodoStatus {
    Pending,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    title: String,
    body: String,
    status: String,
}

impl Todo {
    pub fn new(id: i32, title: String, body: String, status: String) -> Self {
        Self {
            id,
            title,
            body,
            status,
        }
    }
}

impl Clone for Todo {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            body: self.body.clone(),
            status: self.status.clone(),
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub fn get_todo_list(path: &str) -> Result<Vec<String>> {
    let db = FileDatabase::<HashMap<i32, Todo>, Ron>::load_from_path_or_default(Path::new(path));
    let mut todo_items = Vec::new();
    let db = db.unwrap();
    db.read(|db| {
        for item in db.iter() {
            let todo_item = item.1;
            todo_items.push(format_todo_item(todo_item.title.clone()));
        }
    });
    Ok(todo_items)
}

pub fn insert_todo(path: &str, todo_item: String) -> Result<TaskStatus> {
    let db = FileDatabase::<HashMap<i32, Todo>, Ron>::load_from_path_or_default(Path::new(path));
    let db = db.unwrap();
    let mut max_value = 0;
    db.read(|db| {
        for item in db.iter() {
            if max_value < *item.0 {
                max_value = *item.0;
            }
        }
    });

    db.write(|db| {
        db.insert(
            max_value + 1,
            Todo::new(
                max_value + 1,
                todo_item.to_string(),
                "body".to_string(),
                "pending".to_string(),
            ),
        )
    });
    db.save();
    Ok(TaskStatus::Added)
}

fn format_todo_item(todo_item: String) -> String {
    format!("[ ] - {}", todo_item)
}
