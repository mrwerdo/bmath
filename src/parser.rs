use super::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum Kind {
    Literals,
    Parenthesis,
    Tuple
}

#[derive(Debug)]
pub struct Grouped<'a> {
    start_index: usize,
    pub tokens: &'a[Token<'a>],
    pub kind: Kind
}

pub fn grouped<'a>(tokens: &'a [Token<'a>]) -> Result<Vec<Grouped<'a>>, &str> {
    let mut expressions: Vec<Grouped> = Vec::new();
    let mut parenthesis_depth = 0;
    for (index, element) in tokens.iter().enumerate() {
        match element {
            Token::OpenParenthesis => {
                if parenthesis_depth == 0 {
                    expressions.push(Grouped { start_index: index, tokens: &tokens[index..(index+1)], kind: Kind::Parenthesis })
                } else {
                    append_to_existing_token(&mut expressions, index, tokens);
                }
                parenthesis_depth += 1;
            },
            Token::Separator => {
                if parenthesis_depth == 1 {
                    let last = expressions.last_mut().unwrap();
                    last.kind = Kind::Tuple;
                }
            },
            Token::CloseParenthesis => {
                if parenthesis_depth == 0 {
                    return Result::Err("unbalanced parenthesis");
                } else {
                    parenthesis_depth -= 1;
                    append_to_existing_token(&mut expressions, index, tokens);
                }
            },
            _ => {
                if parenthesis_depth > 0 {
                    append_to_existing_token(&mut expressions, index, tokens);
                } else {
                    let last = expressions.last();
                    match last {
                        None => expressions.push(Grouped { start_index: index, tokens: &tokens[index..(index+1)], kind: Kind::Literals }),
                        Some(value) => {
                            match value.kind {
                                Kind::Parenthesis => expressions.push(Grouped { start_index: index, tokens: &tokens[index..(index+1)], kind: Kind::Literals }),
                                Kind::Literals => {
                                    append_to_existing_token(&mut expressions, index, tokens);
                                },
                                Kind::Tuple => {
                                    panic!("unreachable code");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return Result::Ok(expressions);
}

fn append_to_existing_token<'a>(expressions: &mut Vec<Grouped<'a>>, index: usize, tokens: &'a [Token<'a>]) {
    let last = expressions.last_mut().unwrap();
    let lower_index = last.start_index;
    last.tokens = &tokens[lower_index..(index+1)];
}