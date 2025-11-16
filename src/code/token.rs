use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Operator(Operator),
    Logical(Logical),
    Assign(Assign),
    Statement(Statement),
    TypeName(TypeName),
    TypeValue(TypeValue),
    Symbol(Symbol),
    EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    ShiftLeft,
    ShiftRight,

    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Logical {
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Assign {
    // Default
    Assign,

    // Normal Operator Assigns
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,

    // Bitwise Operator Assigns
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    // Public,
    Let,
    Return,
    Print,
    Println,
    Break,
    If,
    Else,
    ElseIf,
    For,
    While,
    Function,
    Import,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeName {
    None,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    QuotedString,
    Bool,
    Float,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeValue {
    None,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    QuotedString(String),
    Bool(bool),
    //Float(f64),
    Identifier(String),
    FunctionCall(String, Vec<Token>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Dot, // .
    Comment, // //
    Arrow, // ->
    Comma, // ,
    Colon, // :
    DoubleColon, // ::
    Semicolon, // ;
    LeftParen, // (
    RightParen, // )
    LeftBrace, // {
    RightBrace, // }
    LeftBracket, // [
    RightBracket, // ]
    LeftAngleBracket, // <
    RightAngleBracket, // >
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::TypeValue(TypeValue::Identifier(_)))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Token::Operator(op) => write!(f, "{}", op),
            Token::Logical(log) => write!(f, "{}", log),
            Token::Assign(asn) => write!(f, "{}", asn),
            Token::Statement(st) => write!(f, "{}", st),
            Token::TypeName(tn) => write!(f, "{}", tn),
            Token::TypeValue(tv) => write!(f, "{}", tv),
            Token::Symbol(sym) => write!(f, "{}", sym),
            Token::EndOfFile => write!(f, "EndOfFile"),
        }
    }
}

impl Operator {
    /// Returns a string representation of the operator.
    pub const fn as_str(&self) -> &str {
        match *self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Remainder => "%",
            Operator::ShiftLeft => "<<",
            Operator::ShiftRight => ">>",
            Operator::BitAnd => "&",
            Operator::BitOr => "|",
            Operator::BitXor => "^",
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl Logical {
    /// Returns a string representation of the operator.
    pub const fn as_str(&self) -> &str {
        match *self {
            Logical::Equals => "==",
            Logical::NotEquals => "!=",
            Logical::LessThan => "<",
            Logical::LessThanEquals => "<=",
            Logical::GreaterThan => ">",
            Logical::GreaterThanEquals => ">=",
            Logical::And => "&&",
            Logical::Or => "||",
            Logical::Not => "!",
        }
    }
}

impl Display for Logical {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl Assign {
    /// Returns a string representation of the operator.
    pub const fn as_str(&self) -> &str {
        match *self {
            Assign::Assign => "=",
            Assign::AddAssign => "+=",
            Assign::SubAssign => "-=",
            Assign::MulAssign => "*=",
            Assign::DivAssign => "/=",
            Assign::RemAssign => "%=",
            Assign::BitAndAssign => "&=",
            Assign::BitOrAssign => "|=",
            Assign::BitXorAssign => "^=",
        }
    }
}

impl Display for Assign {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl Statement {
    /// Returns a string representation of the statement.
    pub const fn as_str(&self) -> &str {
        match *self {
            Statement::Let => "let",
            Statement::Return => "return",
            Statement::Print => "print",
            Statement::Println => "println",
            Statement::Break => "break",
            Statement::If => "if",
            Statement::Else => "else",
            Statement::ElseIf => "else if",
            Statement::For => "for",
            Statement::While => "while",
            Statement::Function => "fn",
            Statement::Import => "import",
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl TypeName {
    /// Returns a string representation of the type.
    pub const fn as_str(&self) -> &str {
        match *self {
            TypeName::None => "none",
            TypeName::I8 => "i8",
            TypeName::I16 => "i16",
            TypeName::I32 => "i32",
            TypeName::I64 => "i64",
            TypeName::U8 => "u8",
            TypeName::U16 => "u16",
            TypeName::U32 => "u32",
            TypeName::U64 => "u64",
            TypeName::QuotedString => "string",
            TypeName::Bool => "bool",
            TypeName::Float => "float",
        }
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for TypeValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            TypeValue::None => write!(f, "None"),
            TypeValue::I8(n) => write!(f, "{}", n),
            TypeValue::I16(n) => write!(f, "{}", n),
            TypeValue::I32(n) => write!(f, "{}", n),
            TypeValue::I64(n) => write!(f, "{}", n),
            TypeValue::U8(n) => write!(f, "{}", n),
            TypeValue::U16(n) => write!(f, "{}", n),
            TypeValue::U32(n) => write!(f, "{}", n),
            TypeValue::U64(n) => write!(f, "{}", n),
            TypeValue::QuotedString(ref s) => write!(f, "{}", s),
            TypeValue::Bool(n) => write!(f, "{}", n),
            //TypeValue::Float(n) => write!(f, "Float({})", n),
            TypeValue::Identifier(ref s) => write!(f, "{}", s),
            TypeValue::FunctionCall(ref s, ref args) => {
                write!(f, "{}(", s)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            } //_ => write!(f, "{}", self.as_str()),
        }
    }
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Symbol::Colon => ":",
            Symbol::Comma => ",",
            Symbol::DoubleColon => "::",
            Symbol::Dot => ".",
            Symbol::LeftParen => "(",
            Symbol::LeftBrace => "{",
            Symbol::LeftBracket => "[",
            Symbol::LeftAngleBracket => "<",
            Symbol::RightParen => ")",
            Symbol::RightBrace => "}",
            Symbol::RightBracket => "]",
            Symbol::RightAngleBracket => ">",
            Symbol::Arrow => "->",
            Symbol::Semicolon => ";",
            Symbol::Comment => "//",
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}
