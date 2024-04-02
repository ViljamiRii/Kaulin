use std::collections::HashMap;
use crate::runtime::values::RuntimeVal;

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeVal>,
}

impl Environment {
    pub fn new(parent_env: Option<Box<Environment>>) -> Self {
        Self {
            parent: parent_env,
            variables: HashMap::new(),
        }
    }

    pub fn declare_var(&mut self, varname: String, value: RuntimeVal) -> RuntimeVal {
        if self.variables.contains_key(&varname) {
            panic!("Cannot declare variable {}. As it already is defined.", varname);
        }

        self.variables.insert(varname.clone(), value.clone());
        value
    }

    pub fn assign_var(&mut self, varname: String, value: RuntimeVal) -> RuntimeVal {
        let env = self.resolve(&varname);
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