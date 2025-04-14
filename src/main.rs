use std::fs::File;
use std::io::Read;
use nom::IResult;

mod cmd;
use cmd::{Command, read_command};
use monkey_lang_lib::evaluator::Evaluator;
use monkey_lang_lib::lexer::Lexer;
use monkey_lang_lib::lexer::token::Tokens;
use monkey_lang_lib::parser::Parser;

fn main() {
    let code_string = match cmd::read_command() {
        Command::FileRead(file_path) => read_file(file_path).ok(),
        Command::RunInlineCode(code) => Some(code),
        Command::Noop => None,
    };

    if let Some(code_string) = code_string {
        let mut evaluator = Evaluator::new();
        let lex_tokens = Lexer::lex_tokens(code_string.as_bytes());
        match lex_tokens {
            Ok((_, r)) => {
                let tokens = Tokens::new(&r);
                let parsed = Parser::parse_tokens(tokens);
                match parsed {
                    Ok((_, program)) => {
                        let eval = evaluator.eval_program(program);
                        println!("{}", eval);
                    }
                    Err(nom::Err::Error(_)) => println!("Parser error"),
                    Err(nom::Err::Failure(_)) => println!("Parser failure"),
                    Err(nom::Err::Incomplete(_)) => println!("Incomplete parsing"),
                }
            }
            Err(nom::Err::Error(_)) => println!("Lexer error"),
            Err(nom::Err::Failure(_)) => println!("Lexer failure"),
            Err(nom::Err::Incomplete(_)) => println!("Incomplete lexing"),
        }
    }

}

fn read_file(file_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
