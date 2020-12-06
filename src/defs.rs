use std::collections::HashMap;

pub type Function = fn(Vec<Object>) -> Object;

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Number(f32),
    Vector(f32, f32),
    Function(Function),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub fun: String,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Number(f32),
    FunctionCall(FunctionCall),
    Ident(String),
}

pub struct Context {
    pub objects: HashMap<String, Object>,
}

#[derive(Debug)]
pub struct Assignment {
    pub ident: String,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct Output {
    pub filename: String,
    pub expr: Expression,
}

pub enum Statement {
    Assignment(Assignment),
    Output(Output),
}
