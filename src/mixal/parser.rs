use super::lexer::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Program(Vec<Instruction>),
    Instruction(Instruction),
    Directive(Directive),
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstNode::Program(inst) => {
                for instruction in inst {
                    write!(f, "{}\n", instruction)?;
                }
            },
            AstNode::Instruction(inst) => write!(f, "{}", inst)?,
            AstNode::Directive(_) => {},
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: String,
    operands: Vec<Operand>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.opcode)?;
        for (i, operand) in self.operands.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", operand)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Register(String),
    Number(i32),
    Label(String),
    Directive(String),
    Parenthesized(Box<Operand>),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Register(name) => write!(f, "{}", name),
            Operand::Number(name) => write!(f, "{}", name),
            Operand::Label(name) => write!(f, "{}", name),
            Operand::Directive(name) => write!(f, "{}", name),
            Operand::Parenthesized(name) => write!(f, "({})", name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Directive {
    Loc(i32),
    Is(String),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<AstNode, &'static str> {
        let mut instructions = Vec::new();
        let mut directives = Vec::new();

            while let Some(token) = self.tokens.first() {
                match token {
                    Token::Instruction(_) => {
                    let instruction = self.parse_instruction()?;
                    instructions.push(instruction);
                },
                Token::Loc | Token::Is => {
                    let directive = self.parse_directive()?;
                    directives.push(directive);
                },
                Token::Error(ref message, _, _) => return Err("Error"),
                _ => return Err("Expected an instruction or directive")
            }
        }

        Ok(AstNode::Program(instructions))
    }

    fn parse_instruction(&mut self) -> Result<Instruction, &'static str> {
        let opcode = match self.tokens.remove(0) {
            Token::Instruction(opcode) => opcode,
            _ => return Err("Expected an instruction"),
        };

        let mut operands = Vec::new();
        while let Some(token) = self.tokens.get(0) {
            match token {
                Token::Register(_) | Token::Number(_) | Token::Label(_) | Token::Directive(_) | Token::ParenOpen=> {
                    let operand = self.parse_operand()?;
                    operands.push(operand);
                },
                Token::Comma => {
                    self.tokens.remove(0);
                },
                _ => break,
            }
        }
        Ok(Instruction { opcode, operands })
    }

    fn parse_directive(&mut self) -> Result<Directive, &'static str> {
        match self.tokens.remove(0) {
            Token::Loc => {
                let value = match self.tokens.remove(0) {
                    Token::Number(value) => value,
                    _ => return Err("Expected a number after LOC directive"),
                };
                Ok(Directive::Loc(value))
            },
            Token::Is => {
                let name = match self.tokens.remove(0) {
                    Token::Number(number) => number.to_string(),
                    Token::Label(name) => name,
                    _ => return Err("Expected a label after IS directive")
                };
                Ok(Directive::Is(name))
            },
            _ => Err("Expected a directive"),
        }
    }

    fn parse_operand(&mut self) -> Result<Operand, &'static str> {
        match self.tokens.remove(0) {
            Token::Register(name) => Ok(Operand::Register(name)),
            Token::Number(value) => Ok(Operand::Number(value)),
            Token::Label(name) => Ok(Operand::Label(name)),
            Token::Directive(name) => Ok(Operand::Directive(name)),
            Token::ParenOpen => {
                let operand = self.parse_operand()?;
                match self.tokens.remove(0) {
                    Token::ParenClose => Ok(Operand::Parenthesized(Box::new(operand))),
                    _ => Err("Expected a closing parenthesis"),
                }
            },
            _ => Err("Expected an operand"),
        }
    }
}
