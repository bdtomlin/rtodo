use super::*;

#[test]
fn test_app_new() {
    let app = App::new(":memory:", std::io::stdout());
    assert_eq!(app.db_path, ":memory:");
    assert!(app.db.table_exists(None, "todos").unwrap());
}

#[test]
fn test_add_todo() {
    let mut todo = Todo::new("example todo".to_string());
    let app = App::new(":memory:", std::io::stdout());
    app.add_todo(&mut todo).unwrap();
    assert_eq!(app.count_todos().unwrap(), 1);
    assert_eq!(todo.text, "example todo");
    assert_eq!(todo.done, false);
}

#[test]
fn test_get_todo() {
    let mut todo = Todo::new("example todo".to_string());
    let app = App::new(":memory:", std::io::stdout());
    app.add_todo(&mut todo).unwrap();
    let todo = app.get_todo(todo.id).unwrap();
    assert_eq!(todo.id, 1);
    assert_eq!(todo.text, "example todo");
}

#[test]
fn test_list_todos() {
    let mut todo1 = Todo::new("example 1".to_string());
    let mut todo2 = Todo::new("example 2".to_string());

    let app = App::new(":memory:", std::io::stdout());

    app.add_todo(&mut todo1).unwrap();
    app.add_todo(&mut todo2).unwrap();

    let list = app.list_todos().unwrap();
    assert_eq!(list.get(0).unwrap().id, todo1.id);
    assert_eq!(list.get(1).unwrap().id, todo2.id);
}

#[test]
fn test_count_todos() {
    let app = App::new(":memory:", std::io::stdout());

    assert_eq!(app.count_todos().unwrap(), 0);
    app.add_todo(&mut Todo::new("example todo".to_string()))
        .unwrap();
    assert_eq!(app.count_todos().unwrap(), 1);
}

#[test]
fn test_delete_todos() {
    let app = App::new(":memory:", std::io::stdout());
    app.add_todo(&mut Todo::new("example todo".to_string()))
        .unwrap();
    assert_eq!(app.count_todos().unwrap(), 1);
    let mut todo = app.get_todo(1).unwrap();
    app.delete_todo(&mut todo).unwrap();
    assert_eq!(app.count_todos().unwrap(), 0);
}

#[test]
fn test_complete_todo() {
    let app = App::new(":memory:", std::io::stdout());

    let mut todo = Todo::new("example todo".to_string());
    app.add_todo(&mut todo).unwrap();

    assert_eq!(todo.done, false);
    app.complete_todo(&mut todo).unwrap();
    assert_eq!(todo.done, true);
    assert_eq!(app.get_todo(todo.id).unwrap().done, true);
}

#[test]
fn test_icomplete_todo() {
    let app = App::new(":memory:", std::io::stdout());

    let mut todo = Todo::new("example todo".to_string());
    app.add_todo(&mut todo).unwrap();

    app.complete_todo(&mut todo).unwrap();
    assert_eq!(todo.done, true);
    assert_eq!(app.get_todo(todo.id).unwrap().done, true);

    app.incomplete_todo(&mut todo).unwrap();
    assert_eq!(todo.done, false);
    assert_eq!(app.get_todo(todo.id).unwrap().done, false);
}
