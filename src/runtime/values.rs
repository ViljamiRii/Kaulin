#[derive(Clone, Debug)]
pub enum ValueType {
    Null,
    Number,
    Boolean,
}

#[derive(Clone, Debug)]
pub enum RuntimeVal {
    Null(NullVal),
    Bool(BoolVal),
    Number(NumberVal),
}

#[derive(Clone, Debug)]
pub struct NullVal {
    pub value_type: ValueType,
    pub value: Option<()>,
}

#[derive(Clone, Debug)]
pub struct BoolVal {
    pub value_type: ValueType,
    pub value: bool,
}

#[derive(Clone, Debug)]
pub struct NumberVal {
    pub value_type: ValueType,
    pub value: f64,
}

impl NullVal {
    pub fn mk_null() -> Self {
        Self {
            value_type: ValueType::Null,
            value: None,
        }
    }
}

impl BoolVal {
    pub fn mk_bool(b: bool) -> Self {
        Self {
            value_type: ValueType::Boolean,
            value: b,
        }
    }
}

impl NumberVal {
    pub fn mk_number(n: f64) -> Self {
        Self {
            value_type: ValueType::Number,
            value: n,
        }
    }
}

pub fn MK_BOOL(value: bool) -> RuntimeVal {
    RuntimeVal::Bool(BoolVal::mk_bool(value))
}

pub fn MK_NULL() -> RuntimeVal {
    RuntimeVal::Null(NullVal::mk_null())
}

pub fn MK_NUMBER(value: f64) -> RuntimeVal {
    RuntimeVal::Number(NumberVal::mk_number(value))
}