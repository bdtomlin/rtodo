use super::*;

#[test]
fn test_conn() {
    let conn = conn(":memory:");
    assert!(conn.is_ok());
}

#[test]
fn test_ensure_table() {
    let conn = conn(":memory:").unwrap();

    assert!(conn.table_exists(None, "todos").unwrap());

    assert_eq!(
        conn.column_metadata(None, "todos", "id").unwrap(),
        (Some(c"INTEGER"), Some(c"BINARY"), false, true, false)
    );
    assert_eq!(
        conn.column_metadata(None, "todos", "text").unwrap(),
        (Some(c"TEXT"), Some(c"BINARY"), true, false, false)
    );
    assert_eq!(
        conn.column_metadata(None, "todos", "done").unwrap(),
        (Some(c"BOOLEAN"), Some(c"BINARY"), true, false, false)
    );
}
