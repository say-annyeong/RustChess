use std::{
    iter::{Cloned, Peekable},
    slice::Iter,
};
use std::collections::HashMap;
use crate::code::token::*;

pub struct Parser<'a> {
    tokens: Peekable<Cloned<Iter<'a, Token>>>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().cloned().peekable(),
        }
    }
    
    fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), AbstractSyntaxTreeParseError> {
        if self.tokens.peek() == Some(&expected) {
            Ok(())
        } else {
            Err(AbstractSyntaxTreeParseError::ExpectedOther {
                token: expected.to_string(),
            })
        }
    }
    
    pub fn parse_statements(&mut self, _return_type: Token) -> Result<Vec<AbstractSyntaxTree>, AbstractSyntaxTreeParseError> {
        let mut statements = Vec::new();
        //println!("{:?}", self.tokens.peek());
        while let Some(token) = self.tokens.peek() {
            //println!("{:?}", token);
            match token {
                Token::Statement(Statement::Let) => {
                    let let_statement = self.let_parser()?;
                    statements.push(let_statement);
                }
                Token::Statement(Statement::For) => {
                    let for_statement = self.for_parser()?;
                    statements.push(for_statement);
                }
                Token::Statement(Statement::If) => {
                    let if_statement = self.if_parser()?;
                    statements.push(if_statement);
                }
                Token::TypeValue(TypeValue::Identifier(_)) => {
                    //println!("PEEK 2 {:?}", self.tokens.peek());
                    let assign_statement = self.identifier_parser()?;
                    statements.push(assign_statement);
                }
                Token::Statement(Statement::Return) => {
                    let return_statement = self.return_parser()?;
                    statements.push(return_statement);
                }
                _ => break,
            }
        }
        Ok(statements)
    }
    
    fn let_parser(&mut self) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.consume(); // Consume Let

        self.expect(Token::Symbol(Symbol::Colon))?;
        self.consume(); // Consume Colon
        let type_name = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Type".to_owned(),
            })?;
        self.consume(); // Consume Type

        let variable_name = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Variable Name".to_owned(),
            })?;
        if !variable_name.is_identifier() {
            return Err(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Variable Name".to_owned(),
            });
        }
        self.consume(); // Consume Variable Name
        //
        self.expect(Token::Assign(Assign::Assign))?;
        self.consume(); // Consume Tokens::Assign

        //check if the value is a Identifier
        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value
        // check if the value is calling a function
        if self.tokens.peek() == Some(&Token::Symbol(Symbol::LeftParen)) {
            self.consume(); // Consume LeftParen
            let mut args: Vec<Token> = Vec::new();
            let mut is_after_comma = false;
            let mut is_after_open_paren = true;
            while let Some(token) = self.tokens.peek() {
                if token == &Token::Symbol(Symbol::RightParen) {
                    if is_after_comma {
                        return Err(AbstractSyntaxTreeParseError::ExpectedOther {
                            token: "Argument".to_owned(),
                        });
                    }
                    break;
                } else if token == &Token::Symbol(Symbol::Comma) {
                    self.consume(); // Consume Comma
                    is_after_comma = true;
                } else if is_after_comma || is_after_open_paren {
                    args.push(token.clone());
                    self.consume(); // Consume Argument
                    is_after_comma = false;
                    is_after_open_paren = false;
                } else {
                    return Err(AbstractSyntaxTreeParseError::ExpectedOther {
                        token: "Argument".to_owned(),
                    });
                }
            }
            self.expect(Token::Symbol(Symbol::RightParen))?;
            self.consume(); // Consume RightParen
            let let_statement = AbstractSyntaxTree::Let {
                name: variable_name.to_string(),
                type_name: Some(type_name.to_string()),
                value: Token::TypeValue(TypeValue::FunctionCall(value.to_string(), args)),
            };
            self.expect(Token::Symbol(Symbol::Semicolon))?;
            self.consume(); // Consume Tokens::Semicolon
            return Ok(let_statement);
        }

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Tokens::Semicolon
        //
        let let_statement = AbstractSyntaxTree::Let {
            name: variable_name.to_string(),
            type_name: Some(type_name.to_string()),
            value,
        };
        Ok(let_statement)
    }
    
    fn identifier_parser(&mut self) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        let variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Variable Name".to_owned(),
            })?;
        self.consume(); // Consume Identifier

        match self.tokens.peek() {
            Some(Token::Assign(Assign::Assign)) => {
                let assign_statement = self.assign_parser(variable)?;
                Ok(assign_statement)
            }

            /*Some(Token::Symbol(Symbol::LeftParen)) => {
                let function_call = self.function_call_parser()?;
                Ok(function_call)
            }*/
            Some(Token::Assign(Assign::AddAssign)) => {
                let add_assign_statement = self.add_assign_parser(variable)?;
                Ok(add_assign_statement)
            }
            Some(Token::Assign(Assign::SubAssign)) => {
                let sub_assign_statement = self.sub_assign_parser(variable)?;
                Ok(sub_assign_statement)
            }
            Some(Token::Assign(Assign::MulAssign)) => {
                let mul_assign_statement = self.mul_assign_parser(variable)?;
                Ok(mul_assign_statement)
            }
            Some(Token::Assign(Assign::DivAssign)) => {
                let div_assign_statement = self.div_assign_parser(variable)?;
                Ok(div_assign_statement)
            }
            Some(Token::Assign(Assign::RemAssign)) => {
                let rem_assign_statement = self.rem_assign_parser(variable)?;
                Ok(rem_assign_statement)
            }
            Some(Token::Symbol(Symbol::LeftParen)) => {
                let function_call = self.function_call_parser(variable)?;
                Ok(function_call)
            }
            _ => Err(AbstractSyntaxTreeParseError::Unknown),
        }
        //Ok()
    }
    
    fn assign_parser(&mut self, variable: Token) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.expect(Token::Assign(Assign::Assign))?;
        self.consume(); // Consume AddAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let assign_statement = AbstractSyntaxTree::AddAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(assign_statement)
    }
    
    fn add_assign_parser(&mut self, variable: Token) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.expect(Token::Assign(Assign::AddAssign))?;
        self.consume(); // Consume AddAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let add_assign_statement = AbstractSyntaxTree::AddAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(add_assign_statement)
    }
    
    fn sub_assign_parser(&mut self, variable: Token) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.expect(Token::Assign(Assign::SubAssign))?;
        self.consume(); // Consume SubAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let sub_assign_statement = AbstractSyntaxTree::SubAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(sub_assign_statement)
    }
    
    fn mul_assign_parser(&mut self, variable: Token) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.expect(Token::Assign(Assign::MulAssign))?;
        self.consume(); // Consume MulAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let mul_assign_statement = AbstractSyntaxTree::MulAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(mul_assign_statement)
    }
    
    fn div_assign_parser(&mut self, variable: Token) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.expect(Token::Assign(Assign::DivAssign))?;
        self.consume(); // Consume DivAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let div_assign_statement = AbstractSyntaxTree::DivAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(div_assign_statement)
    }
    
    fn rem_assign_parser(&mut self, variable: Token) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.expect(Token::Assign(Assign::RemAssign))?;
        self.consume(); // Consume RemAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let rem_assign_statement = AbstractSyntaxTree::RemAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(rem_assign_statement)
    }
    
    fn function_call_parser(&mut self, name: Token) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.expect(Token::Symbol(Symbol::LeftParen))?;
        self.consume(); // Consume Tokens::LeftParen

        let mut done_flag = false;
        // Parse Arguments
        let mut arguments = Vec::new();
        //println!("Parsing Arguments...");
        while !done_flag {
            match self.tokens.peek() {
                Some(Token::Symbol(Symbol::RightParen)) => {
                    self.consume(); // Consume RightParen
                    done_flag = true;
                }
                Some(Token::Symbol(Symbol::Comma)) => {
                    self.consume(); // Consume Comma
                    // Check if there is another argument
                    if self.tokens.peek() == Some(&Token::Symbol(Symbol::RightParen)) {
                        return Err(AbstractSyntaxTreeParseError::ExpectedOther {
                            token: "Argument".to_owned(),
                        });
                    }
                }
                _ => {
                    let arg_name =
                        self.tokens
                            .peek()
                            .cloned()
                            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                                token: "Argument".to_owned(),
                            })?;
                    self.consume(); // Consume Argument Name
                    //println!("Argument: {:?}", arg_name);
                    arguments.push(arg_name);
                }
            }
        }

        self.expect(Token::Symbol(Symbol::RightParen))?;
        self.consume(); // Consume RightParen

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let function_call_statement = AbstractSyntaxTree::FunctionCall {
            name,
            args: arguments,
        };
        Ok(function_call_statement)
    }
    
    fn if_parser(&mut self) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.consume(); // Consume Tokens::If

        self.expect(Token::Symbol(Symbol::LeftParen))?;
        self.consume(); // Consume Tokens::LeftParen

        let l_value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        //self.expect(Token::Logical(_))?;
        let logic = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Logic".to_owned(),
            })?;
        self.consume(); // Consume Logic

        let r_value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::RightParen))?;
        self.consume(); // Consume RightParen

        self.expect(Token::Symbol(Symbol::LeftBrace))?;
        self.consume(); // Consume LeftBrace

        let statements = self.parse_statements(Token::TypeName(TypeName::None))?;
        self.expect(Token::Symbol(Symbol::RightBrace))?;
        self.consume(); // Consume RightBrace

        let if_statement = AbstractSyntaxTree::If {
            l_var: l_value,
            logic,
            r_var: r_value,
            statements,
        };
        Ok(if_statement)
    }
    
    fn for_parser(&mut self) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.consume(); // Consume Tokens::For

        self.expect(Token::Symbol(Symbol::LeftParen))?;
        self.consume(); // Consume Tokens::LeftParen

        let start_variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Variable".to_owned(),
            })?;
        self.consume(); // Consume Variable

        self.expect(Token::Symbol(Symbol::Arrow))?;
        self.consume(); // Consume Tokens::Arrow

        let end_variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Variable".to_owned(),
            })?;
        self.consume(); // Consume Variable

        self.expect(Token::Symbol(Symbol::DoubleColon))?;
        self.consume(); // Consume Tokens::DoubleColon

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::RightParen))?;
        self.consume(); // Consume Tokens::RightParen

        self.expect(Token::Symbol(Symbol::LeftBrace))?;
        self.consume(); // Consume Tokens::LeftBrace

        let statements = self.parse_statements(Token::TypeName(TypeName::None))?;

        self.expect(Token::Symbol(Symbol::RightBrace))?;
        self.consume(); // Consume Tokens::RightBrace
        let for_statement = AbstractSyntaxTree::For {
            start: start_variable,
            end: end_variable,
            value,
            statements,
        };
        Ok(for_statement)
    }

    fn return_parser(&mut self) -> Result<AbstractSyntaxTree, AbstractSyntaxTreeParseError> {
        self.consume(); // Consume Tokens::Return
        if self.tokens.peek() == Some(&Token::Symbol(Symbol::Semicolon)) {
            self.consume(); // Consume Semicolon
            return Ok(AbstractSyntaxTree::Return {
                value: Token::TypeValue(TypeValue::None),
            });
        }
        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AbstractSyntaxTreeParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let return_statement = AbstractSyntaxTree::Return { value };
        Ok(return_statement)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractSyntaxTree {
    // A leaf node representing a single token
    Token(Token),

    Import {
        //path: String,
        name: String,
    },
    // A node representing a statement
    // A node representing a function definition
    Function {
        name: String,
        args: Vec<(Token, Token)>,
        statements: Vec<Self>,
        variables: HashMap<String, Token>,
        return_type: Token,
        return_value: Token,
    },

    Let {
        name: String,
        type_name: Option<String>,
        value: Token,
    },
    Assign {
        l_var: Token,
        r_var: Token,
    },
    AddAssign {
        l_var: Token,
        r_var: Token,
    },
    SubAssign {
        l_var: Token,
        r_var: Token,
    },
    MulAssign {
        l_var: Token,
        r_var: Token,
    },
    DivAssign {
        l_var: Token,
        r_var: Token,
    },
    RemAssign {
        l_var: Token,
        r_var: Token,
    },
    If {
        l_var: Token,
        logic: Token,
        r_var: Token,
        statements: Vec<Self>,
    },
    ElseIf {
        condition: Vec<Token>,
        statements: Vec<Self>,
    },
    Else {
        statements: Vec<Self>,
    },

    For {
        start: Token,
        end: Token,
        value: Token,
        statements: Vec<Self>,
    },

    FunctionCall {
        name: Token,
        args: Vec<Token>,
    },
    Return {
        value: Token,
    },
}

impl AbstractSyntaxTree {
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function { .. })
    }
    
    pub fn function_get_statements(&self) -> Vec<Self> {
        match self {
            Self::Function { statements, .. } => statements.clone(),
            _ => panic!("Not a function"),
        }
    }
    
    pub fn function_get_args_format(&self) -> Vec<(Token, Token)> {
        match self {
            Self::Function { args, .. } => args.clone(),
            _ => panic!("Not a function"),
        }
    }
    
    pub fn function_get_name(&self) -> String {
        match self {
            Self::Function { name, .. } => name.clone(),
            _ => panic!("Not a function"),
        }
    }
    
    pub fn function_set_return_value(&mut self, value: Token) {
        match self {
            Self::Function {
                name: _,
                args: _,
                statements: _,
                variables: _,
                return_type: _,
                return_value,
            } => {
                *return_value = value;
            }
            _ => panic!("Invalid Return Value"),
        }
    }
    
    pub fn function_get_return_value(&self) -> Token {
        match self {
            Self::Function {
                name: _,
                args: _,
                statements: _,
                variables: _,
                return_type: _,
                return_value,
            } => return_value.clone(),
            _ => panic!("Invalid Return Value"),
        }
    }
    
    pub fn function_insert_variable(&mut self, var_name: String, value: Token) {
        match self {
            Self::Function {
                name: _,
                args: _,
                statements: _,
                variables,
                return_type: _,
                return_value: _,
            } => {
                variables.insert(var_name, value);
                //.nsert(var_name, value);
            }
            _ => panic!("Invalid Variable Insertion"),
        }
    }
    
    pub fn function_get_variable(&self, var_name: String) -> Token {
        match self {
            Self::Function {
                name: _,
                args: _,
                statements: _,
                variables,
                return_type: _,
                return_value: _,
            } => {
                if variables.contains_key(&var_name) {
                    variables.get(&var_name).unwrap().clone()
                } else {
                    panic!("Variable not found");
                }
            }
            _ => panic!("Invalid Variable Reference"),
        }
    }
}

#[derive(Debug, Clone)]
enum AbstractSyntaxTreeParseError {
    UnknownToken { token: String },
    ExpectedOther { token: String },
    EndOfFile,
    Unknown,
}