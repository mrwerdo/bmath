use std::collections::HashMap;
use num::Float;

use crate::parser::grouped;
use crate::lexer::Token;

// pub enum Payload<'a> {
//     Number(Token<'a>),
//     Variable(Token<'a>),
//     Tuple(&'a [Token<'a>]),
//     Expression(&'a [Token<'a>])
// }

// pub struct Node<'a> {
//     payload: Payload<'a>,
//     children: Vec<Node<'a>>
// }

enum EvaluationError {
    TypeError,
    ConversionError
}

#[derive(PartialEq)]
pub struct Number<'a> {
    text: &'a str
}

#[derive(PartialEq)]
pub struct AnyValue<'a> {
    dynamicType: Type,
    value: Number<'a>
}

impl AnyValue<'_> {
    fn convert<C: num::Float>(self: &Self) -> Result<C, EvaluationError> {
        return Result::Ok(self.value as C);
    }
}

pub struct Parameter<'a> {
    name: &'a str,
    expected_type: Type
}
pub trait Callable {
    fn parameters() -> Vec<Parameter<'static>>;
    fn callback(self: &Self, parameters: Vec<AnyValue>) -> Vec<AnyValue>;
}

trait CallableExtension {
    fn call(self: &Self, parameters: Vec<AnyValue>) -> Result<Vec<AnyValue>, EvaluationError>;
}

impl<T> CallableExtension for T where T: Callable {
    fn call(self: &Self, parameters: Vec<AnyValue>) -> Result<Vec<AnyValue>, EvaluationError> {
        let lhs: Vec<Type> = parameters.iter().map(|value| value.dynamicType).collect();
        let rhs: Vec<Type> = T::parameters().iter().map(|v| v.expected_type).collect();
        if  lhs == rhs {
            return Result::Err(EvaluationError::TypeError);
        } else {
            return Result::Ok(self.callback(parameters));
        }
    }
}

pub struct PrimitiveFunction<T> where T: Fn(f64) -> f64 {
    function: T
}

impl<T> Callable for PrimitiveFunction<T> where T: Fn(f64) -> f64 {
    fn parameters() -> Vec<Parameter<'static>> {
        vec![Parameter { name: "arg0", expected_type: Type::Number }]
    }

    fn callback(self: &Self, parameters: Vec<AnyValue>) -> Vec<AnyValue> {
        let first = parameters.first().unwrap();
        first.convert::<f64>();
    }
}



pub struct Function<T> where T: Fn(Vec<Parameter>) -> Vec<AnyValue> {
    callback: T
}

// The question is how can I recover the context where I have the parameters to the function
// call and know their dynamic types, as well as the signature of the function.

pub struct Context<'a> {
    functions: HashMap<&'a str, &'a str>
}

#[derive(PartialEq, Copy, Clone)]
pub enum Type {
    Number
}

impl Context<'_> {
    pub fn new() -> Self {
        return Context { functions: HashMap::new() };
    }

    pub fn function<T>(self: Self, name: &str, callback: T)  -> Self where T: Fn(f64) -> f64 {
        let mut context = self;
        // context.functions.insert(name, Function {
        //     name: name,
        //     callback: callback,
        // });
        return context;
    }

    pub fn interpret<'a>(self: &Self, tokens: &'a [Token<'a>]) -> Result<Token<'a>, &str> {
        return Result::Err("not implemented");
        // return grouped(&tokens[..])//.and_then(|groups | {
            // for group in groups {
            //     let subgroups = self.interpret(&group.tokens);
                
            // }
            // return Result::Err("not implemented");
        // });
    }
}
