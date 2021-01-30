use std::fmt::Debug;
use crate::node::base::AardvarkStatement;
use crate::error::AardvarkError;
use std::any::{TypeId, Any};
use crate::frame::Frame;

#[derive(Eq, PartialEq)]
pub enum AardvarkTyped {
    AardvarkI32,
    AardvarkStrLiteral
}

pub trait AardvarkValue where Self: Debug {
    fn get_type(&self) -> AardvarkTyped;
}


pub struct AardvarkFunction {
    statement_nodes: Vec<Box<dyn AardvarkStatement>>,
    argument_types: Vec<AardvarkTyped>,
    args: Vec<Box<dyn AardvarkValue>>,
}

impl AardvarkStatement for AardvarkFunction {
    fn execute(&self, frame: &mut Frame<'eval>) -> Result<(), AardvarkError> {
        for (i, value) in self.args.iter().enumerate() {
            if value.get_type() != *self.argument_types.get(i).expect("Extra argument provided.") {
                return Err(AardvarkError::Other("Type mismatch".parse().unwrap()));
            }
        }

        for x in &self.statement_nodes {
            x.execute(frame)?;
        }

        Ok(())
    }
}
