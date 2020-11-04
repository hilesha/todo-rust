use todo;
#[test]
fn test_get_all_todo_items() {
    let path = String::from("/Users/hilesha/self/rust/todo/src/todos.data");
    assert_eq!(8, todo::get_todo_list(&path).unwrap().len())
}
