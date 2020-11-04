use rustbreak::{deser::Ron, FileDatabase};
use std::collections::HashMap;
use std::convert::TryFrom;
use structopt::StructOpt;
use todo;
use todo::{Error, TaskStatus, Todo};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "f", long = "file", env = "TASK_ARCHIVE")]
    file_path: String,
    #[structopt(name = "command", short = "c", long = "command")]
    command: String,
    #[structopt(short = "t", long = "task", default_value = "")]
    todo: String,
}

enum Command {
    List,
    Add,
    Delete,
}

impl TryFrom<&str> for Command {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" => Ok(Command::Add),
            "list" => Ok(Command::List),
            "delete" => Ok(Command::Delete),
            _ => Err(Error::InvalidInput(String::from(value))),
        }
    }
}

pub fn main() {
    let opt = Opt::from_args();
    match Command::try_from(opt.command.as_ref()) {
        Ok(command) => match command {
            Command::List => match todo::get_todo_list(&opt.file_path) {
                Ok(todo_items) => {
                    for item in todo_items {
                        println!("{}", item);
                    }
                }
                Err(error) => println!("{}", error),
            },
            Command::Add => match todo::insert_todo(&opt.file_path, opt.todo) {
                Ok(TaskStatus::Added) => println!("task added successfully"),
                Ok(_) => println!("invalid status"),
                Err(error) => println!("{}", error),
            },
            Command::Delete => println!("yet to be added"),
            _ => println!("come back later"),
        },
        Err(error) => println!("error encountered"),
    }
}
