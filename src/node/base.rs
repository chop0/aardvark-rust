use crate::frame::Frame;
use crate::error::AardvarkError;

pub trait AardvarkStatement {
    fn execute(&self, frame: &mut Frame) -> Result<(), AardvarkError>;
}