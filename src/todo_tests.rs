use super::*;

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
