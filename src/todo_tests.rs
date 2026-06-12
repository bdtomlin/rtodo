use super::*;
mod db;

#[test]
fn test_todo_new() {
    let todo = Todo::new("test".to_string());
    assert_eq!(todo.id, 0);
    assert_eq!(todo.text, "test");
    assert_eq!(todo.done, false);
}

#[test]
fn test_display_format() {
    let todo = Todo {
        id: 1,
        text: "test".to_string(),
        done: false,
    };
    assert_eq!(todo.to_string(), "id: 1, text: test, done: false");
}

#[test]
fn test_todo_add() {
    let mut todo = Todo::new("example todo".to_string());
    let app = App::new(":memory:", std::io::stdout());
    Todo::add(&app, &mut todo).unwrap();
    assert_eq!(Todo::count(&app).unwrap(), 1);
    assert_eq!(todo.text, "example todo");
    assert_eq!(todo.done, false);
}

#[test]
fn test_todo_get() {
    let mut todo = Todo::new("example todo".to_string());
    let app = App::new(":memory:", std::io::stdout());
    Todo::add(&app, &mut todo).unwrap();
    let todo = Todo::get(&app, todo.id).unwrap();
    assert_eq!(todo.id, 1);
    assert_eq!(todo.text, "example todo");
}

#[test]
fn test_todo_list() {
    let mut todo1 = Todo::new("example 1".to_string());
    let mut todo2 = Todo::new("example 2".to_string());

    let app = App::new(":memory:", std::io::stdout());

    Todo::add(&app, &mut todo1).unwrap();
    Todo::add(&app, &mut todo2).unwrap();

    let list = Todo::list(&app).unwrap();
    assert_eq!(list.get(0).unwrap().id, todo1.id);
    assert_eq!(list.get(1).unwrap().id, todo2.id);
}

#[test]
fn test_todo_count() {
    let app = App::new(":memory:", std::io::stdout());

    assert_eq!(Todo::count(&app).unwrap(), 0);
    Todo::add(&app, &mut Todo::new("example todo".to_string())).unwrap();
    assert_eq!(Todo::count(&app).unwrap(), 1);
}

#[test]
fn test_todo_delete() {
    let app = App::new(":memory:", std::io::stdout());
    Todo::add(&app, &mut Todo::new("example todo".to_string())).unwrap();
    assert_eq!(Todo::count(&app).unwrap(), 1);
    let todo = Todo::get(&app, 1).unwrap();
    todo.delete(&app).unwrap();
    assert_eq!(Todo::count(&app).unwrap(), 0);
}

#[test]
fn test_todo_complete() {
    let app = App::new(":memory:", std::io::stdout());

    let mut todo = Todo::new("example todo".to_string());
    Todo::add(&app, &mut todo).unwrap();

    assert_eq!(todo.done, false);
    todo.complete(&app).unwrap();
    assert_eq!(todo.done, true);
    assert_eq!(Todo::get(&app, todo.id).unwrap().done, true);
}

#[test]
fn test_todo_incomplete() {
    let app = App::new(":memory:", std::io::stdout());

    let mut todo = Todo::new("example todo".to_string());
    Todo::add(&app, &mut todo).unwrap();

    todo.complete(&app).unwrap();
    assert_eq!(todo.done, true);
    assert_eq!(Todo::get(&app, todo.id).unwrap().done, true);

    todo.incomplete(&app).unwrap();
    assert_eq!(todo.done, false);
    assert_eq!(Todo::get(&app, todo.id).unwrap().done, false);
}
