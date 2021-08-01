#[derive(Debug)]
pub enum Token<'a> {
    OpenParenthesis,
    Separator,
    CloseParenthesis,
    Whitespace(&'a str),
    Literal(&'a str)
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = Vec::new();
    for (index, character) in input.char_indices() {
        match character {
            '(' => {
                tokens.push(Token::OpenParenthesis)
            },
            ',' => {
                tokens.push(Token::Separator)
            },
            ')' => {
                tokens.push(Token::CloseParenthesis)
            },
            _ => {
                let last = tokens.last_mut();
                match last {
                    Some(token) => {
                        if character == ' ' || character == '\t' {
                            if let Token::Whitespace(values) = token {
                                let lower_index = input.subslice_offset(values).unwrap();
                                *token = Token::Whitespace(&input[lower_index..(index+1)]);
                            } else {
                                let s = &input[index..(index+1)];
                                tokens.push(Token::Whitespace(s));
                            }
                        } else {
                            if let Token::Literal(values) = token {
                                let lower_index = input.subslice_offset(values).unwrap();
                                *token = Token::Literal(&input[lower_index..(index+1)]);
                            } else {
                                let s = &input[index..(index+1)];
                                tokens.push(Token::Literal(s));
                            }
                        }
                    },
                    None => {
                        tokens.push(new_token(index, character, input));
                    }
                }
            }
        }
    }
    return tokens;
}

fn new_token<'a>(index: usize, character: char, input: &'a str) -> Token<'a> {
    match character {
        '(' => Token::OpenParenthesis,
        ',' => Token::Separator,
        ')' => Token::CloseParenthesis,
        ' ' => Token::Whitespace(&input[index..(index+1)]),
        '\t' => Token::Whitespace(&input[index..(index+1)]),
        _ => Token::Literal(&input[index..(index+1)])
    }
}

pub trait SubsliceOffset {
    /**
    Returns the byte offset of an inner slice relative to an enclosing outer slice.

    Examples

    ```ignore
    let string = "a\nb\nc";
    let lines: Vec<&str> = string.lines().collect();
    assert!(string.subslice_offset(lines[0]) == Some(0)); // &"a"
    assert!(string.subslice_offset(lines[1]) == Some(2)); // &"b"
    assert!(string.subslice_offset(lines[2]) == Some(4)); // &"c"
    assert!(string.subslice_offset("other!") == None);
    ```
    */
    fn subslice_offset(&self, inner: &Self) -> Option<usize>;
}

impl SubsliceOffset for str {
    fn subslice_offset(&self, inner: &str) -> Option<usize> {
        let self_beg = self.as_ptr() as usize;
        let inner = inner.as_ptr() as usize;
        if inner < self_beg || inner > self_beg.wrapping_add(self.len()) {
            None
        } else {
            Some(inner.wrapping_sub(self_beg))
        }
    }
}
