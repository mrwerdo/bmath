mod lexer;
mod parser;
mod interpreter;
mod objects;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let context = interpreter::Context::new()
        .function("sin", f64::sin);

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("bmath > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let tokens= lexer::tokenize(&line);
                println!("tokens: {:?}", tokens);
                match context.interpret(&tokens) {
                    Ok(result) => println!("{:?}", result),
                    Err(error) => println!("error: {}", error)
                }
                
            },
            Err(ReadlineError::Interrupted) => {
                continue
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
