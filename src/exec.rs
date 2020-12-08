use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::builtins::get_builtins;
use crate::defs::{Assignment, Context, Expression, FunctionCall, Object, Render, Statement, ElementType};

impl Context {
    fn default() -> Self {
        let mut objects = HashMap::new();
        for (name, fun) in get_builtins().iter() {
            objects.insert(name.clone(), Object::Element(ElementType::Function(*fun)));
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
            Statement::Render(render) => self.exec_render(render),
        }
    }

    fn exec_assignement(&mut self, assignment: Assignment) {
        println!(">>> {} = {:?};", assignment.ident, assignment.expr);
        let object = assignment.expr.eval(self);
        println!("{:?}", object);
        self.objects.insert(assignment.ident, object);
    }

    fn exec_render(&self, render: Render) {
        println!(">>> {:?} > \"{}\"", render.expr, render.filename);
        let object = render.expr.eval(self);
        println!("{:?}", object);
        let file = File::create(render.filename).unwrap();
        let start = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1000 1000\">";
        let coll_obj = match object {
            Object::Element(ElementType::Polygon(p)) => vec!(Object::Element(ElementType::Polygon(p))),
            Object::Set(s) => s,
            _ => panic!("Expr does not yield polygon or polygon set. Cannot render.")
        };
        let end = "</svg>";
        writeln!(&file, "{}", start).unwrap();
        for obj in coll_obj {
            let coord_str = match obj {
                Object::Element(ElementType::Polygon(points)) => get_coord_str(points),
                _ => panic!("The programmer is a bobo fool.")
            };
            writeln!(&file, "<polygon points=\"{}\" fill=\"grey\" stroke=\"black\" />", coord_str).unwrap();
        }

        writeln!(&file, "{}", end).unwrap();
    }
}

fn get_coord_str(points: Vec<(f32, f32)>) -> String {
    let mut coord_str = String::new();
    for (x, y) in points {
        let frag = format!("{},{} ", x, y);
        coord_str += &*frag;
    }
    coord_str
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
    let render1 = Render {
        filename: String::from("add.svg"),
        expr: fc1,
    };
    let render2 = Render {
        filename: String::from("sub.svg"),
        expr: fc2,
    };
    let render3 = Render {
        filename: String::from("mul.svg"),
        expr: fc3,
    };
    let render4 = Render {
        filename: String::from("div.svg"),
        expr: fc4,
    };
    let program = Program(vec!(
        Statement::Assignment(asg1),
        Statement::Assignment(asg2),
        Statement::Render(render1),
        Statement::Render(render2),
        Statement::Render(render3),
        Statement::Render(render4),
    ));
    program
}
