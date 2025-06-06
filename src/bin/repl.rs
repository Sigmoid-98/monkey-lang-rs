use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use nom::IResult;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::{Cmd, CompletionType, Config, Context, EditMode, Editor, KeyEvent, OutputStreamType};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{MatchingBracketValidator, ValidationContext, ValidationResult, Validator};
use rustyline_derive::Helper;
use monkey_lang_lib::evaluator::Evaluator;
use monkey_lang_lib::lexer::Lexer;
use monkey_lang_lib::lexer::token::Tokens;
use monkey_lang_lib::parser::Parser;

fn main() {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();
    let h = MyHelper {
        complete: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('N'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('P'), Cmd::HistorySearchBackward);
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    println!();
    println!("This is the monkey language repl v0.0.1");
    println!("Press Ctrl-D or enter \"quit\" to exit.");
    println!();

    let mut evaluator = Evaluator::new();
    let mut count = 1;

    loop {
        let p = format!("{}> ", count);
        rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let lex_tokens = Lexer::lex_tokens(line.as_bytes());
                match lex_tokens {
                    Ok((_, r)) => {
                        let tokens = Tokens::new(&r);
                        let parsed = Parser::parse_tokens(tokens);
                        match parsed {
                            Ok((_, program)) => {
                                let eval  =evaluator.eval_program(program);
                                println!("{}", eval);
                            },
                            Err(nom::Err::Error(_)) => println!("Parser error"),
                            Err(nom::Err::Failure(_)) => println!("Parser failure"),
                            Err(nom::Err::Incomplete(_)) => println!("Incomplete parsing"),
                        }
                    },
                    Err(nom::Err::Error(_)) => println!("Lexer error"),
                    Err(nom::Err::Failure(_)) => println!("Lexer failure"),
                    Err(nom::Err::Incomplete(_)) => println!("Incomplete lexing"),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        count += 1;
    }
}

#[derive(Helper)]
struct MyHelper {
    complete: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.complete.complete(line, pos, ctx)
    }
}

impl Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for MyHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, default: bool) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_string() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for MyHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}