use std::rc::Rc;
use std::cell::RefCell;
use crate::frontend::ast::*;
use crate::runtime::environment::*;

#[derive(Clone, Debug)]
pub enum RuntimeVal {
    Null,
    Bool(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Object(Vec<(String, RuntimeVal)>),
    Array(Vec<RuntimeVal>),
    NativeFunction(NativeFunction),
    Function(Function),
}

impl RuntimeVal {
    pub fn is_truthy(&self) -> bool {
        match self {
            RuntimeVal::Null => false,
            RuntimeVal::Bool(b) => *b,
            RuntimeVal::Integer(i) => *i != 0,
            RuntimeVal::Number(n) => *n != 0.0,
            RuntimeVal::String(s) => !s.is_empty(),
            RuntimeVal::Object(_) => true,
            RuntimeVal::Array(a) => !a.is_empty(),
            RuntimeVal::NativeFunction(_) => true,
            RuntimeVal::Function(_) => true,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            RuntimeVal::Bool(b) => Some(*b),
            _ => None,
        }
    }
}


pub struct NativeFunction(Rc<dyn Fn(Vec<RuntimeVal>, Vec<(String, RuntimeVal)>) -> RuntimeVal>);

impl NativeFunction {
    pub fn get_fn(&self) -> Rc<dyn Fn(Vec<RuntimeVal>, Vec<(String, RuntimeVal)>) -> RuntimeVal> {
        Rc::clone(&self.0)
    }
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NativeFunction")
    }
}

impl Clone for NativeFunction {
    fn clone(&self) -> Self {
        NativeFunction(Rc::clone(&self.0))
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub parameters: Vec<String>,
    pub declaration_env: Rc<RefCell<Environment>>,
    pub body: Vec<Stmt>,
}

pub fn MK_BOOL(value: bool) -> RuntimeVal {
    RuntimeVal::Bool(value)
}

pub fn MK_NULL() -> RuntimeVal {
    RuntimeVal::Null
}

pub fn MK_INTEGER(value: i64) -> RuntimeVal {
    RuntimeVal::Integer(value)
}

pub fn MK_NUMBER(value: f64) -> RuntimeVal {
    RuntimeVal::Number(value)
}

pub fn MK_OBJECT(properties: Vec<(String, RuntimeVal)>) -> RuntimeVal {
    RuntimeVal::Object(properties)
}

pub fn MK_NATIVE_FN(
    call: Rc<dyn Fn(Vec<RuntimeVal>, Vec<(String, RuntimeVal)>) -> RuntimeVal>
) -> RuntimeVal {
    RuntimeVal::NativeFunction(NativeFunction(call))
}

pub fn MK_STRING(value: String) -> RuntimeVal {
    RuntimeVal::String(value)
}

pub fn MK_ARRAY(elements: Vec<RuntimeVal>) -> RuntimeVal {
    RuntimeVal::Array(elements)
}