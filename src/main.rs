#[macro_use]
extern crate lalrpop_util;

use crate::exec::{exec, Program};

mod defs;
mod exec;
mod builtins;
mod expr;

lalrpop_mod!(pub parser);

#[test]
fn parser1() {
    assert!(parser::StatementParser::new().parse("a = 22.0;").is_ok());
    assert!(parser::StatementParser::new().parse("abc = 22.2;").is_ok());
    assert!(parser::StatementParser::new().parse("abc = a;").is_ok());
    assert!(parser::StatementParser::new().parse("00 = a;").is_err());
    assert!(parser::StatementParser::new().parse("0aa = a;").is_err());

    assert!(parser::StatementParser::new().parse("22.3 > \"a\";").is_ok());

    assert!(parser::StatementParser::new().parse("a = add(n1, n2);").is_ok());

    assert!(parser::StatementsParser::new().parse("a = add(1.0, 3.14); b  = add(a, 1.0);").is_ok());
}

fn get_test_source() -> &'static str {
    r#"
        a = 3.14;
        b = add(mul(2.0, a), 1.0);
        b > "whatever.txt";
    "#
}

fn main() {
    let parser = parser::StatementsParser::new();
    let statements = parser.parse(get_test_source()).unwrap();
    let program = Program(statements);
    exec(program);
}
