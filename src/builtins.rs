use std::{
    collections::HashMap,
};

use crate::defs::{
    ElementType::{self, Number, Point, Polygon, Vector},
    FunctionType,
    Object::{self, Element, Set},
};

pub fn get_builtins() -> HashMap<String, FunctionType> {
    let mut builtins = HashMap::new();

    // Constructors
    builtins.insert(String::from("vector"), vector as FunctionType);
    builtins.insert(String::from("point"), point as FunctionType);
    builtins.insert(String::from("polygon"), polygon as FunctionType);

    // Binary operators
    builtins.insert(String::from("add"), add as FunctionType);
    builtins.insert(String::from("sub"), sub as FunctionType);
    builtins.insert(String::from("mul"), mul as FunctionType);
    builtins.insert(String::from("div"), div as FunctionType);

    // Transformations
    builtins.insert(String::from("translate"), translate as FunctionType);
    builtins.insert(String::from("rotate"), rotate as FunctionType);

    builtins
}

fn map2(fun: Box<dyn Fn(ElementType, ElementType) -> ElementType>, o1: Object, o2: Object) -> Object {
    match (o1.clone(), o2.clone()) {
        (Element(e1), Element(e2)) => Element(fun(e1, e2)),
        (Set(s), Element(e)) => {
            let mut result = Vec::new();
            for es in s {
                match es {
                    Element(es) => result.push(Element(fun(es.clone(), e.clone()))),
                    _ => panic!("Set of set should not happen here.")
                }
            }
            Set(result)
        }
        (Element(e), Set(s)) => {
            let mut result = Vec::new();
            for es in s {
                match es {
                    Element(es) => result.push(Element(fun(e.clone(), es.clone()))),
                    _ => panic!("Set of set should not happen here.")
                }
            }
            Set(result)
        }
        (Set(s1), Set(s2)) => {
            let mut result = Vec::new();
            for e1 in s1 {
                for e2 in s2.clone() {
                    match (e1.clone(), e2) {
                        (Element(e1), Element(e2)) => result.push(Element(fun(e1.clone(), e2.clone()))),
                        _ => panic!("Set of set should not happen here.")
                    }
                }
            }
            Set(result)
        }
    }
}

fn vector(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`vector` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    map2(Box::new(vector_elem), o1.clone(), o2.clone())
}

fn vector_elem(e1: ElementType, e2: ElementType) -> ElementType {
    match (e1.clone(), e2.clone()) {
        (Number(x), Number(y)) => Vector(x, y),
        _ => panic!("`vector` not implemented for {:?} and {:?}", e1, e2)
    }
}

fn point(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`point` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    map2(Box::new(point_elem), o1.clone(), o2.clone())
}

fn point_elem(e1: ElementType, e2: ElementType) -> ElementType {
    match (e1.clone(), e2.clone()) {
        (Number(x), Number(y)) => Point(x, y),
        _ => panic!("`point` not implemented for {:?} and {:?}", e1, e2)
    }
}

fn polygon(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 1 {
        panic!("`polygon` called with {} arguments. Exactly 1 expected (set of points).", length);
    }
    let mut polygon = Vec::new();
    match objects[0].clone() {
        Set(elements) => {
            for e in elements {
                match e {
                    Element(Point(x, y)) => polygon.push((x, y)),
                    _ => panic!("`polygon` received {:?} which is not a Point element.", e)
                }
            }
            Element(Polygon(polygon))
        }
        _ => panic!("Argument to `polygon must be a set.")
    }
}


fn add(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`add` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    map2(Box::new(add_elem), o1.clone(), o2.clone())
}

fn add_elem(e1: ElementType, e2: ElementType) -> ElementType {
    match (e1.clone(), e2.clone()) {
        (Number(n1), Number(n2)) => Number(n1 + n2),
        (Vector(x1, y1), Vector(x2, y2)) => Vector(x1 + x2, y1 + y2),
        _ => panic!("`add` not implemented for {:?} and {:?}", e1, e2)
    }
}

fn sub(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`sub` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    map2(Box::new(sub_elem), o1.clone(), o2.clone())
}

fn sub_elem(e1: ElementType, e2: ElementType) -> ElementType {
    match (e1.clone(), e2.clone()) {
        (Number(n1), Number(n2)) => Number(n1 - n2),
        (Vector(x1, y1), Vector(x2, y2)) => Vector(x1 - x2, y1 - y2),
        _ => panic!("`sub` not implemented for {:?} and {:?}", e1, e2)
    }
}

fn mul(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`mul` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    map2(Box::new(mul_elem), o1.clone(), o2.clone())
}

fn mul_elem(e1: ElementType, e2: ElementType) -> ElementType {
    match (e1.clone(), e2.clone()) {
        (Number(n1), Number(n2)) => Number(n1 * n2),
        (Number(k), Vector(x, y)) => Vector(k * x, k * y),
        _ => panic!("`mul` not implemented for {:?} and {:?}", e1, e2)
    }
}

fn div(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`div` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    map2(Box::new(div_elem), o1.clone(), o2.clone())
}

fn div_elem(e1: ElementType, e2: ElementType) -> ElementType {
    match (e1.clone(), e2.clone()) {
        (Number(n1), Number(n2)) => Number(n1 / n2),
        _ => panic!("`div` not implemented for {:?} and {:?}", e1, e2)
    }
}

fn translate(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 2 {
        panic!("`translate` called with {} arguments. 2 expected.", length);
    }
    let (o1, o2) = (&objects[0], &objects[1]);
    map2(Box::new(translate_elem), o1.clone(), o2.clone())
}

fn translate_elem(e1: ElementType, e2: ElementType) -> ElementType {
    match (e1.clone(), e2.clone()) {
        (Polygon(coords), Vector(dx, dy)) => {
            let mut new_coords = Vec::new();
            for coord in coords {
                let (x, y) = coord;
                new_coords.push((x + dx, y + dy));
            }
            Polygon(new_coords)
        }
        (Point(x, y), Vector(dx, dy)) => Point(x + dx, y + dy),
        _ => panic!("`translate` not implemented for {:?} and {:?}", e1, e2)
    }
}

fn rotate(objects: Vec<Object>) -> Object {
    let length = objects.len();
    if length != 3 {
        panic!("`translate` called with {} arguments. 3 expected.", length);
    }
    let (o1, o2, o3) = (&objects[0], &objects[1], &objects[2]);
    match o3.clone() {
        Element(e3) => {
            let e3b = e3.clone();
            let rotate_elem_point =
                move |e1: ElementType, e2: ElementType| {
                    rotate_elem(e1, e2, e3b.clone())
                };
            map2(Box::new(rotate_elem_point), o1.clone(), o2.clone())
        }
        _ => panic!("`rotate` expects a point as a third argument, not a set.")
    }
}

fn rotate_elem(e1: ElementType, e2: ElementType, e3: ElementType) -> ElementType {
    match e3.clone() {
        Point(x0, y0) => {
            match (e1.clone(), e2.clone()) {
                (Polygon(coords), Number(t)) => {
                    let mut new_coords = Vec::new();
                    for coord in coords {
                        let (x, y) = coord;
                        let (dx, dy) = (x - x0, y - y0);
                        let dx1 = dx * t.cos() - dy * t.sin();
                        let dy1 = dx * t.sin() + dy * t.cos();
                        new_coords.push((x0 + dx1, y0 + dy1));
                    }
                    Polygon(new_coords)
                }
                (Point(x, y), Vector(dx, dy)) => Point(x + dx, y + dy),
                _ => panic!("`translate` not implemented for {:?} and {:?}", e1, e2)
            }
        }
        _ => panic!("Trying to use `rotate` with third argument not being a point.")
    }
}