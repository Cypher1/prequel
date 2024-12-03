fn main() {
    use pg_parse::ast::Node;

    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(&path).expect("?");
    let result = pg_parse::parse(&input);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(matches!(*&result[0], Node::SelectStmt(_)));
    dbg!(&result[0]);
}

#[test]
fn sanity_test() {
    use pg_parse::ast::Node;

    let result = pg_parse::parse("SELECT * FROM contacts");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(matches!(*&result[0], Node::SelectStmt(_)));

    // We can also convert back to a string, if the `str` feature is enabled (enabled by default).
    assert_eq!(result[0].to_string(), "SELECT * FROM contacts");
}
