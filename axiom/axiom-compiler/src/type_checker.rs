use std::collections::HashMap;

use crate::ast::{BinOp, Expr, Statement, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Null,
    Array(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    TypeMismatch { expected: Type, found: Type },
    UndefinedVariable(String),
    UnsupportedOperator { op: String, ty: Type },
    InvalidOperation(String),
}

pub struct TypeChecker {
    symbol_table: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: HashMap::new(),
        }
    }

    pub fn check_program(&mut self, statements: &[Statement]) -> Result<(), TypeError> {
        for stmt in statements {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn check_stmt(&mut self, stmt: &Statement) -> Result<(), TypeError> {
        match stmt {
            Statement::Let { name, value, .. } => {
                let ty = self.check_expr(value)?;
                self.symbol_table.insert(name.clone(), ty);
                Ok(())
            }
            Statement::Expression(expr) => {
                self.check_expr(expr)?;
                Ok(())
            }
            Statement::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    self.check_expr(expr)?;
                }
                Ok(())
            }
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        match expr {
            Expr::Number(_) => Ok(Type::Number),
            Expr::String(_) => Ok(Type::String),
            Expr::Boolean(_) => Ok(Type::Boolean),
            Expr::Null => Ok(Type::Null),
            Expr::Identifier(name) => {
                if let Some(ty) = self.symbol_table.get(name) {
                    Ok(ty.clone())
                } else {
                    Err(TypeError::UndefinedVariable(name.clone()))
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_ty = self.check_expr(left)?;
                let right_ty = self.check_expr(right)?;

                match op {
                    BinOp::Add | BinOp::Subtract | BinOp::Multiply | BinOp::Divide | BinOp::Modulo => {
                        if left_ty == Type::Number && right_ty == Type::Number {
                            Ok(Type::Number)
                        } else if *op == BinOp::Add && left_ty == Type::String && right_ty == Type::String {
                            Ok(Type::String) // String concatenation
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: Type::Number,
                                found: right_ty,
                            })
                        }
                    }
                    BinOp::Equal | BinOp::NotEqual => {
                        if left_ty == right_ty {
                            Ok(Type::Boolean)
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: left_ty,
                                found: right_ty,
                            })
                        }
                    }
                    BinOp::LessThan | BinOp::GreaterThan | BinOp::LessThanOrEqual | BinOp::GreaterThanOrEqual => {
                        if left_ty == Type::Number && right_ty == Type::Number {
                            Ok(Type::Boolean)
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: Type::Number,
                                found: right_ty,
                            })
                        }
                    }
                    BinOp::And | BinOp::Or => {
                        if left_ty == Type::Boolean && right_ty == Type::Boolean {
                            Ok(Type::Boolean)
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: Type::Boolean,
                                found: right_ty,
                            })
                        }
                    }
                }
            }
            Expr::UnaryOp { op, operand } => {
                let operand_ty = self.check_expr(operand)?;
                match op {
                    UnaryOp::Negate => {
                        if operand_ty == Type::Number {
                            Ok(Type::Number)
                        } else {
                            Err(TypeError::UnsupportedOperator {
                                op: "-".to_string(),
                                ty: operand_ty,
                            })
                        }
                    }
                    UnaryOp::Not => {
                        if operand_ty == Type::Boolean {
                            Ok(Type::Boolean)
                        } else {
                            Err(TypeError::UnsupportedOperator {
                                op: "!".to_string(),
                                ty: operand_ty,
                            })
                        }
                    }
                }
            }
            _ => Ok(Type::Unknown), // Simplification for other structures
        }
    }
}
