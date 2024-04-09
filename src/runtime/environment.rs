use crate::runtime::values::*;
use crate::runtime::native_functions::*;
use std::rc::Rc;

pub fn create_global_env() -> Environment {
    let mut env = Environment::new(None);
    env.declare_var("tosi".to_string(), MK_BOOL(true), true);
    env.declare_var("epätosi".to_string(), MK_BOOL(false), true);
    env.declare_var("tyhjä".to_string(), MK_NULL(), true);
    env.declare_var("tulosta".to_string(), MK_NATIVE_FN(Rc::new(print_function)), true);
    env.declare_var("aika".to_string(), MK_NATIVE_FN(Rc::new(time_function)), true);
    env.declare_var("sekunnit".to_string(), MK_NATIVE_FN(Rc::new(millis_to_seconds_function)), true);
    env.declare_var("itseisarvo".to_string(), MK_NATIVE_FN(Rc::new(abs_function)), true);
    env.declare_var("pyöristä".to_string(), MK_NATIVE_FN(Rc::new(round_function)), true);
    env.declare_var("neliöjuuri".to_string(), MK_NATIVE_FN(Rc::new(sqrt_function)), true);
    env.declare_var("syöte".to_string(), MK_NATIVE_FN(Rc::new(input_function)), true);
    env.declare_var("satunnainen".to_string(), MK_NATIVE_FN(Rc::new(random_function)), true);
    env.declare_var("maksimi".to_string(), MK_NATIVE_FN(Rc::new(max_function)), true);
    env.declare_var("minimi".to_string(), MK_NATIVE_FN(Rc::new(min_function)), true);
    env.declare_var("pituus".to_string(), MK_NATIVE_FN(Rc::new(length_function)), true);
    env.declare_var("järjestä".to_string(), MK_NATIVE_FN(Rc::new(sort_function)), true);
    env.declare_var("käänteinen".to_string(), MK_NATIVE_FN(Rc::new(reverse_function)), true);
    env.declare_var("kluku".to_string(), MK_NATIVE_FN(Rc::new(kluku_function)), true);
    env.declare_var("lluku".to_string(), MK_NATIVE_FN(Rc::new(lluku_function)), true);
    env.declare_var("mjono".to_string(), MK_NATIVE_FN(Rc::new(mjono_function)), true);
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
            panic!("Ei voida määrittää muuttujaa {}, sillä se on jo määritelty.", varname);
        }

        self.variables.push((varname.clone(), value.clone()));
        if constant {
            self.constants.push(varname.clone());
        }
        value
    }

    pub fn assign_var(&mut self, varname: &String, value: &RuntimeVal) {
        let env = self.resolve(varname);
    
        // Cannot assign to constant
        if env.constants.contains(varname) {
            panic!("Ei voida määrittää uudelleen muuttujaa {}, koska se luotiin vakioksi.", varname);
        }
    
        if let Some((_, val)) = env.variables.iter_mut().find(|(name, _)| *name == *varname) {
            *val = value.clone();
        }
        value.clone();
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
            None => panic!("Kohdetta '{}' ei voida ratkaista, koska sitä ei ole olemassa.", varname),
        }
    }
}
