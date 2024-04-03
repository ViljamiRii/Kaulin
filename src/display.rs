use std::fmt;
use crate::runtime::values::RuntimeVal;

impl fmt::Display for RuntimeVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeVal::Null(_) => write!(f, "null"),
            RuntimeVal::Bool(b) => write!(f, "{{ \"value\": {}, \"type\": \"bool\" }}", b.value),
            RuntimeVal::Number(n) => write!(f, "{{ \"value\": {}, \"type\": \"number\" }}", n.value),
            RuntimeVal::Object(o) => {
                let mut simple_properties: Vec<_> = o.properties.iter().filter(|(_, v)| matches!(v, RuntimeVal::Number(_) | RuntimeVal::Bool(_))).collect();
                simple_properties.sort_by(|a, b| a.0.cmp(&b.0));
                let mut complex_properties: Vec<_> = o.properties.iter().filter(|(_, v)| matches!(v, RuntimeVal::Object(_))).collect();
                complex_properties.sort_by(|a, b| a.0.cmp(&b.0));
                let properties = simple_properties.into_iter().chain(complex_properties.into_iter())
                    .map(|(k, v)| format!("\t\t\"{}\" => {}", k, v.to_string().replace("\n", "\n\t")))
                    .collect::<Vec<_>>()
                    .join(",\n");
                write!(f, "{{\n\t\"type\": \"object\",\n\t\"properties\": Map {{\n{} \n\t}}\n}}", properties)
            },
        }
    }
}