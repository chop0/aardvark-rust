use std::collections::HashMap;
use std::rc::Rc;
use crate::AardvarkResult;
use crate::error::AardvarkError;
use crate::var::AardvarkValue;

pub struct Frame<'a> {
    pub(crate) parent: Option<&'a Frame<'a>>,
    pub variables: HashMap<&'a str, Rc<dyn AardvarkValue>>
}

impl<'a> Frame<'a> {
    pub fn make_child(&'a self) -> Self {
        Frame {
            parent: Some(&self),
            variables: HashMap::new()
        }
    }

    pub fn access_value(&'a self, name: &'a str) -> AardvarkResult<Rc<dyn AardvarkValue>> {
        match self.variables.get(name) {
            None => {
                match self.parent {
                    None => Err(AardvarkError::Other(format!("Variable {} not defined.", name))),
                    Some(parent) => parent.access_value(name)
                }
            }
            Some(var) => Ok(var.clone())
        }
    }
}
