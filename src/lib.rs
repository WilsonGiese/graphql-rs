pub mod graphql; // synthesized by LALRPOP

#[test]
fn test_IntValue() {
    assert!(graphql::parse_IntValue("1").is_ok());
    assert!(graphql::parse_IntValue("-1").is_ok());
    assert!(graphql::parse_IntValue("123").is_ok());
    assert!(graphql::parse_IntValue("-123").is_ok());
}

#[test]
fn test_Comment() {
    assert!(graphql::parse_Comment("# Hello, World\nX").is_ok());
    assert!(graphql::parse_Comment("# Hello, World\r").is_ok());
    assert!(graphql::parse_Comment("# Hello, World\r\n").is_ok());
    assert!(graphql::parse_Comment("#a\tb\n").is_ok());
    assert!(graphql::parse_Comment("#\n").is_ok());
    assert!(graphql::parse_Comment("#   \n").is_ok());
    assert!(graphql::parse_Comment("###\n").is_ok());
}
