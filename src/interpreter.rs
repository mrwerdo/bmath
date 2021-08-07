use std::collections::HashMap;
use crate::objects::{Callable, Object, PrimitiveFunction};
use crate::parser::{grouped, Kind};
use crate::lexer::Token;

pub struct Context<'a> {
    functions: HashMap<&'a str, Box<dyn Callable<'a> + 'a>>
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        return Context { functions: HashMap::new() };
    }

    pub fn function<T: 'a>(self: Self, name: &'a str, function: T) -> Self where T: Fn(f64) -> f64 {
        let mut context = self;
        context.functions.insert(name, Box::new(PrimitiveFunction::<'a, T> { name, function }));
        return context;
    }

    pub fn interpret<'b>(self: &Self, tokens: &'b [Token<'b>]) -> Result<Object, &'b str> where 'a: 'b {
        self._interpret(&tokens, 0)
    }

    fn _interpret<'b>(self: &Self, tokens: &'b [Token<'b>], indent_level: usize) -> Result<Object, &'b str> where 'a: 'b {
        return grouped(&tokens[..]).and_then(|groups | {
            for group in groups {
                let indent = " ".repeat(indent_level*4);
                if Kind::Literals == group.kind {
                    let r = group.tokens.iter().map(|t| -> String { format!("{:?}", t) });
                    println!("{}{}", indent, r.collect::<Vec<String>>().join(", "));
                } else {
                    assert_eq!(Token::OpenParenthesis, *group.tokens.first().unwrap());
                    assert_eq!(Token::CloseParenthesis, *group.tokens.last().unwrap());
                    println!("{}{:?}", indent, Token::OpenParenthesis);
                    let slice = &group.tokens[1..(group.tokens.len()-1)];
                    let _ = self._interpret(&slice, indent_level+1);
                    println!("{}{:?}", indent, Token::CloseParenthesis);
                }
            }
            return Result::Err("not implemented");
        });
    }
}
