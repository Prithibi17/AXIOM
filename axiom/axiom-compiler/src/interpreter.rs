use std::collections::HashMap;

use crate::ast::{BinOp, Expr, Statement};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable: '{}'", name))
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Statement]) -> Result<(), String> {
        for stmt in statements {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn eval_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.environment.define(name.clone(), val);
                Ok(())
            }
            Statement::Expression(expr) => {
                let val = self.eval_expr(expr)?;
                // Simple implicit print for top level expressions in REPL style
                println!("=> {:?}", val);
                Ok(())
            }
            Statement::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    let _val = self.eval_expr(expr)?;
                    // In a full implementation, this would bubble up a Return value
                }
                Ok(())
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Identifier(name) => self.environment.get(name),
            Expr::BinaryOp { left, op, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match (left_val, op, right_val) {
                    (Value::Number(a), BinOp::Add, Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Number(a), BinOp::Subtract, Value::Number(b)) => Ok(Value::Number(a - b)),
                    (Value::Number(a), BinOp::Multiply, Value::Number(b)) => Ok(Value::Number(a * b)),
                    (Value::Number(a), BinOp::Divide, Value::Number(b)) => {
                        if b == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::Number(a / b))
                    }
                    (Value::Number(a), BinOp::Equal, Value::Number(b)) => Ok(Value::Boolean(a == b)),
                    (Value::Number(a), BinOp::LessThan, Value::Number(b)) => Ok(Value::Boolean(a < b)),
                    (Value::Number(a), BinOp::GreaterThan, Value::Number(b)) => Ok(Value::Boolean(a > b)),
                    (Value::String(a), BinOp::Add, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    _ => Err(format!("Unsupported binary operation")),
                }
            }
            // Add if/else evaluation and function calls here as it expands
            _ => Err("Expression evaluation not fully implemented".to_string()),
        }
    }
}
