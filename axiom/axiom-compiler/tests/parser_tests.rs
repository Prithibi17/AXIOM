use axiom_compiler::ast::{BinOp, Expr, Statement};
use axiom_compiler::lexer::Lexer;
use axiom_compiler::parser::Parser;

fn parse(input: &str) -> Vec<Statement> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

#[test]
fn test_let_statements() {
    let input = "
        let x = 5
        mut y = 10
    ";
    let program = parse(input);
    assert_eq!(program.len(), 2);

    assert_eq!(
        program[0],
        Statement::Let {
            name: "x".to_string(),
            value: Expr::Number(5.0),
            mutable: false,
        }
    );

    assert_eq!(
        program[1],
        Statement::Let {
            name: "y".to_string(),
            value: Expr::Number(10.0),
            mutable: true,
        }
    );
}

#[test]
fn test_operator_precedence_parsing() {
    let input = "a + b * c == d";
    let program = parse(input);
    
    // Equivalent to (a + (b * c)) == d
    let expected = Statement::Expression(Expr::BinaryOp {
        left: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            op: BinOp::Add,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("b".to_string())),
                op: BinOp::Multiply,
                right: Box::new(Expr::Identifier("c".to_string())),
            }),
        }),
        op: BinOp::Equal,
        right: Box::new(Expr::Identifier("d".to_string())),
    });

    assert_eq!(program[0], expected);
}

#[test]
fn test_grouped_expressions() {
    let input = "(a + b) * c";
    let program = parse(input);
    
    let expected = Statement::Expression(Expr::BinaryOp {
        left: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            op: BinOp::Add,
            right: Box::new(Expr::Identifier("b".to_string())),
        }),
        op: BinOp::Multiply,
        right: Box::new(Expr::Identifier("c".to_string())),
    });

    assert_eq!(program[0], expected);
}
