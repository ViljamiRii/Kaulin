use crate::runtime::values::*;
use crate::runtime::native_functions::*;
use std::rc::Rc;

pub fn create_global_env() -> Environment {
    let mut env = Environment::new(None);
    env.declare_var("tosi".to_string(), MK_BOOL(true), true);
    env.declare_var("epätosi".to_string(), MK_BOOL(false), true);
    env.declare_var("tyhjä".to_string(), MK_NULL(), true);
    env.declare_var(
        "tulosta".to_string(),
        MK_NATIVE_FN(
            Rc::new(|args, _scope| {
                for arg in args {
                    println!("{:?}", arg);
                }
                MK_NULL()
            })
        ),
        true
    );
    env.declare_var("aika".to_string(), MK_NATIVE_FN(Rc::new(time_function)), true);
    env.declare_var("itseisarvo".to_string(), MK_NATIVE_FN(Rc::new(abs_function)), true);
    env.declare_var("pyöristä".to_string(), MK_NATIVE_FN(Rc::new(round_function)), true);
    env.declare_var("neliöjuuri".to_string(), MK_NATIVE_FN(Rc::new(sqrt_function)), true);
    env.declare_var("syöte".to_string(), MK_NATIVE_FN(Rc::new(input_function)), true);
    env
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub variables: Vec<(String, RuntimeVal)>,
    pub constants: Vec<String>,
}

impl Environment {
    pub fn new(parent_env: Option<Box<Environment>>) -> Self {
        Self {
            parent: parent_env,
            variables: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn declare_var(
        &mut self,
        varname: String,
        value: RuntimeVal,
        constant: bool
    ) -> RuntimeVal {
        if self.variables.iter().any(|(name, _)| name == &varname) {
            panic!("Cannot declare variable {}. As it already is defined.", varname);
        }

        self.variables.push((varname.clone(), value.clone()));
        if constant {
            self.constants.push(varname.clone());
        }
        value
    }

    pub fn assign_var(&mut self, varname: String, value: RuntimeVal) -> RuntimeVal {
        let env = self.resolve(&varname);

        // Cannot assign to constant
        if env.constants.contains(&varname) {
            panic!("Cannot reassign to variable {} as it was declared constant.", varname);
        }

        if let Some((_, val)) = env.variables.iter_mut().find(|(name, _)| name == &varname) {
            *val = value.clone();
        }
        value
    }

    pub fn lookup_var(&mut self, varname: &str) -> RuntimeVal {
        let env = self.resolve(varname);
        env.variables
            .iter()
            .find(|(name, _)| name == &varname)
            .unwrap()
            .1.clone()
    }

    pub fn resolve(&mut self, varname: &str) -> &mut Self {
        if self.variables.iter().any(|(name, _)| name == &varname) {
            return self;
        }

        match &mut self.parent {
            Some(parent) => parent.resolve(varname),
            None => panic!("Cannot resolve '{}' as it does not exist.", varname),
        }
    }
}
