use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let, Mut, Fn, If, Else, For, While, Break, Continue, Match,
    Try, Catch, Finally, Throw, Import, From, Export, Return,
    True, False, Null, Type, Interface, Extends, Implements,
    As, In, Is, And, Or, Not,

    // Literals
    Number(f64),
    String(String),
    Identifier(String),

    // Operators & Punctuation
    Plus, Minus, Star, Slash, Percent, Power, // +, -, *, /, %, **
    Equal, EqualEqual, NotEqual, Less, Greater, LessEqual, GreaterEqual, // =, ==, !=, <, >, <=, >=
    PlusEqual, MinusEqual, StarEqual, SlashEqual, PercentEqual, // +=, -=, *=, /=, %=
    AmpersandAmpersand, PipePipe, Bang, // &&, ||, !
    Ampersand, Pipe, Caret, Tilde, ShiftLeft, ShiftRight, // &, |, ^, ~, <<, >>
    
    LeftBrace, RightBrace, // {, }
    LeftParen, RightParen, // (, )
    LeftBracket, RightBracket, // [, ]
    Colon, Semicolon, Comma, Dot, Arrow, // :, ;, ,, ., ->

    // Special
    Eof,
    Illegal(char),
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        // Assume we already matched one '-'
        if let Some(&c) = self.peek() {
            if c == '-' {
                // Single line comment --
                self.advance();
                while let Some(&ch) = self.peek() {
                    if ch == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if c == '#' {
                // Multi-line comment -# ... #-
                self.advance();
                loop {
                    match self.advance() {
                        Some('#') => {
                            if let Some('-') = self.advance() {
                                break;
                            }
                        }
                        Some(_) => continue,
                        None => break, // Unclosed comment
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    fn lookup_ident(ident: &str) -> Token {
        match ident {
            "let" => Token::Let,
            "mut" => Token::Mut,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "match" => Token::Match,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "finally" => Token::Finally,
            "throw" => Token::Throw,
            "import" => Token::Import,
            "from" => Token::From,
            "export" => Token::Export,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "type" => Token::Type,
            "interface" => Token::Interface,
            "extends" => Token::Extends,
            "implements" => Token::Implements,
            "as" => Token::As,
            "in" => Token::In,
            "is" => Token::Is,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Identifier(ident.to_string()),
        }
    }

    fn read_number(&mut self, first: char) -> f64 {
        let mut number_str = String::new();
        number_str.push(first);
        
        let mut has_dot = first == '.';
        
        while let Some(&c) = self.peek() {
            if c.is_digit(10) {
                number_str.push(self.advance().unwrap());
            } else if c == '.' && !has_dot {
                has_dot = true;
                number_str.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        
        // Very basic parsing, error handling could be better
        number_str.parse::<f64>().unwrap_or(0.0)
    }

    fn read_string(&mut self, quote: char) -> String {
        let mut s = String::new();
        while let Some(&c) = self.peek() {
            if c == quote {
                self.advance(); // consume closing quote
                break;
            } else if c == '\\' {
                self.advance(); // consume backslash
                if let Some(esc) = self.advance() {
                    match esc {
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        '\\' => s.push('\\'),
                        '"' => s.push('"'),
                        '\'' => s.push('\''),
                        _ => s.push(esc),
                    }
                }
            } else {
                s.push(self.advance().unwrap());
            }
        }
        s
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = match self.advance() {
            Some(c) => c,
            None => return Token::Eof,
        };

        match ch {
            '-' => {
                match self.peek() {
                    Some(&'-') | Some(&'#') => {
                        self.skip_comments();
                        self.next_token() // recursive call after comment
                    }
                    Some(&'>') => {
                        self.advance();
                        Token::Arrow
                    }
                    Some(&'=') => {
                        self.advance();
                        Token::MinusEqual
                    }
                    _ => Token::Minus,
                }
            }
            '+' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::PlusEqual
                } else {
                    Token::Plus
                }
            }
            '*' => {
                if let Some(&'*') = self.peek() {
                    self.advance();
                    Token::Power
                } else if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::StarEqual
                } else {
                    Token::Star
                }
            }
            '/' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::SlashEqual
                } else {
                    Token::Slash
                }
            }
            '%' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::PercentEqual
                } else {
                    Token::Percent
                }
            }
            '=' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::LessEqual
                } else if let Some(&'<') = self.peek() {
                    self.advance();
                    Token::ShiftLeft
                } else {
                    Token::Less
                }
            }
            '>' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    Token::GreaterEqual
                } else if let Some(&'>') = self.peek() {
                    self.advance();
                    Token::ShiftRight
                } else {
                    Token::Greater
                }
            }
            '&' => {
                if let Some(&'&') = self.peek() {
                    self.advance();
                    Token::AmpersandAmpersand
                } else {
                    Token::Ampersand
                }
            }
            '|' => {
                if let Some(&'|') = self.peek() {
                    self.advance();
                    Token::PipePipe
                } else {
                    Token::Pipe
                }
            }
            '^' => Token::Caret,
            '~' => Token::Tilde,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '"' | '\'' => Token::String(self.read_string(ch)),
            c if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier(c);
                Self::lookup_ident(&ident)
            }
            c if c.is_digit(10) => Token::Number(self.read_number(c)),
            _ => Token::Illegal(ch),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::Eof {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        tokens
    }
}
