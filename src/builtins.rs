use std::collections::HashMap;

use crate::defs::{Function, Object};

pub fn get_builtins() -> HashMap<String, Function> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("add"), add as Function);
    builtins.insert(String::from("sub"), sub as Function);
    builtins.insert(String::from("mul"), mul as Function);
    builtins.insert(String::from("div"), div as Function);
    builtins.insert(String::from("vector"), vector as Function);
    builtins.insert(String::from("point"), point as Function);
    builtins.insert(String::from("polygon"), polygon as Function);
    builtins
}

fn collape_set(object: Object) -> Object {
    match object {
        Object::Set(s) => {
            let mut set = Vec::new();
            for e in s {
                match *e {
                    Object::Set(se) => {
                        for ee in se {
                            set.push(ee)
                        }
                    }
                    o => set.push(Box::new(o))
                }
            }
            Object::Set(set)
        }
        o => o
    }
}

fn add(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`add` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    match (o1, o2) {
        (Object::Number(n1), Object::Number(n2)) => Object::Number(n1 + n2),
        (Object::Vector(x1, y1), Object::Vector(x2, y2)) => Object::Vector(x1 + x2, y1 + y2),
        (Object::Point(x, y), Object::Vector(dx, dy)) => Object::Point(x + dx, y + dy),
        (Object::Polygon(coords), Object::Vector(dx, dy)) => {
            let mut new_coords = Vec::new();
            for coord in coords {
                let (x, y) = *coord;
                new_coords.push((x + dx, y + dy));
            }
            Object::Polygon(new_coords)
        }
        (Object::Set(s), o) => {
            let mut result_set = Vec::new();
            for element in s {
                let result = Box::new(add(vec!(*element.clone(), o.clone())));
                result_set.push(result);
            }
            collape_set(Object::Set(result_set))
        }
        (o, Object::Set(s)) => {
            let mut result_set = Vec::new();
            for element in s {
                let result = Box::new(add(vec!(o.clone(), *element.clone())));
                result_set.push(result);
            }
            collape_set(Object::Set(result_set))
        }
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
        (Object::Vector(x1, y1), Object::Vector(x2, y2)) => Object::Vector(x1 - x2, y1 - y2),
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
        (Object::Number(k), Object::Vector(x, y)) => Object::Vector(k * x, k * y),
        (Object::Set(s), o) => {
            let mut result_set = Vec::new();
            for element in s {
                let result = Box::new(mul(vec!(*element.clone(), o.clone())));
                result_set.push(result);
            }
            collape_set(Object::Set(result_set))
        }
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

fn vector(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`vector` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    match (o1, o2) {
        (Object::Number(x), Object::Number(y)) => Object::Vector(*x, *y),
        _ => panic!("`vector` not implemented for {:?} and {:?}", o1, o2)
    }
}

fn point(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`point` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    match (o1, o2) {
        (Object::Number(x), Object::Number(y)) => Object::Point(*x, *y),
        _ => panic!("`point` not implemented for {:?} and {:?}", o1, o2)
    }
}

fn polygon(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length < 2 {
        panic!("`polygon` called with {} arguments. At leats 2 expected.", length);
    }
    let mut polygon = Vec::new();
    for o in objects {
        match o {
            Object::Point(x, y) => polygon.push((x, y)),
            _ => panic!("`polygon` received {:?} which is not a Point.", o)
        }
    }
    Object::Polygon(polygon)
}
