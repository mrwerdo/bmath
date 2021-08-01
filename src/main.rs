mod lexer;
mod parser;
mod interpreter;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let context = interpreter::Context::new().function("sin", |value| { f64::sin(value) });

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("bmath > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let tokens= lexer::tokenize(&line);
                println!("tokens: {:?}", tokens);
                context.interpret(&tokens);
                
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
