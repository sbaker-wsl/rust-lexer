use crate::tokens;

pub struct Lexer {
    input: String,
    position: usize,
    curr_state: tokens::LexerState,
    curr_token: tokens::Token,
    buffer: String
}

impl Lexer {
    pub fn set_input(input : String) -> Self {
        Self {
            input,
            position: 0,
            curr_state: tokens::LexerState::INITIAL,
            curr_token: tokens::Token::EOI,
            buffer: "".to_string()
        }
    }

    pub fn advance(&mut self) -> tokens::Token {
        self.buffer = "".to_string();
        loop {
            match self.input.chars().nth(self.position) {
                Some(c) => {
                    self.buffer.push(c);
                    self.position += 1;
                    match true {
                        _val if c == '(' => {
                            self.curr_state = tokens::LexerState::L_PAREN;
                            self.curr_token = tokens::Token::PARENS_L;
                            break;
                        },

                        _val if c == ')' => {
                            self.curr_state = tokens::LexerState::R_PAREN;
                            self.curr_token = tokens::Token::PARENS_R;
                            break;
                        },

                        _val if c == '[' => {
                            self.curr_state = tokens::LexerState::L_BRACK;
                            self.curr_token = tokens::Token::BRACKET_L;
                            break;
                        },

                        _val if c == ']' => {
                            self.curr_state = tokens::LexerState::R_BRACK;
                            self.curr_token = tokens::Token::BRACKET_R;
                            break;
                        },

                        _val if c == '{' => {
                            self.curr_state = tokens::LexerState::L_BRACE;
                            self.curr_token = tokens::Token::BRACE_L;
                            break;
                        },

                        _val if c == '}' => {
                            self.curr_state = tokens::LexerState::R_BRACE;
                            self.curr_token = tokens::Token::BRACE_R;
                            break;
                        },
                        
                        _val if c == '.' => {
                            self.curr_state = tokens::LexerState::POINT;
                            self.curr_token = tokens::Token::POINT;
                            break;
                        },

                        _val if c == ',' => {
                            self.curr_state = tokens::LexerState::COMMA;
                            self.curr_token = tokens::Token::COMMA;
                            break;
                        },

                        _val if c == ':' => {
                            self.curr_state = tokens::LexerState::COLON;
                            self.curr_token = tokens::Token::COLON;
                            break;
                        },

                        _val if c == ';' => {
                            self.curr_state = tokens::LexerState::SEMICOLON;
                            self.curr_token = tokens::Token::SEMICOLON;
                            break;
                        },

                        _val if c == '-' => {
                            self.curr_state = tokens::LexerState::MINUS;
                            self.curr_token = tokens::Token::SUB;
                            match self.input.chars().nth(self.position) {
                                Some(c) => {
                                    match true {
                                        _val if c == '>' => {
                                            self.curr_state = tokens::LexerState::ARROW_R;
                                            self.curr_token = tokens::Token::ARROW_R;
                                            self.position += 1;
                                            break;
                                        }

                                        _ => break,                   
                                    }
                                },

                                None => break,
                            }
                        }
                            
                        _val if c == '+' => {
                            self.curr_state = tokens::LexerState::PLUS;
                            self.curr_token = tokens::Token::ADD;
                            break;
                        }

                        _val if c == '*' => {
                            self.curr_state = tokens::LexerState::MULT;
                            self.curr_token = tokens::Token::MUL;
                            break;
                        }

                        _val if c == '/' => {
                            self.curr_state = tokens::LexerState::DIVIDE;
                            self.curr_token = tokens::Token::DIV;
                            break;
                        }

                        _val if c == '=' => {
                            self.curr_state = tokens::LexerState::ASSIGN;
                            self.curr_token = tokens::Token::ASSIGN;
                            match self.input.chars().nth(self.position) {
                                Some(c) => {
                                    match c {
                                        '=' => {
                                            self.position += 1;
                                            self.curr_state = tokens::LexerState::EQUALS;
                                            self.curr_token = tokens::Token::EQ;
                                            break;
                                        }

                                        _ => break,
                                    }

                                }
                                None => break,
                            }
                        }

                        _val if c == '<' => {
                            self.curr_state = tokens::LexerState::LESSTHAN;
                            self.curr_token = tokens::Token::LT;
                            match self.input.chars().nth(self.position) {
                                Some(c) => {
                                    match c {
                                        '=' => {
                                            self.curr_state = tokens::LexerState::NOTGREATTHAN;
                                            self.curr_token = tokens::Token::NGT;
                                            self.position += 1;
                                            break;
                                        }
                                        _ => break,
                                    }
                                }
                                None => break,
                            }
                        }

                        _val if c == '>' => {
                            self.curr_state = tokens::LexerState::GREATTHAN;
                            self.curr_token = tokens::Token::GT;
                            match self.input.chars().nth(self.position) {
                                Some(c) => {
                                    match c {
                                        '=' => {
                                            self.curr_state = tokens::LexerState::NOTLESSTHAN;
                                            self.curr_token = tokens::Token::NLT;
                                            self.position += 1;
                                            break;
                                        }
                                        _ => break,
                                    }
                                }
                                None => break,
                            }
                        }

                        _val if c == '!' => {
                            self.curr_state = tokens::LexerState::EXC;
                            match self.input.chars().nth(self.position) {
                                Some(c) => {
                                    match c {
                                        '=' => {
                                            self.curr_state = tokens::LexerState::NEQUAL;
                                            self.curr_token = tokens::Token::NEQ;
                                            self.position += 1;
                                            break;
                                        }
                                        _ => panic!("unkown input"),
                                    }
                                }
                                None => panic!("bad input"),
                            }
                        }


                        val if val == c.is_ascii_lowercase() || c == '_' => {
                            self.curr_state = tokens::LexerState::LETTER;
                            self.curr_token = tokens::Token::ID(c.to_string());
                            let mut is_type = false;
                            loop {
                                match self.input.chars().nth(self.position) {
                                    Some(c) => {
                                        match true {
                                            val if val == c.is_ascii_lowercase() => {
                                                if is_type {
                                                    break;
                                                }
                                                self.buffer.push(c);
                                                self.position += 1;
                                            }
                                            val if val == c.is_ascii_digit() => {
                                                self.buffer.push(c);
                                                self.position += 1;
                                                is_type = true;
                                            }
                                            _ => break,
                                        }
                                    }
                                    None => break,
                                }
                            }
                            match self.buffer.clone().trim() {
                                "not" => {
                                    self.curr_state = tokens::LexerState::NOT;
                                    self.curr_token = tokens::Token::NOT;
                                },

                                "and" => {
                                    self.curr_state = tokens::LexerState::AND;
                                    self.curr_token = tokens::Token::AND;
                                }

                                "or" => {
                                    self.curr_state = tokens::LexerState::OR;
                                    self.curr_token = tokens::Token::OR;
                                }

                                "func" => {
                                    self.curr_state = tokens::LexerState::FUNC;
                                    self.curr_token = tokens::Token::FUNC;
                                }

                                "let" => {
                                    self.curr_state = tokens::LexerState::LET;
                                    self.curr_token = tokens::Token::LET;
                                }

                                "if" => {
                                    self.curr_state = tokens::LexerState::IF;
                                    self.curr_token = tokens::Token::IF;
                                }

                                "else" => {
                                    self.curr_state = tokens::LexerState::ELSE;
                                    self.curr_token = tokens::Token::ELSE;
                                }

                                "while" => {
                                    self.curr_state = tokens::LexerState::WHILE;
                                    self.curr_token = tokens::Token::WHILE;
                                }

                                "print" => {
                                    self.curr_state = tokens::LexerState::PRINT;
                                    self.curr_token = tokens::Token::PRINT;
                                }

                                "i32" => {
                                    self.curr_state = tokens::LexerState::T_I32;
                                    self.curr_token = tokens::Token::TYPE_INT32;
                                }

                                "f32" => {
                                    self.curr_state = tokens::LexerState::T_F32;
                                    self.curr_token = tokens::Token::TYPE_FLT32;
                                }

                                "char" => {
                                    self.curr_state = tokens::LexerState::T_CH;
                                    self.curr_token = tokens::Token::TYPE_CHAR;
                                }

                                _ => {
                                    self.curr_state = tokens::LexerState::ID;
                                    self.curr_token = tokens::Token::ID(self.buffer.clone().trim().to_string());
                                }
                            }
                            break;   
                        }
                        _val if c == ' ' || c == '\n' => {
                            if c == '\n' {
                                println!();
                            }
                            continue;
                        }

                        val if val == c.is_ascii_digit() || c == '.' => {
                            self.curr_state = tokens::LexerState::NUMBER;
                            self.curr_token = tokens::Token::ID(c.to_string());
                            let mut is_float = c == '.';
                            loop {
                                match self.input.chars().nth(self.position) {
                                    Some(c) => {
                                        match true {
                                            val if val == c.is_ascii_digit() => {
                                                self.buffer.push(c);
                                                self.position += 1;
                                            }
                                            _val if c == '.' => {
                                                self.buffer.push(c);
                                                self.position += 1;
                                                is_float = true;
                                            }
                                            _ => break,
                                        }
                                    }
                                    None => break,
                                }
                            }
                            if is_float {
                                self.curr_state = tokens::LexerState::L_F32;
                                match self.buffer.clone().trim().parse::<f32>() {
                                    Ok(float) => self.curr_token = tokens::Token::LIT_FLT32(float),
                                    Err(e) => panic!("failed parsing {}, {}", self.buffer.clone(), e),
                                }
                            } else {
                                self.curr_state = tokens::LexerState::L_I32;
                                match self.buffer.clone().trim().parse::<i32>() {
                                    Ok(int) => self.curr_token = tokens::Token::LIT_INT32(int),
                                    Err(e) => panic!("failed parsing {}, {}", self.buffer.clone(), e),
                                }
                            }
                            break;
                        }

                        _val if c == '\'' => {
                            self.curr_state = tokens::LexerState::L_CH;
                            match self.input.chars().nth(self.position) {
                                Some(c) => {
                                    match true {
                                        val if val == c.is_ascii() => {
                                            self.buffer.push(c);
                                            self.position += 1;
                                            match self.input.chars().nth(self.position) {
                                                Some(c) => {
                                                    match c {
                                                        '\'' => {
                                                            self.buffer.push(c);
                                                            self.position += 1;
                                                            match self.buffer.chars().nth(2) {
                                                                Some(c) => {
                                                                    self.curr_token = tokens::Token::LIT_CHAR(c);
                                                                }
                                                                None => panic!("will never happen"),
                                                            }
                                                        }
                                                        _ => {
                                                            self.buffer.push(c);
                                                            panic!("{} invalid char literal!", self.buffer.clone());
                                                        }
                                                    }
                                                }
                                                None => panic!("invalid char literal!"),
                                            }
                                        }
                                        _ => panic!("invalid usage of ' character"),
                                    }
                                }
                                None => break,
                            }
                            break;
                        }

                        _val if c == '"' => {
                            self.curr_state = tokens::LexerState::L_STR;
                            loop {
                                match self.input.chars().nth(self.position) {
                                    Some(c) => {
                                        self.position += 1;
                                        if c != '"' {
                                            self.buffer.push(c);
                                        } else {
                                            break;
                                        }
                                    }
                                    None => break,
                                }
                            }
                            self.curr_token = tokens::Token::LIT_STRING(self.buffer.clone().trim().replace('"', "").to_string());
                            break;
                        }
                        
                        _ => panic!("invalid: {}", self.buffer),
                    }
                },
                None => {
                    self.curr_state = tokens::LexerState::END;
                    self.curr_token = tokens::Token::EOI;
                    break;
                }
            }
        }

        self.curr_token.clone()
    }

    pub fn curr(&self) -> tokens::Token {
        self.curr_token.clone()
    }

    pub fn print_tokens(&mut self) {
        while self.curr_state != tokens::LexerState::END {
            self.advance();
            if self.curr() == tokens::Token::EOI {
                print!("{:?}", self.curr());
            } else {
                print!("{:?}, ", self.curr());
            }
        }
        println!();
    }
}

