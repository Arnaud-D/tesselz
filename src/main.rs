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
        k = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};
        A = point(0, 0);
        B = point(50, 0);
        C = point(50, 50);
        D = point(0, 50);
        square = polygon({A, B, C, D});
        square_offset = translate(square, vector(550, 550));
        angles = mul(div(6.28, 12), k);
        pattern = rotate(square_offset, angles, point(500, 500));
        pattern > "output.svg";
    "#
}

fn main() {
    let parser = parser::StatementsParser::new();
    let statements = parser.parse(get_test_source()).unwrap();
    let program = Program(statements);
    exec(program);
}
