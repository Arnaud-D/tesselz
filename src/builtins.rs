use std::collections::HashMap;

use crate::defs::{Function, Object};

pub fn get_builtins() -> HashMap<String, Function> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("add"), add as Function);
    builtins.insert(String::from("sub"), sub as Function);
    builtins.insert(String::from("mul"), mul as Function);
    builtins.insert(String::from("div"), div as Function);
    builtins
}

fn add(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`add` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    match (o1, o2) {
        (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 + n2),
        _ => panic!("`add` not implemented for {:?} and {:?}", o1, o2)
    }
}

fn sub(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`sub` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    match (o1, o2) {
        (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 - n2),
        _ => panic!("`sub` not implemented for {:?} and {:?}", o1, o2)
    }
}

fn mul(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`mul` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    match (o1, o2) {
        (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 * n2),
        _ => panic!("`mul` not implemented for {:?} and {:?}", o1, o2)
    }
}

fn div(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`div` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    match (o1, o2) {
        (Object::Number(n1), Object::Number(n2)) => {
                if *n2 != 0.0 {
                    Object::Number(n1 / n2)
                } else {
                    panic!("Division by zero")
                }
            }
        _ => panic!("`div` not implemented for {:?} and {:?}", o1, o2)
    }
}
