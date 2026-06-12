use super::*;
use crate::app::App;

fn args(input: &[&str]) -> Vec<String> {
    let mut args = vec!["rtodo".to_string()];
    args.extend(input.iter().map(|s| s.to_string()));
    args
}
fn app() -> App<Vec<u8>> {
    App::new(":memory:", Vec::new())
}

#[test]
fn test_run_add() {
    let mut app = app();
    run(&mut app, args(&["add", "buy milk"]));
    app.get_todo(1).unwrap();
    assert_eq!(
        String::from_utf8(app.output).unwrap(),
        "id: 1, text: buy milk, done: false\n".to_string()
    );
}

#[test]
fn test_run_add_and_list() {
    let mut app = app();
    app.add_todo(&mut Todo::new("buy milk".to_string()))
        .unwrap();
    app.add_todo(&mut Todo::new("walk dog".to_string()))
        .unwrap();
    run(&mut app, args(&["list"]));
    assert_eq!(
        String::from_utf8(app.output).unwrap(),
        "id: 1, text: buy milk, done: false\nid: 2, text: walk dog, done: false\n".to_string()
    );
}

#[test]
fn test_run_count() {
    let mut app = app();
    app.add_todo(&mut Todo::new("buy milk".to_string()))
        .unwrap();
    app.add_todo(&mut Todo::new("walk dog".to_string()))
        .unwrap();
    run(&mut app, args(&["count"]));
    assert_eq!(String::from_utf8(app.output).unwrap(), "2\n".to_string());
}

#[test]
fn test_run_get() {
    let mut app = app();
    app.add_todo(&mut Todo::new("buy milk".to_string()))
        .unwrap();
    run(&mut app, args(&["get", "1"]));
    assert_eq!(
        String::from_utf8(app.output).unwrap(),
        "id: 1, text: buy milk, done: false\n".to_string()
    );
}

#[test]
fn test_run_complete_and_incomplete() {
    let mut app = app();
    app.add_todo(&mut Todo::new("buy milk".to_string()))
        .unwrap();
    run(&mut app, args(&["complete", "1"]));
    run(&mut app, args(&["incomplete", "1"]));
    assert_eq!(
        String::from_utf8(app.output).unwrap(),
        "Completed\nIncompleted\n".to_string()
    );
}

#[test]
fn test_run_delete() {
    let mut app = app();
    app.add_todo(&mut Todo::new("buy milk".to_string()))
        .unwrap();
    run(&mut app, args(&["delete", "1"]));
    run(&mut app, args(&["list"]));
    assert_eq!(
        String::from_utf8(app.output).unwrap(),
        "Deleted\n".to_string()
    );
}
