use std::fmt;
use std::fmt::{Display, Formatter};
use crate::code::token::{Assign, Logical, Operator, Statement, Symbol, Token, TypeName, TypeValue};

pub struct Lexer {
    tokens: Vec<Token>,
    pos: usize,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        let tokens = lexer(code);

        Self { tokens, pos: 0 }
    }
}

pub fn lexer(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut double_state = false;
    let mut string_flag = false;

    // Iterate through the string by character
    for (i, c) in code.chars().enumerate() {
        // Check if is a String
        if c == '"' && !string_flag {
            string_flag = true;
            buffer.clear();
            continue;
        }
        if string_flag {
            buffer.push(c);
            if c == '"' {
                string_flag = false;
                tokens.push(Token::TypeValue(TypeValue::QuotedString(
                    buffer.trim_matches('"').to_string(),
                )));
                buffer.clear();
            }
            continue;
        }

        // Check if the current character is a letter or a digit
        if c.is_alphabetic() || c.is_numeric() || (c == '_' && !buffer.is_empty()) {
            buffer.push(c);

            // If the next character is not a digit, add the numeric literal to the tokens list
            if (i + 1 == code.len() || !code.chars().nth(i + 1).unwrap().is_alphanumeric())
                && buffer.chars().all(char::is_numeric)
            {
                tokens.push(Token::TypeValue(TypeValue::I32(buffer.parse().unwrap())));
                buffer.clear();
            }
            continue;
        }

        // Check if the character has been used in a double state
        if double_state {
            double_state = false;
            continue;
        }
        // Check for Symbol that is made with two Symbols
        match c {
            '-' => {
                // Check if the Double Symbol is a Arrow
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '>' {
                    tokens.push(Token::Symbol(Symbol::Arrow));
                    buffer.clear();
                    double_state = true;
                    continue;
                } else if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::SubAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '=' => {
                // Check if the Double Symbol is a Equals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Logical(Logical::Equals));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '!' => {
                // Check if the Double Symbol is a NotEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Logical(Logical::NotEquals));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '+' => {
                // Check if the Double Symbol is a PlusEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::AddAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '*' => {
                // Check if the Double Symbol is a StarEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::MulAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '/' => {
                // Check if the Double Symbol is a SlashEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::DivAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '%' => {
                // Check if the Double Symbol is a PercentEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::RemAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }

            ':' => {
                // Check if the Double Symbol is a DoubleColon
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == ':' {
                    tokens.push(Token::Symbol(Symbol::DoubleColon));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            _ => {}
        }

        // If the buffer is not empty, check if it contains a keyword or identifier
        if !buffer.is_empty() {
            let token = match buffer.as_str() {
                "let" => Token::Statement(Statement::Let),
                "fn" => Token::Statement(Statement::Function),
                //"->" => Token::Symbol(Symbol::Arrow),
                //"::" => Token::Symbol(Symbol::DoubleColon),
                "return" => Token::Statement(Statement::Return),
                "import" => Token::Statement(Statement::Import),
                //"==" => Token::Logical(Logical::Equals),
                //"!=" => Token::Logical(Logical::NotEquals),
                "if" => Token::Statement(Statement::If),
                "else" => Token::Statement(Statement::Else),
                "while" => Token::Statement(Statement::While),
                "print" => Token::Statement(Statement::Print),
                "println" => Token::Statement(Statement::Println),
                "for" => Token::Statement(Statement::For),
                "none" => Token::TypeName(TypeName::None),
                "bool" => Token::TypeName(TypeName::Bool),
                "String" => Token::TypeName(TypeName::QuotedString),
                "i8" => Token::TypeName(TypeName::I8),
                "i16" => Token::TypeName(TypeName::I16),
                "i32" => Token::TypeName(TypeName::I32),

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Token::TypeValue(TypeValue::Identifier(
                    identifier_parser(buffer.clone()).unwrap(),
                )),
            };
            tokens.push(token);
            buffer.clear();
        }

        // Add the symbol to the tokens list
        let token = match c {
            '*' => Token::Operator(Operator::Multiply),
            //'^' => Tokens::Carat,
            ':' => Token::Symbol(Symbol::Colon),
            '.' => Token::Symbol(Symbol::Dot),
            '=' => Token::Assign(Assign::Assign),
            '-' => Token::Operator(Operator::Subtract),
            '(' => Token::Symbol(Symbol::LeftParen),
            '{' => Token::Symbol(Symbol::LeftBrace),
            '<' => Token::Logical(Logical::LessThan),
            '[' => Token::Symbol(Symbol::LeftBracket),
            ')' => Token::Symbol(Symbol::RightParen),
            '}' => Token::Symbol(Symbol::RightBrace),
            '>' => Token::Logical(Logical::GreaterThan),
            ']' => Token::Symbol(Symbol::RightBracket),
            '+' => Token::Operator(Operator::Add),
            '%' => Token::Operator(Operator::Remainder),
            ';' => Token::Symbol(Symbol::Semicolon),
            '/' => Token::Operator(Operator::Divide),
            ',' => Token::Symbol(Symbol::Comma),
            '!' => Token::Logical(Logical::Not),

            ' ' | '\n' | '\t' | '\r' => continue,
            _ => panic!("Unexpected character: {}", c),
        };
        tokens.push(token);

        if !buffer.is_empty() {
            let token = match buffer.as_str() {
                "let" => Token::Statement(Statement::Let),
                "fn" => Token::Statement(Statement::Function),
                //"->" => Token::Symbol(Symbol::Arrow),
                //"::" => Token::Symbol(Symbol::DoubleColon),
                "return" => Token::Statement(Statement::Return),
                "import" => Token::Statement(Statement::Import),
                //"==" => Token::Logical(Logical::Equals),
                //"!=" => Token::Logical(Logical::NotEquals),
                "if" => Token::Statement(Statement::If),
                "else" => Token::Statement(Statement::Else),
                "while" => Token::Statement(Statement::While),
                "print" => Token::Statement(Statement::Print),
                "println" => Token::Statement(Statement::Println),
                "for" => Token::Statement(Statement::For),
                "none" => Token::TypeName(TypeName::None),
                "bool" => Token::TypeName(TypeName::Bool),
                "String" => Token::TypeName(TypeName::QuotedString),
                "i8" => Token::TypeName(TypeName::I8),
                "i16" => Token::TypeName(TypeName::I16),
                "i32" => Token::TypeName(TypeName::I32),
                "i64" => Token::TypeName(TypeName::I64),

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Token::TypeValue(TypeValue::Identifier(
                    identifier_parser(buffer.clone()).unwrap(),
                )),
            };
            tokens.push(token);
        }
    }

    tokens
}

fn identifier_parser(buffer: String) -> Result<String, LexerError> {
    if buffer.chars().next().unwrap().is_numeric() {
        return Err(LexerError::InvalidIdentifierNum(buffer));
    }
    if buffer.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Err(LexerError::InvalidIdentifierChar(buffer));
    }
    Ok(buffer)
}

#[derive(Debug, Clone)]
pub enum LexerError {
    InvalidIdentifierChar(String),
    InvalidIdentifierNum(String),
    InvalidNumber(String),
    InvalidOperator(String),
    InvalidString(String),
    InvalidSymbol(String),
    InvalidToken(String),
    UnexpectedEndOfInput,
    UnknownCharacter(String),
    UnmatchedQuote,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidIdentifierChar(id) =>
                write!(f, "Invalid identifier character in '{}'- identifiers can only contain letters, numbers and underscores", id),
            LexerError::InvalidIdentifierNum(id) =>
                write!(f, "Invalid identifier '{}' - identifiers cannot start with a number", id),
            LexerError::InvalidNumber(n) =>
                write!(f, "Invalid number: {}", n),
            LexerError::InvalidOperator(op) =>
                write!(f, "Invalid operator: {}", op),
            LexerError::InvalidString(s) =>
                write!(f, "Invalid string: {}", s),
            LexerError::InvalidSymbol(s) =>
                write!(f, "Invalid symbol: {}", s),
            LexerError::InvalidToken(t) =>
                write!(f, "Invalid token: {}", t),
            LexerError::UnexpectedEndOfInput =>
                write!(f, "Unexpected end of input"),
            LexerError::UnknownCharacter(c) =>
                write!(f, "Unknown character: {}", c),
            LexerError::UnmatchedQuote =>
                write!(f, "Unmatched quote"),
        }
    }
}

impl std::error::Error for LexerError {}

impl From<std::io::Error> for LexerError {
    fn from(err: std::io::Error) -> Self {
        LexerError::InvalidString(err.to_string())
    }
}

impl From<String> for LexerError {
    fn from(err: String) -> Self {
        LexerError::InvalidToken(err)
    }
}

impl From<&str> for LexerError {
    fn from(err: &str) -> Self {
        LexerError::InvalidToken(err.to_string())
    }
}
