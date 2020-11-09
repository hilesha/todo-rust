mod repository;

pub use crate::repository::db;
use rustbreak::error::RustbreakError;
use rustbreak::{deser::Ron, FileDatabase};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DBError(#[from] rustbreak::error::RustbreakError),
    #[error("invalid command specified in the input")]
    InvalidInput(String),
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Added,
    Deleted,
    Marked,
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TodoStatus {
    Pending,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    title: String,
    body: String,
    status: TodoStatus,
}

impl Todo {
    pub fn new(id: i32, title: String, body: String, status: TodoStatus) -> Self {
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

fn descending(a: &&i32, b: &&i32) -> Ordering {
    a.cmp(b)
}

pub fn get_todo_list(path: &str) -> Result<Vec<String>> {
    println!("data");
    let db = FileDatabase::<HashMap<i32, Todo>, Ron>::load_from_path_or_default(Path::new(path));
    let db = db?;
    match db::fetch_records(&db, descending) {
        Ok(records) => Ok({
            let mut format_records = Vec::<String>::new();
            for record in records {
                format_records.push(format_todo_item(record));
            }
            format_records
        }),
        Err(err) => Err(Error::DBError(err)),
    }
}

pub fn insert_todo(path: &str, todo_item: String) -> Result<TaskStatus> {
    let db = FileDatabase::<HashMap<i32, Todo>, Ron>::load_from_path_or_default(Path::new(path));
    let db = db.unwrap();
    let max_value = db::fetch_max_id(&db);
    db::write_record(&db, max_value, todo_item);
    Ok(TaskStatus::Added)
}

pub fn delete_todo(path: &str, todo_id: i32) -> Result<TaskStatus> {
    let db = FileDatabase::<HashMap<i32, Todo>, Ron>::load_from_path_or_default(Path::new(path));
    let db = db.unwrap();
    db::delete_record(&db, &todo_id);
    Ok(TaskStatus::Deleted)
}

pub fn complete_todo(path: &str, todo_id: i32) -> Result<TaskStatus> {
    let db = FileDatabase::<HashMap<i32, Todo>, Ron>::load_from_path_or_default(Path::new(path));
    let db = db.unwrap();
    let output = db::get_record(&db, &todo_id);
    let mut completed_todo = output?;
    completed_todo.status = TodoStatus::Complete;
    db::update_record(&db, completed_todo);
    Ok(TaskStatus::Marked)
}

fn format_todo_item(todo_item: (i32, String)) -> String {
    format!("[ {} ] - {}", todo_item.0, todo_item.1)
}
