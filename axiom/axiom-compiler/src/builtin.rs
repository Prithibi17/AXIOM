//! This module provides hooks for the AXIOM standard library functions
//! that are expected to be injected into the LLVM environment during compilation.

pub struct NativeFunction {
    pub name: &'static str,
    // In a real implementation, this would map to inkwell::types::FunctionType
    // and would be registered directly with the LLVM module.
}

pub fn get_core_builtins() -> Vec<NativeFunction> {
    vec![
        // I/O
        NativeFunction { name: "print" },
        NativeFunction { name: "println" },
        
        // Math
        NativeFunction { name: "sin" },
        NativeFunction { name: "cos" },
        NativeFunction { name: "sqrt" },
        
        // Strings
        NativeFunction { name: "string_length" },
        NativeFunction { name: "string_concat" },
        
        // Arrays
        NativeFunction { name: "array_length" },
        NativeFunction { name: "array_push" },
    ]
}

// In the compiler's initialization phase, we would loop over these 
// native functions and declare them in the LLVM module:
//
// for builtin in get_core_builtins() {
//     let fn_type = ... // define signature
//     module.add_function(builtin.name, fn_type, None);
// }
