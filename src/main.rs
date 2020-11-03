use structopt::StructOpt;
use todo;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "f", long = "file", required = true)]
    file_path: String,
}

pub fn main() {
    let opt = Opt::from_args();
    let todo_items = todo::get_todo_list(&opt.file_path).unwrap();
    for item in todo_items {
        println!("{}", item);
    }
}
