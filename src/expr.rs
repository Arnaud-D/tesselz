use crate::defs::{Context, Expression, FunctionCall, Object, ElementType};

impl Expression {
    pub fn eval(&self, context: &Context) -> Object {
        match self {
            Expression::Number(n) => self.eval_number(n),
            Expression::FunctionCall(call) => self.eval_functioncall(call, &context),
            Expression::Ident(ident) => self.eval_ident(ident, &context),
            Expression::Set(set) => self.eval_set(set, &context),
        }
    }

    fn eval_number(&self, n: &f32) -> Object {
        Object::Element(ElementType::Number(*n))
    }

    fn eval_functioncall(&self, call: &FunctionCall, context: &Context) -> Object {
        let opt_fun = context.objects.get(&call.fun);
        match opt_fun {
            Some(fun) => match fun {
                Object::Element(ElementType::Function(f)) => {
                    let args =
                        call.args
                            .iter()
                            .map(|x: &Expression| x.eval(&context))
                            .collect();
                    f(args)
                }
                _ => panic!("`{}` is not a Function", "//TODO//"),
            }
            None => panic!("Function `{}` does not exist", call.fun),
        }
    }

    fn eval_ident(&self, ident: &String, context: &Context) -> Object {
        let opt_object = context.objects.get(ident);
        match opt_object {
            Some(object) => object.clone(),
            None => panic!("Identifier unkown"),
        }
    }

    fn eval_set(&self, set: &Vec<Expression>, context: &Context) -> Object {
        Object::Set(
            set
                .iter()
                .map(|x: &Expression| x.eval(&context))
                .collect()
        )
    }
}
