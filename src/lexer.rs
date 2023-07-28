use crate::token::{
    Token, BANG, COMMA, EOF, EQ, GT, ILLEGAL, LBRACE, LPAREN, LT, MINUS, PLUS, RBRACE, RPAREN,
    SEMICOLON, SLASH, ASSIGN, NOT_EQ, ASTERISK,
};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            let input_bytes: Vec<u8> = self.input.bytes().collect();
            self.ch = input_bytes[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        let input_char: Vec<char> = self.input.chars().collect();
        input_char[position..self.position]
            .iter()
            .collect::<String>()
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char()
        }
        let input_char: Vec<char> = self.input.chars().collect();
        input_char[position..self.position]
            .iter()
            .collect::<String>()
    }

    pub fn skip_white_spaces(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }

    pub fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }

        let input_bytes: Vec<u8> = self.input.bytes().collect();
        return input_bytes[self.read_position];
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_spaces();

        let literal = self.ch.to_string();
        let token: Token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(EQ.to_owned(), String::from("=="))
                } else {
                    Token::new(ASSIGN.to_owned(), literal)
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(NOT_EQ.to_owned(), String::from("!="))
                } else {
                    Token::new(BANG.to_owned(), literal)
                }
            }
            b';' => Token::new(SEMICOLON.to_owned(), literal),
            b'(' => Token::new(LPAREN.to_owned(), literal),
            b')' => Token::new(RPAREN.to_owned(), literal),
            b'{' => Token::new(LBRACE.to_owned(), literal),
            b'}' => Token::new(RBRACE.to_owned(), literal),
            b'+' => Token::new(PLUS.to_owned(), literal),
            b'*' => Token::new(ASTERISK.to_owned(), literal),
            b',' => Token::new(COMMA.to_owned(), literal),
            b'-' => Token::new(MINUS.to_owned(), literal),
            b'/' => Token::new(SLASH.to_owned(), literal),
            b'<' => Token::new(LT.to_owned(), literal),
            b'>' => Token::new(GT.to_owned(), literal),
            0 => Token::new(EOF.to_owned(), "".to_string()),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::get_keyword(literal.as_str());
                    return Token::new(token_type.to_owned(), literal);
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    let token_type = Token::get_keyword(literal.as_str());
                    return Token::new(token_type.to_owned(), literal);
                } else {
                    Token::new(ILLEGAL.to_owned(), self.ch.to_string())
                }
            }
        };

        self.read_char();
        return token;
    }
}

fn is_letter(ch: u8) -> bool {
    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
}

fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

#[test]
fn test() {
    let input = "fn(x, y) {}; let five = 5;";
    let mut lexer = Lexer::new(input.to_string());
    let mut counter = 0;

    while counter <= input.len() {
        let l = lexer.next_token();
        println!("{:?}", l);
        counter += 1;
    }
}
