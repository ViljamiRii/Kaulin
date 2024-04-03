use std::collections::{HashMap, HashSet};
use crate::runtime::values::*;

pub fn create_global_env() -> Environment {
    let mut env = Environment::new(None);
    //env.declare_var("true".to_string(), MK_BOOL(true), true);
    env.declare_var("tosi".to_string(), MK_BOOL(true), true);
    //env.declare_var("false".to_string(), MK_BOOL(false), true);
    env.declare_var("epätosi".to_string(), MK_BOOL(false), true);
    //env.declare_var("null".to_string(), MK_NULL(), true);
    env.declare_var("tyhjä".to_string(), MK_NULL(), true);
    env
}

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeVal>,
    constants: HashSet<String>,
}

impl Environment {
    pub fn new(parent_env: Option<Box<Environment>>) -> Self {
        Self {
            parent: parent_env,
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }

    pub fn declare_var(&mut self, varname: String, value: RuntimeVal, constant: bool) -> RuntimeVal {
        if self.variables.contains_key(&varname) {
            panic!("Cannot declare variable {}. As it already is defined.", varname);
        }

        self.variables.insert(varname.clone(), value.clone());
        if constant {
            self.constants.insert(varname.clone());
        }
        value
    }

    pub fn assign_var(&mut self, varname: String, value: RuntimeVal) -> RuntimeVal {
        let env = self.resolve(&varname);

        // Cannot assign to constant
        if env.constants.contains(&varname) {
            panic!("Cannot reassign to variable {} as it was declared constant.", varname);
        }

        env.variables.insert(varname, value.clone());
        value
    }
    
    pub fn lookup_var(&mut self, varname: &str) -> RuntimeVal {
        let env = self.resolve(varname);
        env.variables.get(varname).unwrap().clone()
    }
    
    pub fn resolve(&mut self, varname: &str) -> &mut Self {
        if self.variables.contains_key(varname) {
            return self;
        }
    
        match &mut self.parent {
            Some(parent) => parent.resolve(varname),
            None => panic!("Cannot resolve '{}' as it does not exist.", varname),
        }
    }
}