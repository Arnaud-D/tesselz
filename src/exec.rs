use std::collections::HashMap;

use crate::builtins::get_builtins;
use crate::defs::{Assignment, Context, Expression, FunctionCall, Object, Output, Statement};

impl Context {
    fn default() -> Self {
        let mut objects = HashMap::new();
        for (name, fun) in get_builtins().iter() {
            objects.insert(name.clone(), Object::Function(*fun));
        }
        Self {
            objects
        }
    }
}

impl Context {
    fn exec(&mut self, statement: Statement) {
        match statement {
            Statement::Assignment(assignment) => self.exec_assignement(assignment),
            Statement::Output(output) => self.exec_output(output),
        }
    }

    fn exec_assignement(&mut self, assignment: Assignment) {
        println!(">>> {} = {:?};", assignment.ident, assignment.expr);
        let object = assignment.expr.eval(self);
        self.objects.insert(assignment.ident, object);
        println!("{:?}", object);
    }

    fn exec_output(&self, output: Output) {
        println!(">>> {:?} > \"{}\"", output.expr, output.filename);
        let object = output.expr.eval(self);
        println!("{:?}", object);
    }
}

pub struct Program(pub Vec<Statement>);

pub fn exec(program: Program) {
    let mut context = Context::default();
    let Program(statements) = program;
    for statement in statements {
        context.exec(statement);
    }
}

pub fn get_test_program() -> Program {
    let id1 = String::from("a");
    let id2 = String::from("b");
    let n1 = Expression::Number(2.0);
    let n2 = Expression::Number(3.0);
    let asg1 = Assignment { ident: id1.clone(), expr: n1 };
    let asg2 = Assignment { ident: id2.clone(), expr: n2 };
    let fc1 = Expression::FunctionCall(FunctionCall {
        fun: String::from("add"),
        args: vec!(Expression::Ident(id1.clone()), Expression::Ident(id2.clone())),
    });
    let fc2 = Expression::FunctionCall(FunctionCall {
        fun: String::from("sub"),
        args: vec!(Expression::Ident(id1.clone()), Expression::Ident(id2.clone())),
    });
    let fc3 = Expression::FunctionCall(FunctionCall {
        fun: String::from("mul"),
        args: vec!(Expression::Ident(id1.clone()), Expression::Ident(id2.clone())),
    });
    let fc4 = Expression::FunctionCall(FunctionCall {
        fun: String::from("div"),
        args: vec!(Expression::Ident(id1.clone()), Expression::Ident(id2.clone())),
    });
    let output1 = Output {
        filename: String::from("add.svg"),
        expr: fc1,
    };
    let output2 = Output {
        filename: String::from("sub.svg"),
        expr: fc2,
    };
    let output3 = Output {
        filename: String::from("mul.svg"),
        expr: fc3,
    };
    let output4 = Output {
        filename: String::from("div.svg"),
        expr: fc4,
    };
    let program = Program(vec!(
        Statement::Assignment(asg1),
        Statement::Assignment(asg2),
        Statement::Output(output1),
        Statement::Output(output2),
        Statement::Output(output3),
        Statement::Output(output4),
    ));
    program
}
