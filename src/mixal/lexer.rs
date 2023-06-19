use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    Instruction(String),
    Register(String),
    Number(i32),
    Label(String),
    Directive(String),
    Loc,
    Is,
    ParenOpen,
    ParenClose,
    Comma,
    Error(String, usize, usize),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Instruction(x) => write!(f, "Instruction({})", x),
            Token::Register(x) => write!(f, "Register({})", x),
            Token::Number(x) => write!(f, "Number({})", x),
            Token::Label(x) => write!(f, "Label({})", x),
            Token::Directive(x) => write!(f, "Directive({})", x),
            Token::Loc => write!(f, "LOC"),
            Token::Is => write!(f, "IS"),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Error(x, start, end) => write!(f, "Error({},{},{})", x, start,end)
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<dyn Iterator<Item = char> + 'a>,
    lookahead: Option<char>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = Box::new(input.chars());
        let lookahead = chars.next();
        Self {
            input, chars, lookahead, position: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.lookahead {
            let token = match c {
                '#' => {
                    self.lex_comment();
                    continue;
                },
                'I' => self.lex_is(),
                '(' => {
                    self.next_char();
                    Token::ParenOpen
                },
                ')' => {
                    self.next_char();
                    Token::ParenClose
                },
                ',' => {
                    self.next_char();
                    Token::Comma
                }
                ':' => self.lex_label(),
                '$' => self.lex_dollar(),
                _ if c.is_whitespace() => {
                    self.next_char();
                    continue;
                },
                _ if c.is_digit(10) => self.lex_number(),
                _ if c.is_alphabetic() => self.lex_instruction(),
                _ => {
                    let start = self.current_position();
                    self.lex_error(start)
                },
            };
            tokens.push(token);
        }
        tokens
    }
    
    fn lex_register(&mut self) -> Token {
        let mut name = String::new();
        while let Some(c) = self.lookahead {
            if c.is_alphanumeric() {
                name.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Token::Register(name)
    }

    fn lex_label(&mut self) -> Token {
        let mut name = String::new();
        while let Some(c) = self.lookahead {
            if c.is_alphanumeric() || c == ':' {
                name.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Token::Label(name)
    }

    fn lex_directive(&mut self) -> Token {
        let mut name = String::new();
        while let Some(c) = self.lookahead {
            if c.is_alphanumeric() || c == '$' {
                name.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Token::Directive(name)
    }

    fn lex_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(c) = self.lookahead {
            if c.is_digit(10) {
                number.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Token::Number(number.parse().unwrap())
    }

    fn lex_instruction(&mut self) -> Token {
        let mut name = String::new();
        while let Some(c) = self.lookahead {
            if c.is_alphanumeric() || c == '_' {
                name.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        if name == "LOC" {
            Token::Loc
        } else {
            Token::Instruction(name)
        }
    }

    fn lex_error(&mut self, start:usize) -> Token {
        let mut end = start;
        while let Some(c) = self.lookahead {
            if c.is_whitespace() {
                break;
            }
            end += 1;
            self.next_char();
        }
        let message = format!("Unexpected token: {}", &self.input[start..end]);
        Token::Error(message, start, end)
    }


    fn lex_comment(&mut self) {
        while let Some(c) = self.lookahead {
            if c != '\n' {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn lex_dollar(&mut self) -> Token {
        self.next_char(); // consume the '$'
        match self.lookahead {
            Some(c) if c.is_digit(10) => self.lex_register(),
            _ => self.lex_directive(),
        }
    }

    fn lex_is(&mut self) -> Token {
        self.consume_string("IS");
        Token::Is
    }

    fn consume_string(&mut self, string: &str) {
        for _ in string.chars() {
            self.next_char();
        }
    }

    fn next_char(&mut self) {
        self.lookahead = self.chars.next();
        self.position += 1;
    }

    fn current_position(&self) -> usize {
        self.position
    }
}
