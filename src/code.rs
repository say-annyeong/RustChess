pub struct Code {
    tokens: Vec<Token>
}

impl Code {
    pub fn new(code: String) -> Code {
        let tokens = Self::laxer(code);
        Code { tokens }
    }

    fn laxer(code: String) -> Vec<Token> {
        todo!()
    }
}

pub enum Token {
    ILLEGAL(char),
    EOF,
    IDENT,
    INTEGER,
    ASSIGN, // =
    COMMA, // ,
    COLON, // :
    SEMICOLON, // ;
    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }
    FUNCTION,
    LET
}

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    char: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Self { input, position: 0, read_position: 0, char: '\0' };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.char = '\x00';
        } else {
            let Some(char) = self.input.chars().nth(self.position) else {
                panic!("{}번째 문자가 없음!", self.position);
            };
            self.char = char;
        }
        self.read_position += 1;
        println!("Next:: char: {}, cur_position: {}, peek_position: {}", self.char, self.position, self.read_position);
        self.position = self.read_position;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\x00'
        } else {
            eprintln!("Peek:: char: {}, cur_position: {}, peek_position: {}", self.char, self.position, self.read_position);
            self.input.chars().nth(self.position).unwrap()
        }
    }

    fn next_token(&mut self) -> Token {
        let token = match self.char {
            '=' => Token::ASSIGN,
            ',' => Token::COMMA,
            ':' => Token::COLON,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\x00' => Token::EOF,
            char if char.is_ascii() && !char.is_ascii_whitespace() => {
                
            }
            illegal => Token::ILLEGAL(illegal),
        }
    }
}