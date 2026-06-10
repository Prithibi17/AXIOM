use axiom_compiler::ast::{BinOp, Expr, Statement};
use axiom_compiler::type_checker::{Type, TypeChecker, TypeError};

#[test]
fn test_valid_types() {
    let mut checker = TypeChecker::new();

    let stmt = Statement::Let {
        name: "x".to_string(),
        value: Expr::BinaryOp {
            left: Box::new(Expr::Number(5.0)),
            op: BinOp::Add,
            right: Box::new(Expr::Number(10.0)),
        },
        mutable: false,
    };

    assert_eq!(checker.check_stmt(&stmt), Ok(()));
}

#[test]
fn test_type_mismatch() {
    let mut checker = TypeChecker::new();

    let expr = Expr::BinaryOp {
        left: Box::new(Expr::Number(5.0)),
        op: BinOp::Add,
        right: Box::new(Expr::String("10".to_string())),
    };

    let result = checker.check_expr(&expr);
    assert_eq!(
        result,
        Err(TypeError::TypeMismatch {
            expected: Type::Number,
            found: Type::String,
        })
    );
}

#[test]
fn test_undefined_variable() {
    let mut checker = TypeChecker::new();

    let expr = Expr::Identifier("unknown_var".to_string());
    
    let result = checker.check_expr(&expr);
    assert_eq!(
        result,
        Err(TypeError::UndefinedVariable("unknown_var".to_string()))
    );
}
