use super::*;

#[test]
fn test_app_new() {
    let app = App::new(":memory:", std::io::stdout());
    assert_eq!(app.db_path, ":memory:");
    assert!(app.db.table_exists(None, "todos").unwrap());
}
