use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
mod helper;
mod grammar;
mod lexer;
mod tokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rules = vec!(
        grammar::Rule::new('N', "n+&N"),
        grammar::Rule::new('N', "E"),
        grammar::Rule::new('E', "(e/x)-E"),
        grammar::Rule::new('E', "z"),
    );
    let grammar = grammar::Grammar::from_rules(&rules);
    let mut derivation = grammar::Derivation::new(&grammar);

    if args.len() > 1 {
        match args[1].as_str() {
            "help" => { 
                match args.len() {
                    2 => helper::help_display(""),
                    3 => helper::help_display(args[2].as_str()),
                    _=> panic!("help only accepts one command inquiry at a time!"),
                }
            }

            "list" => {
                match args.len() {
                    3 => match args[2].as_str() {
                        "commands" => helper::print_list(),
                        "rules" => helper::print_rules(),
                        "tokens" => helper::print_tokens_list(),
                        _=> panic!("invalid command: {} {}", args[1], args[2]),
                    }
                    _=> panic!("invalid command for list! NOTE: see usage in help!"),
                }   
            }

            "print" => {
                match args.len() {
                    3 => {
                        clearscreen::clear().expect("failed to clear screen");
                        let contents = fs::read_to_string(args[2].as_str()).expect("cannot read file doesn't exist! (make sure to include extenstion!)");
                        println!("{contents}");
                    },
                    4 => match args[3].as_str() {
                        "--numbered" => {
                            clearscreen::clear().expect("failed to clear screen");
                            let file = File::open(args[2].as_str()).expect("cannot find file");
                            let reader = BufReader::new(file);
                            for (line_num, line_result) in reader.lines().enumerate() {
                                let line = line_result.expect("something");
                                println!("{}: {}", line_num + 1, line);
                            }
                        }
                        _=> panic!("bad flag!"),
                    },
                    _=> panic!("invalid command!"),
                }
            }

            "derive" => {
                match args.len() {
                    3 => match args[2].as_str() {
                        "random" => {
                            let sent : Option<grammar::Sentenial> = grammar::print_random(100);
                            match sent {
                                Some(value) => value.print(),
                                None => println!("Invalid!"),
                            }
                        },
                        _ => match args[2].parse::<i32>() {
                            Ok(num) => {
                                derivation.derive_leftmost(&grammar, num as usize);
                                derivation.print();
                            },
                            Err(_) => println!("bad input! not a integer!"),
                        },
                    }
                    _=> for i in 2..args.len() {
                        match args[i].parse::<i32>() {
                            Ok(num) => {
                                derivation.derive_leftmost(&grammar, num as usize);
                                derivation.print();
                            },
                            Err(_) => println!("bad input! not a integer!"),
                        }
                    },
                }
            }

            "tokenize" => {
                match args.len() {
                    3 => {
                        clearscreen::clear().expect("failed to clear screen");
                        let contents = fs::read_to_string(args[2].as_str()).expect("cannot read file!");
                        let mut lexer = lexer::Lexer::set_input(contents);
                        lexer.print_tokens();
                    }
                    _ => panic!("invalid command!"),
                }
            }

            _ => println!("other"),
        }
    }
}
