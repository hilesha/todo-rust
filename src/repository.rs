pub mod db {
    use crate::Todo;
    use rustbreak::error::RustbreakError;
    use rustbreak::{deser::Ron, BackendError, FileDatabase};
    use serde::{Deserialize, Serialize};
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    pub fn fetch_records(
        db: &rustbreak::Database<
            std::collections::HashMap<i32, Todo>,
            rustbreak::backend::FileBackend,
            rustbreak::deser::Ron,
        >,
        ordering: fn(&&i32, &&i32) -> Ordering,
    ) -> Result<Vec<(i32, String)>, RustbreakError> {
        let mut todo_items = Vec::<(i32, String)>::new();
        let read_data = |db: &std::collections::HashMap<i32, Todo>| {
            let mut keys: Vec<&i32> = db.keys().collect();
            keys.sort_by(ordering);
            for key in keys {
                let todo_item = db.get(key).unwrap();
                todo_items.push((*key, todo_item.title.clone()));
            }
        };
        match db.read(read_data) {
            Ok(_) => Ok(todo_items),
            Err(error) => Err(error),
        }
    }

    pub fn fetch_max_id(
        db: &rustbreak::Database<
            std::collections::HashMap<i32, Todo>,
            rustbreak::backend::FileBackend,
            rustbreak::deser::Ron,
        >,
    ) -> i32 {
        let mut max_value = 0;
        db.read(|db| {
            for item in db.iter() {
                if max_value < *item.0 {
                    max_value = *item.0;
                }
            }
        });
        max_value
    }

    pub fn write_record(
        db: &rustbreak::Database<
            std::collections::HashMap<i32, Todo>,
            rustbreak::backend::FileBackend,
            rustbreak::deser::Ron,
        >,
        max_id: i32,
        todo_item: String,
    ) {
        db.write(|db| {
            db.insert(
                max_id + 1,
                Todo::new(
                    max_id + 1,
                    todo_item.to_string(),
                    "body".to_string(),
                    crate::TodoStatus::Pending,
                ),
            )
        });
        db.save();
    }

    pub fn delete_record(
        db: &rustbreak::Database<
            std::collections::HashMap<i32, Todo>,
            rustbreak::backend::FileBackend,
            rustbreak::deser::Ron,
        >,
        todo_id: &i32,
    ) {
        db.write(|db| db.remove(todo_id));
        db.save();
    }

    pub fn get_record(
        db: &rustbreak::Database<
            std::collections::HashMap<i32, Todo>,
            rustbreak::backend::FileBackend,
            rustbreak::deser::Ron,
        >,
        todo_id: &i32,
    ) -> Result<Todo, RustbreakError> {
        let record = db.read(|db| {
            let fetch_record = db.get(todo_id);
            if let Some(todo_item) = fetch_record {
                Ok(todo_item.clone())
            } else {
                Err(RustbreakError::Poison)
            }
        });
        match record {
            Ok(todo_item) => match todo_item {
                Ok(item) => Ok(item),
                Err(error) => Err(RustbreakError::Poison),
            },
            Err(error) => Err(RustbreakError::Poison),
        }
    }

    pub fn update_record(
        db: &rustbreak::Database<
            std::collections::HashMap<i32, Todo>,
            rustbreak::backend::FileBackend,
            rustbreak::deser::Ron,
        >,
        todo: Todo,
    ) {
        db.write(|db| db.insert(todo.id, todo));
        db.save();
    }
}
