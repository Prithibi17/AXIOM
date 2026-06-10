use axiom_compiler::lexer::{Lexer, Token};

fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = lexer.tokenize();
    // Remove EOF for easier testing
    if let Some(Token::Eof) = tokens.last() {
        tokens.pop();
    }
    tokens
}

#[test]
fn test_single_character_tokens() {
    let input = "+ - * / % { } ( ) [ ] : ; , .";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::Percent,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Colon,
            Token::Semicolon,
            Token::Comma,
            Token::Dot,
        ]
    );
}

#[test]
fn test_multi_character_operators() {
    let input = "== != < > <= >= && || ! += -= *= /= %= -> ** << >>";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::EqualEqual,
            Token::NotEqual,
            Token::Less,
            Token::Greater,
            Token::LessEqual,
            Token::GreaterEqual,
            Token::AmpersandAmpersand,
            Token::PipePipe,
            Token::Bang,
            Token::PlusEqual,
            Token::MinusEqual,
            Token::StarEqual,
            Token::SlashEqual,
            Token::PercentEqual,
            Token::Arrow,
            Token::Power,
            Token::ShiftLeft,
            Token::ShiftRight,
        ]
    );
}

#[test]
fn test_keywords_and_identifiers() {
    let input = "let mut fn if else for while custom_var123";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Mut,
            Token::Fn,
            Token::If,
            Token::Else,
            Token::For,
            Token::While,
            Token::Identifier("custom_var123".to_string()),
        ]
    );
}

#[test]
fn test_numbers() {
    let input = "123 3.14";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![Token::Number(123.0), Token::Number(3.14),]
    );
}

#[test]
fn test_strings() {
    let input = "\"hello world\" 'single quote'";
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::String("hello world".to_string()),
            Token::String("single quote".to_string()),
        ]
    );
}

#[test]
fn test_comments() {
    let input = r#"
        let x = 10 -- this is a comment
        -#
            multi-line
            comment
        #-
        let y = 20
    "#;
    let tokens = tokenize(input);
    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number(10.0),
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Equal,
            Token::Number(20.0),
        ]
    );
}
