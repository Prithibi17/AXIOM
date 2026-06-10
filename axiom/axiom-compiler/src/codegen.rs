use crate::ast::{BinOp, Expr, Statement};

// NOTE: Since the LLVM C++ libraries (and thus `inkwell`) are required 
// to be installed on the host machine to compile this code, we provide 
// a conceptual implementation mirroring how the LLVM IR generation 
// operates structurally.
//
// To actually compile this project with inkwell, you would add `inkwell = "0.2.0"`
// to Cargo.toml and replace these mock types with the real inkwell imports:
// use inkwell::context::Context;
// use inkwell::module::Module;
// use inkwell::builder::Builder;
// use inkwell::values::{BasicValueEnum, FloatValue};

pub struct MockContext;
pub struct MockModule;
pub struct MockBuilder;
pub struct MockBasicValueEnum;
pub struct MockFloatValue;

pub struct CodeGenerator {
    context: MockContext,
    module: MockModule,
    builder: MockBuilder,
    // variables: HashMap<String, inkwell::values::PointerValue<'ctx>>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            context: MockContext,
            module: MockModule,
            builder: MockBuilder,
        }
    }

    pub fn compile(&mut self, statements: &[Statement]) -> Result<(), String> {
        for stmt in statements {
            self.codegen_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn codegen_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let { name, value, mutable } => {
                let _val = self.codegen_expr(value)?;
                // In LLVM:
                // 1. Create alloca instruction for variable in the entry block
                // 2. Store `val` into the alloca
                // 3. Save the alloca pointer in self.variables hashmap
                Ok(())
            }
            Statement::Expression(expr) => {
                self.codegen_expr(expr)?;
                Ok(())
            }
            Statement::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    let _val = self.codegen_expr(expr)?;
                    // In LLVM: builder.build_return(Some(&val));
                } else {
                    // In LLVM: builder.build_return(None);
                }
                Ok(())
            }
        }
    }

    pub fn codegen_expr(&mut self, expr: &Expr) -> Result<MockBasicValueEnum, String> {
        match expr {
            Expr::Number(_n) => {
                // In LLVM:
                // Ok(self.context.f64_type().const_float(*n).into())
                Ok(MockBasicValueEnum)
            }
            Expr::String(_s) => {
                // In LLVM: Generate global string ptr
                Ok(MockBasicValueEnum)
            }
            Expr::Identifier(_name) => {
                // In LLVM:
                // let ptr = self.variables.get(name).ok_or("Unknown var")?;
                // Ok(self.builder.build_load(*ptr, name).into())
                Ok(MockBasicValueEnum)
            }
            Expr::BinaryOp { left, op, right } => {
                let _lhs = self.codegen_expr(left)?;
                let _rhs = self.codegen_expr(right)?;

                match op {
                    BinOp::Add => {
                        // In LLVM: builder.build_float_add(lhs, rhs, "addtmp")
                    }
                    BinOp::Subtract => {
                        // In LLVM: builder.build_float_sub(lhs, rhs, "subtmp")
                    }
                    BinOp::Multiply => {
                        // In LLVM: builder.build_float_mul(lhs, rhs, "multmp")
                    }
                    // ... other operators
                    _ => {}
                }

                Ok(MockBasicValueEnum)
            }
            _ => Err("Unsupported expression for codegen".to_string()),
        }
    }
}
